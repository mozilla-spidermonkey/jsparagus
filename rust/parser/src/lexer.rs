//! JavaScript lexer.

use crate::errors::{ParseError, Result};
use crate::parser::Parser;
use generated_parser::{TerminalId, Token};
use std::convert::TryFrom;

// Note: Clone is used when lexing `<!--`, which requires more than one
// character of lookahead.
pub struct Lexer<Iter: Iterator<Item = char> + Clone> {
    chars: std::iter::Peekable<Iter>,
}

fn is_identifier_start(c: char) -> bool {
    // TODO - Adjust this to match the Unicode ID_Start property (#23).
    c == '$' || c == '_' || c.is_alphabetic()
}

fn is_identifier_part(c: char) -> bool {
    if (c as u32) < 128 {
        match c {
            '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        }
    } else {
        // TODO - Adjust this to match the Unicode ID_Continue property (#23).
        c.is_alphabetic()
    }
}

impl<'a, Iter: Iterator<Item = char> + Clone> Lexer<Iter> {
    pub fn new(chars: Iter) -> Lexer<Iter> {
        Lexer {
            chars: chars.peekable(),
        }
    }

    pub fn next(&mut self, parser: &Parser<'a>) -> Result<'a, Token<'a>> {
        let mut text = String::new();
        let mut saw_newline = false;
        self.advance_impl(parser, &mut text, &mut saw_newline)
            .map(|terminal_id| Token {
                terminal_id,
                saw_newline,
                value: if !text.is_empty() || terminal_id == TerminalId::StringLiteral {
                    Some(text.into())
                } else {
                    None
                },
            })
    }

    fn accept_digits(&mut self, text: &mut String) -> bool {
        let mut at_least_one = false;
        while let Some(&next) = self.chars.peek() {
            match next {
                '0'..='9' => {
                    at_least_one = true;
                    self.chars.next();
                    text.push(next);
                }
                _ => break,
            }
        }
        at_least_one
    }

    fn optional_exponent(&mut self, text: &mut String) -> Result<'a, ()> {
        if let Some('e') | Some('E') = self.chars.peek() {
            text.push(self.chars.next().unwrap());
            if let Some('+') | Some('-') = self.chars.peek() {
                text.push(self.chars.next().unwrap());
            }
            if !self.accept_digits(text) {
                // require at least one digit
                return Err(self.unexpected_err());
            }
        }
        Ok(())
    }

    fn unexpected_err(&mut self) -> ParseError<'static> {
        if let Some(&ch) = self.chars.peek() {
            ParseError::IllegalCharacter(ch)
        } else {
            ParseError::UnexpectedEnd
        }
    }

    fn hex_digit(&mut self) -> Result<'a, u32> {
        match self.chars.next() {
            None => Err(ParseError::UnterminatedString),
            Some(c @ '0'..='9') => Ok(c as u32 - '0' as u32),
            Some(c @ 'a'..='f') => Ok(10 + (c as u32 - 'a' as u32)),
            Some(c @ 'A'..='F') => Ok(10 + (c as u32 - 'A' as u32)),
            Some(other) => Err(ParseError::IllegalCharacter(other)),
        }
    }

    fn escape_sequence(&mut self, text: &mut String) -> Result<'a, ()> {
        match self.chars.next() {
            None => {
                return Err(ParseError::UnterminatedString);
            }
            Some(c) => match c {
                '\n' | '\u{2028}' | '\u{2029}' => {
                    // LineContinuation. Ignore it.
                }
                '\r' => {
                    // LineContinuation. Check for the sequence \r\n; otherwise
                    // ignore it.
                    if self.chars.peek() == Some(&'\n') {
                        self.chars.next();
                    }
                }

                '\'' | '"' | '\\' => {
                    text.push(c);
                }
                'b' => {
                    text.push('\u{8}');
                }
                'f' => {
                    text.push('\u{c}');
                }
                'n' => {
                    text.push('\n');
                }
                'r' => {
                    text.push('\r');
                }
                't' => {
                    text.push('\t');
                }
                'v' => {
                    text.push('\u{b}');
                }
                'x' => {
                    let mut value = self.hex_digit()?;
                    value = (value << 4) | self.hex_digit()?;
                    match char::try_from(value) {
                        Err(_) => {
                            return Err(ParseError::InvalidEscapeSequence);
                        }
                        Ok(c) => {
                            text.push(c);
                        }
                    }
                }
                'u' => {
                    let mut value = 0;
                    for _ in 0..4 {
                        value = (value << 4) | self.hex_digit()?;
                    }
                    match char::try_from(value) {
                        Err(_) => {
                            return Err(ParseError::InvalidEscapeSequence);
                        }
                        Ok(c) => {
                            text.push(c);
                        }
                    }
                }
                '0' => {
                    if let Some('0'..='9') = self.chars.peek() {
                        return Err(ParseError::InvalidEscapeSequence);
                    }
                    text.push('\0');
                }
                '1'..='9' => {
                    return Err(ParseError::InvalidEscapeSequence);
                }
                other => {
                    text.push(other);
                }
            },
        }
        Ok(())
    }

    fn string_literal(&mut self, stop: char, text: &mut String) -> Result<'a, TerminalId> {
        loop {
            match self.chars.next() {
                None | Some('\r') | Some('\n') => {
                    return Err(ParseError::UnterminatedString);
                }

                Some(c @ '"') | Some(c @ '\'') => {
                    if c == stop {
                        return Ok(TerminalId::StringLiteral);
                    } else {
                        text.push(c);
                    }
                }

                Some('\\') => {
                    self.escape_sequence(text)?;
                }

                Some(other) => {
                    text.push(other);
                }
            }
        }
    }

    fn regular_expression_backslash_sequence(&mut self, text: &mut String) -> Result<'a, ()> {
        text.push('\\');
        match self.chars.next() {
            None | Some('\r') | Some('\n') | Some('\u{2028}') | Some('\u{2029}') => {
                Err(ParseError::UnterminatedRegExp)
            }
            Some(c) => {
                text.push(c);
                Ok(())
            }
        }
    }

    fn regular_expression_literal(&mut self, text: &mut String) -> Result<'a, TerminalId> {
        text.push('/');
        loop {
            match self.chars.next() {
                None | Some('\r') | Some('\n') | Some('\u{2028}') | Some('\u{2029}') => {
                    return Err(ParseError::UnterminatedRegExp);
                }
                Some('/') => {
                    break;
                }
                Some('[') => {
                    // RegularExpressionClass.
                    text.push('[');
                    loop {
                        match self.chars.next() {
                            None | Some('\r') | Some('\n') | Some('\u{2028}')
                            | Some('\u{2029}') => {
                                return Err(ParseError::UnterminatedRegExp);
                            }
                            Some(']') => {
                                break;
                            }
                            Some('\\') => {
                                self.regular_expression_backslash_sequence(text)?;
                            }
                            Some(ch) => {
                                text.push(ch);
                            }
                        }
                    }
                    text.push(']');
                }
                Some('\\') => {
                    self.regular_expression_backslash_sequence(text)?;
                }
                Some(ch) => {
                    text.push(ch);
                }
            }
        }
        text.push('/');
        while let Some(&ch) = self.chars.peek() {
            match ch {
                '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    self.chars.next();
                    text.push(ch);
                }
                _ => break,
            }
        }

        Ok(TerminalId::RegularExpressionLiteral)
    }

    fn conditional_keyword(
        &mut self,
        parser: &Parser,
        text: &mut String,
        name: String,
        keyword_id: TerminalId,
    ) -> Result<TerminalId> {
        if parser.can_accept_terminal(keyword_id) {
            Ok(keyword_id)
        } else {
            *text = name;
            Ok(TerminalId::Identifier)
        }
    }

    fn identifier(&mut self, parser: &Parser, c: char, text: &mut String) -> Result<TerminalId> {
        let mut var = String::new();
        var.push(c);
        while let Some(&ch) = self.chars.peek() {
            if !is_identifier_part(ch) {
                break;
            }
            self.chars.next();
            var.push(ch);
        }
        if parser.can_accept_terminal(TerminalId::IdentifierName) {
            *text = var;
            return Ok(TerminalId::IdentifierName);
        }

        match &var as &str {
            "as" => return self.conditional_keyword(parser, text, var, TerminalId::As),
            "async" => return self.conditional_keyword(parser, text, var, TerminalId::Async),
            "await" => return self.conditional_keyword(parser, text, var, TerminalId::Await),
            "break" => return Ok(TerminalId::Break),
            "case" => return Ok(TerminalId::Case),
            "catch" => return Ok(TerminalId::Catch),
            "class" => return Ok(TerminalId::Class),
            "const" => return Ok(TerminalId::Const),
            "continue" => return Ok(TerminalId::Continue),
            "debugger" => return Ok(TerminalId::Debugger),
            "default" => return Ok(TerminalId::Default),
            "delete" => return Ok(TerminalId::Delete),
            "do" => return Ok(TerminalId::Do),
            "else" => return Ok(TerminalId::Else),
            "export" => return Ok(TerminalId::Export),
            "extends" => return Ok(TerminalId::Extends),
            "finally" => return Ok(TerminalId::Finally),
            "for" => return Ok(TerminalId::For),
            "from" => return self.conditional_keyword(parser, text, var, TerminalId::From),
            "function" => return Ok(TerminalId::Function),
            "get" => return self.conditional_keyword(parser, text, var, TerminalId::Get),
            "if" => return Ok(TerminalId::If),
            "import" => return Ok(TerminalId::Import),
            "in" => return Ok(TerminalId::In),
            "instanceof" => return Ok(TerminalId::Instanceof),
            "let" => return self.conditional_keyword(parser, text, var, TerminalId::Let),
            "new" => return Ok(TerminalId::New),
            "of" => return self.conditional_keyword(parser, text, var, TerminalId::Of),
            "return" => return Ok(TerminalId::Return),
            "set" => return self.conditional_keyword(parser, text, var, TerminalId::Set),
            "static" => return Ok(TerminalId::Static),
            "super" => return Ok(TerminalId::Super),
            "switch" => return Ok(TerminalId::Switch),
            "target" => return self.conditional_keyword(parser, text, var, TerminalId::Target),
            "this" => return Ok(TerminalId::This),
            "throw" => return Ok(TerminalId::Throw),
            "try" => return Ok(TerminalId::Try),
            "typeof" => return Ok(TerminalId::Typeof),
            "var" => return Ok(TerminalId::Var),
            "void" => return Ok(TerminalId::Void),
            "while" => return Ok(TerminalId::While),
            "with" => return Ok(TerminalId::With),
            "yield" => return self.conditional_keyword(parser, text, var, TerminalId::Yield),
            "null" => return Ok(TerminalId::NullLiteral),
            "true" => {
                *text = var;
                return Ok(TerminalId::BooleanLiteral);
            }
            "false" => {
                *text = var;
                return Ok(TerminalId::BooleanLiteral);
            }
            _ => {
                *text = var;
                return Ok(TerminalId::Identifier);
            }
        }
    }

    fn skip_single_line_comment(&mut self) {
        while let Some(ch) = self.chars.next() {
            match ch {
                '\u{a}' | '\u{d}' | '\u{2028}' | '\u{2029}' => break,
                _ => continue,
            }
        }
    }

    fn advance_impl(
        &mut self,
        parser: &Parser,
        text: &mut String,
        saw_newline: &mut bool,
    ) -> Result<'a, TerminalId> {
        while let Some(c) = self.chars.next() {
            match c {
                // WhiteSpace.
                '\u{9}' | // <TAB>
                '\u{b}' | // <VT>
                '\u{c}' | // <FF>
                '\u{20}' | // <SP>, the space character
                '\u{a0}' | // <NBSP>
                '\u{1680}' | // Ogham space mark (in <USP>)
                '\u{2000}' ..= '\u{200a}' | // typesetting spaces (in <USP>)
                '\u{202f}' | // Narrow no-break space (in <USP>)
                '\u{205f}' | // Medium mathematical space (in <USP>)
                '\u{3000}' | // Ideographic space (in <USP>)
                '\u{feff}' // <ZWNBSP>
                    => {
                    // TODO - The spec uses <USP> to stand for any character
                    // with category "Space_Separator" (Zs). New Unicode
                    // standards may add characters to this set. This should therefore be
                    // implemented using the Unicode database somehow.
                    continue;
                }

                // LineTerminator
                '\u{a}' | '\u{d}' | '\u{2028}' | '\u{2029}' => {
                    *saw_newline = true;
                    continue;
                }

                // Numbers
                '0'..='9' => {
                    text.push(c);
                    match self.chars.peek() {
                        // DecimalLiteral
                        Some('0'..='9') if c != '0' => {
                            self.accept_digits(text);
                            if let Some('.') = self.chars.peek() {
                                self.chars.next();
                                text.push('.');
                                self.accept_digits(text);
                            }
                            self.optional_exponent(text)?;
                        }
                        Some('.') | Some('e') | Some('E') => {
                            if let Some('.') = self.chars.peek() {
                                self.chars.next();
                                text.push('.');
                                self.accept_digits(text);
                            }
                            self.optional_exponent(text)?;
                        }
                        // BinaryIntegerLiteral
                        Some('b') | Some('B') if c == '0' => {
                            text.push(self.chars.next().unwrap());
                            let mut at_least_one = false;
                            while let Some(&next) = self.chars.peek() {
                                match next {
                                    '0' | '1' => {
                                        at_least_one = true;
                                        self.chars.next();
                                        text.push(next);
                                    }
                                    _ => break,
                                }
                            }
                            if !at_least_one {
                                return Err(self.unexpected_err());
                            }
                        }
                        // OctalIntegerLiteral
                        Some('o') | Some('O') if c == '0' => {
                            text.push(self.chars.next().unwrap());
                            let mut at_least_one = false;
                            while let Some(&next) = self.chars.peek() {
                                match next {
                                    '0'..='7' => {
                                        at_least_one = true;
                                        self.chars.next();
                                        text.push(next);
                                    }
                                    _ => break,
                                }
                            }
                            if !at_least_one {
                                return Err(self.unexpected_err());
                            }
                        }
                        // HexIntegerLiteral
                        Some('x') | Some('X') if c == '0' => {
                            text.push(self.chars.next().unwrap());
                            let mut at_least_one = false;
                            while let Some(&next) = self.chars.peek() {
                                match next {
                                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                                        at_least_one = true;
                                        self.chars.next();
                                        text.push(next);
                                    }
                                    _ => break,
                                }
                            }
                            if !at_least_one {
                                return Err(self.unexpected_err());
                            }
                        }
                        // `0`
                        _ => {
                            // TODO: implement `strict_mode` check
                            let strict_mode = true;
                            if !strict_mode {
                                // TODO: Distinguish between Octal and NonOctal
                                self.accept_digits(text);
                            }
                        }
                    }

                    // The SourceCharacter immediately following a
                    // NumericLiteral must not be an IdentifierStart or
                    // DecimalDigit.
                    if let Some(&ch) = self.chars.peek() {
                        if is_identifier_start(ch) || ch.is_digit(10) {
                            return Err(ParseError::IllegalCharacter(ch));
                        }
                    }
                    return Ok(TerminalId::NumericLiteral);
                }

                // Strings
                '"' | '\'' => {
                    return self.string_literal(c, text);
                }

                '`' => {
                    // include quotes?
                    while let Some(ch) = self.chars.next() {
                        if ch == '$' && self.chars.peek() == Some(&'{') {
                            self.chars.next();
                            return Ok(TerminalId::TemplateHead);
                        }
                        if ch == '`' {
                            return Ok(TerminalId::NoSubstitutionTemplate);
                        }
                        text.push(ch)
                    }
                    return Ok(TerminalId::StringLiteral);
                }

                '!' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok(TerminalId::ExclamationMarkEqualsSignEqualsSign);
                            }
                            _ => return Ok(TerminalId::ExclamationMarkEqualsSign),
                        }
                    }
                    _ => return Ok(TerminalId::ExclamationMark),
                },

                '%' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::PercentSignEqualsSign);
                    }
                    _ => return Ok(TerminalId::PercentSign),
                },

                '&' => match self.chars.peek() {
                    Some('&') => {
                        self.chars.next();
                        return Ok(TerminalId::AmpersandAmpersand);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::AmpersandEqualsSign);
                    }
                    _ => return Ok(TerminalId::Ampersand),
                },

                '*' => match self.chars.peek() {
                    Some('*') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok(TerminalId::AsteriskAsteriskEqualsSign);
                            }
                            _ => return Ok(TerminalId::AsteriskAsterisk),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::AsteriskEqualsSign);
                    }
                    _ => return Ok(TerminalId::Asterisk),
                },

                '+' => match self.chars.peek() {
                    Some('+') => {
                        self.chars.next();
                        return Ok(TerminalId::PlusSignPlusSign);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::PlusSignEqualsSign);
                    }
                    _ => return Ok(TerminalId::PlusSign),
                },

                '-' => match self.chars.peek() {
                    Some('-') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('>') => {
                                // B.1.3 SingleLineHTMLCloseComment
                                // TODO: Limit this to Script (not Module) and
                                // at the start of a line.
                                self.skip_single_line_comment();
                                continue;
                            }
                            _ => return Ok(TerminalId::HyphenMinusHyphenMinus),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::HyphenMinusEqualsSign);
                    }
                    _ => return Ok(TerminalId::HyphenMinus),
                },

                '.' => match self.chars.peek() {
                    Some('.') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('.') => {
                                self.chars.next();
                                return Ok(TerminalId::FullStopFullStopFullStop);
                            }
                            _ => return Err(ParseError::IllegalCharacter('.')),
                        }
                    }
                    Some('0'..='9') => {
                        text.push('.');
                        self.accept_digits(text);
                        self.optional_exponent(text)?;

                        // The SourceCharacter immediately following a
                        // NumericLiteral must not be an IdentifierStart or
                        // DecimalDigit.
                        if let Some(&ch) = self.chars.peek() {
                            if is_identifier_start(ch) || ch.is_digit(10) {
                                return Err(ParseError::IllegalCharacter(ch));
                            }
                        }
                        return Ok(TerminalId::NumericLiteral);
                    }
                    _ => return Ok(TerminalId::FullStop),
                },

                '/' => match self.chars.peek() {
                    // Comments take priority over regexps
                    // Single-line comment
                    Some('/') => {
                        self.chars.next();
                        self.skip_single_line_comment();
                        continue;
                    }
                    // Multiline comment
                    Some('*') => {
                        self.chars.next();
                        while let Some(ch) = self.chars.next() {
                            // TODO: ASI
                            if ch == '*' && self.chars.peek() == Some(&'/') {
                                self.chars.next();
                                break;
                            }
                        }
                        continue;
                    }
                    _ => {
                        if parser.can_accept_terminal(TerminalId::RegularExpressionLiteral) {
                            return self.regular_expression_literal(text);
                        }
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok(TerminalId::SolidusEqualsSign);
                            }
                            _ => return Ok(TerminalId::Solidus),
                        }
                    }
                },

                '<' => match self.chars.peek() {
                    Some('<') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok(TerminalId::LessThanSignLessThanSignEqualsSign);
                            }
                            _ => return Ok(TerminalId::LessThanSignLessThanSign),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::LessThanSignEqualsSign);
                    }
                    Some('!') => {
                        // Check for B.1.3 SingleLineHTMLOpenComment. This
                        // requires three characters of lookahead, because
                        // `x<!--` has a comment but `x<!-y` does not.
                        //
                        // TODO: Limit this to Script (not Module).
                        self.chars.next();
                        let mut lookahead_iter = self.chars.clone();
                        if lookahead_iter.next() == Some('-') && lookahead_iter.next() == Some('-')
                        {
                            self.skip_single_line_comment();
                            continue;
                        }
                        return Ok(TerminalId::LessThanSign);
                    }
                    _ => return Ok(TerminalId::LessThanSign),
                },

                '=' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok(TerminalId::EqualsSignEqualsSignEqualsSign);
                            }
                            _ => return Ok(TerminalId::EqualsSignEqualsSign),
                        }
                    }
                    Some('>') => {
                        self.chars.next();
                        return Ok(TerminalId::Arrow);
                    }
                    _ => return Ok(TerminalId::EqualsSign),
                },

                '>' => match self.chars.peek() {
                    Some('>') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('>') => {
                                self.chars.next();
                                match self.chars.peek() {
                                    Some('=') => {
                                        self.chars.next();
                                        return Ok(TerminalId::GreaterThanSignGreaterThanSignGreaterThanSignEqualsSign);
                                    }
                                    _ => return Ok(
                                        TerminalId::GreaterThanSignGreaterThanSignGreaterThanSign,
                                    ),
                                }
                            }
                            Some('=') => {
                                self.chars.next();
                                return Ok(TerminalId::GreaterThanSignGreaterThanSignEqualsSign);
                            }
                            _ => return Ok(TerminalId::GreaterThanSignGreaterThanSign),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::GreaterThanSignEqualsSign);
                    }
                    _ => return Ok(TerminalId::GreaterThanSign),
                },

                '^' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::CircumflexAccentEqualsSign);
                    }
                    _ => return Ok(TerminalId::CircumflexAccent),
                },

                '|' => match self.chars.peek() {
                    Some('|') => {
                        self.chars.next();
                        return Ok(TerminalId::VerticalLineVerticalLine);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok(TerminalId::VerticalLineEqualsSign);
                    }
                    _ => return Ok(TerminalId::VerticalLine),
                },

                '(' => return Ok(TerminalId::LeftParenthesis),
                ')' => return Ok(TerminalId::RightParenthesis),
                ',' => return Ok(TerminalId::Comma),
                ':' => return Ok(TerminalId::Colon),
                ';' => return Ok(TerminalId::Semicolon),
                '?' => return Ok(TerminalId::QuestionMark),
                '[' => return Ok(TerminalId::LeftSquareBracket),
                ']' => return Ok(TerminalId::RightSquareBracket),
                '{' => return Ok(TerminalId::LeftCurlyBracket),
                '}' => return Ok(TerminalId::RightCurlyBracket),
                '~' => return Ok(TerminalId::Tilde),

                // Idents
                '$' | '_' | 'a'..='z' | 'A'..='Z' => {
                    return self.identifier(parser, c, text);
                }

                other => {
                    if is_identifier_start(other) {
                        return self.identifier(parser, other, text);
                    }
                    return Err(ParseError::IllegalCharacter(other));
                }
            }
        }
        Ok(TerminalId::End)
    }
}
