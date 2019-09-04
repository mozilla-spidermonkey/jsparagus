//! JavaScript lexer.

use crate::errors::{ParseError, Result};
use crate::parser::Parser;
use generated_parser::{TerminalId, Token};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::str::Chars;

// Note: Clone is used when lexing `<!--`, which requires more than one
// character of lookahead.
pub struct Lexer<'a> {
    chars: Chars<'a>,
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

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Lexer<'a> {
        Lexer { chars }
    }

    fn peek(&self) -> Option<char> {
        self.chars.as_str().chars().next()
    }

    pub fn next<'parser>(&mut self, parser: &Parser<'parser>) -> Result<Token<'a>> {
        let mut saw_newline = false;
        self.advance_impl(parser, &mut saw_newline)
            .map(|(value, terminal_id)| Token {
                terminal_id,
                saw_newline,
                value,
            })
    }

    fn accept_digits(&mut self) -> bool {
        let mut at_least_one = false;
        while let Some(next) = self.peek() {
            match next {
                '0'..='9' => {
                    at_least_one = true;
                    self.chars.next();
                }
                _ => break,
            }
        }
        at_least_one
    }

    fn optional_exponent(&mut self) -> Result<()> {
        if let Some('e') | Some('E') = self.peek() {
            self.chars.next().unwrap();
            if let Some('+') | Some('-') = self.peek() {
                self.chars.next().unwrap();
            }
            if !self.accept_digits() {
                // require at least one digit
                return Err(self.unexpected_err());
            }
        }
        Ok(())
    }

    fn unexpected_err(&mut self) -> ParseError {
        if let Some(ch) = self.peek() {
            ParseError::IllegalCharacter(ch)
        } else {
            ParseError::UnexpectedEnd
        }
    }

    fn hex_digit(&mut self) -> Result<u32> {
        match self.chars.next() {
            None => Err(ParseError::UnterminatedString),
            Some(c @ '0'..='9') => Ok(c as u32 - '0' as u32),
            Some(c @ 'a'..='f') => Ok(10 + (c as u32 - 'a' as u32)),
            Some(c @ 'A'..='F') => Ok(10 + (c as u32 - 'A' as u32)),
            Some(other) => Err(ParseError::IllegalCharacter(other)),
        }
    }

    fn escape_sequence(&mut self, text: &mut String) -> Result<()> {
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
                    if self.peek() == Some('\n') {
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
                    if let Some('0'..='9') = self.peek() {
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

    fn string_literal(&mut self, stop: char) -> Result<(Option<Cow<'a, str>>, TerminalId)> {
        let mut builder = AutoCow::new(&self);
        loop {
            match self.chars.next() {
                None | Some('\r') | Some('\n') => {
                    return Err(ParseError::UnterminatedString);
                }

                Some(c @ '"') | Some(c @ '\'') => {
                    if c == stop {
                        return Ok((Some(builder.into_cow(&self)), TerminalId::StringLiteral));
                    } else {
                        builder.push_matching(c);
                    }
                }

                Some('\\') => {
                    let text = builder.get_mut_string(&self);
                    self.escape_sequence(text)?;
                }

                Some(other) => {
                    builder.push_matching(other);
                }
            }
        }
    }

    fn regular_expression_backslash_sequence(&mut self, text: &mut String) -> Result<()> {
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

    fn regular_expression_literal(&mut self) -> Result<(Option<Cow<'a, str>>, TerminalId)> {
        // TODO: First `/` isn't included
        let mut builder = AutoCow::new(&self);
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
                    builder.push_matching('[');
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
                                let text = builder.get_mut_string(&self);
                                self.regular_expression_backslash_sequence(text)?;
                            }
                            Some(ch) => {
                                builder.push_matching(ch);
                            }
                        }
                    }
                    builder.push_matching(']');
                }
                Some('\\') => {
                    let text = builder.get_mut_string(&self);
                    self.regular_expression_backslash_sequence(text)?;
                }
                Some(ch) => {
                    builder.push_matching(ch);
                }
            }
        }
        builder.push_matching('/');
        while let Some(ch) = self.peek() {
            match ch {
                '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    self.chars.next();
                    builder.push_matching(ch);
                }
                _ => break,
            }
        }

        Ok((
            Some(builder.into_cow(&self)),
            TerminalId::RegularExpressionLiteral,
        ))
    }

    fn conditional_keyword(
        &mut self,
        parser: &Parser,
        keyword_id: TerminalId,
        text: Cow<'a, str>,
    ) -> Result<(Option<Cow<'a, str>>, TerminalId)> {
        if parser.can_accept_terminal(keyword_id) {
            Ok((None, keyword_id))
        } else {
            Ok((Some(text), TerminalId::Identifier))
        }
    }

    fn identifier<'parser>(
        &mut self,
        parser: &Parser<'parser>,
        mut builder: AutoCow<'a>,
    ) -> Result<(Option<Cow<'a, str>>, TerminalId)> {
        while let Some(ch) = self.peek() {
            if !is_identifier_part(ch) {
                break;
            }
            self.chars.next();
            builder.push_matching(ch);
        }
        let text = builder.into_cow(&self);
        if parser.can_accept_terminal(TerminalId::IdentifierName) {
            return Ok((Some(text), TerminalId::IdentifierName));
        }

        match &text as &str {
            "as" => self.conditional_keyword(parser, TerminalId::As, text),
            "async" => self.conditional_keyword(parser, TerminalId::Async, text),
            "await" => self.conditional_keyword(parser, TerminalId::Await, text),
            "break" => Ok((None, TerminalId::Break)),
            "case" => Ok((None, TerminalId::Case)),
            "catch" => Ok((None, TerminalId::Catch)),
            "class" => Ok((None, TerminalId::Class)),
            "const" => Ok((None, TerminalId::Const)),
            "continue" => Ok((None, TerminalId::Continue)),
            "debugger" => Ok((None, TerminalId::Debugger)),
            "default" => Ok((None, TerminalId::Default)),
            "delete" => Ok((None, TerminalId::Delete)),
            "do" => Ok((None, TerminalId::Do)),
            "else" => Ok((None, TerminalId::Else)),
            "export" => Ok((None, TerminalId::Export)),
            "extends" => Ok((None, TerminalId::Extends)),
            "finally" => Ok((None, TerminalId::Finally)),
            "for" => Ok((None, TerminalId::For)),
            "from" => self.conditional_keyword(parser, TerminalId::From, text),
            "function" => Ok((None, TerminalId::Function)),
            "get" => self.conditional_keyword(parser, TerminalId::Get, text),
            "if" => Ok((None, TerminalId::If)),
            "import" => Ok((None, TerminalId::Import)),
            "in" => Ok((None, TerminalId::In)),
            "instanceof" => Ok((None, TerminalId::Instanceof)),
            "let" => self.conditional_keyword(parser, TerminalId::Let, text),
            "new" => Ok((None, TerminalId::New)),
            "of" => self.conditional_keyword(parser, TerminalId::Of, text),
            "return" => Ok((None, TerminalId::Return)),
            "set" => self.conditional_keyword(parser, TerminalId::Set, text),
            "static" => Ok((None, TerminalId::Static)),
            "super" => Ok((None, TerminalId::Super)),
            "switch" => Ok((None, TerminalId::Switch)),
            "target" => self.conditional_keyword(parser, TerminalId::Target, text),
            "this" => Ok((None, TerminalId::This)),
            "throw" => Ok((None, TerminalId::Throw)),
            "try" => Ok((None, TerminalId::Try)),
            "typeof" => Ok((None, TerminalId::Typeof)),
            "var" => Ok((None, TerminalId::Var)),
            "void" => Ok((None, TerminalId::Void)),
            "while" => Ok((None, TerminalId::While)),
            "with" => Ok((None, TerminalId::With)),
            "yield" => self.conditional_keyword(parser, TerminalId::Yield, text),
            "null" => Ok((None, TerminalId::NullLiteral)),
            "true" => Ok((Some(text), TerminalId::BooleanLiteral)),
            "false" => Ok((Some(text), TerminalId::BooleanLiteral)),
            _ => Ok((Some(text), TerminalId::Identifier)),
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

    fn advance_impl<'parser>(
        &mut self,
        parser: &Parser<'parser>,
        saw_newline: &mut bool,
    ) -> Result<(Option<Cow<'a, str>>, TerminalId)> {
        let mut builder = AutoCow::new(&self);
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
                    builder = AutoCow::new(&self);
                    continue;
                }

                // LineTerminator
                '\u{a}' | '\u{d}' | '\u{2028}' | '\u{2029}' => {
                    *saw_newline = true;
                    builder = AutoCow::new(&self);
                    continue;
                }

                // Numbers
                '0'..='9' => {
                    match self.peek() {
                        // DecimalLiteral
                        Some('0'..='9') if c != '0' => {
                            self.accept_digits();
                            if let Some('.') = self.peek() {
                                self.chars.next();
                                self.accept_digits();
                            }
                            self.optional_exponent()?;
                        }
                        Some('.') | Some('e') | Some('E') => {
                            if let Some('.') = self.peek() {
                                self.chars.next();
                                self.accept_digits();
                            }
                            self.optional_exponent()?;
                        }
                        // BinaryIntegerLiteral
                        Some('b') | Some('B') if c == '0' => {
                            self.chars.next().unwrap();
                            let mut at_least_one = false;
                            while let Some(next) = self.peek() {
                                match next {
                                    '0' | '1' => {
                                        at_least_one = true;
                                        self.chars.next();
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
                            self.chars.next().unwrap();
                            let mut at_least_one = false;
                            while let Some(next) = self.peek() {
                                match next {
                                    '0'..='7' => {
                                        at_least_one = true;
                                        self.chars.next();
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
                            self.chars.next().unwrap();
                            let mut at_least_one = false;
                            while let Some(next) = self.peek() {
                                match next {
                                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                                        at_least_one = true;
                                        self.chars.next();
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
                                self.accept_digits();
                            }
                        }
                    }

                    // The SourceCharacter immediately following a
                    // NumericLiteral must not be an IdentifierStart or
                    // DecimalDigit.
                    if let Some(ch) = self.peek() {
                        if is_identifier_start(ch) || ch.is_digit(10) {
                            return Err(ParseError::IllegalCharacter(ch));
                        }
                    }

                    // Don't have to push_matching since push_different is never called.
                    return Ok((Some(builder.into_cow(&self)), TerminalId::NumericLiteral));
                }

                // Strings
                '"' | '\'' => {
                    return self.string_literal(c);
                }

                '`' => {
                    let mut builder = AutoCow::new(&self);
                    // include quotes?
                    while let Some(ch) = self.chars.next() {
                        if ch == '$' && self.peek() == Some('{') {
                            self.chars.next();
                            return Ok((None, TerminalId::TemplateHead));
                        }
                        if ch == '`' {
                            return Ok((None, TerminalId::NoSubstitutionTemplate));
                        }
                        // TODO: Do escapes exist? Should we support them?
                        builder.push_matching(ch);
                    }
                    return Ok((Some(builder.into_cow(&self)), TerminalId::StringLiteral));
                }

                '!' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((None, TerminalId::ExclamationMarkEqualsSignEqualsSign));
                            }
                            _ => return Ok((None, TerminalId::ExclamationMarkEqualsSign)),
                        }
                    }
                    _ => return Ok((None, TerminalId::ExclamationMark)),
                },

                '%' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::PercentSignEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::PercentSign)),
                },

                '&' => match self.peek() {
                    Some('&') => {
                        self.chars.next();
                        return Ok((None, TerminalId::AmpersandAmpersand));
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::AmpersandEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::Ampersand)),
                },

                '*' => match self.peek() {
                    Some('*') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((None, TerminalId::AsteriskAsteriskEqualsSign));
                            }
                            _ => return Ok((None, TerminalId::AsteriskAsterisk)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::AsteriskEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::Asterisk)),
                },

                '+' => match self.peek() {
                    Some('+') => {
                        self.chars.next();
                        return Ok((None, TerminalId::PlusSignPlusSign));
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::PlusSignEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::PlusSign)),
                },

                '-' => match self.peek() {
                    Some('-') => {
                        self.chars.next();
                        match self.peek() {
                            Some('>') => {
                                // B.1.3 SingleLineHTMLCloseComment
                                // TODO: Limit this to Script (not Module) and
                                // at the start of a line.
                                self.skip_single_line_comment();
                                builder = AutoCow::new(&self);
                                continue;
                            }
                            _ => return Ok((None, TerminalId::HyphenMinusHyphenMinus)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::HyphenMinusEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::HyphenMinus)),
                },

                '.' => match self.peek() {
                    Some('.') => {
                        self.chars.next();
                        match self.peek() {
                            Some('.') => {
                                self.chars.next();
                                return Ok((None, TerminalId::FullStopFullStopFullStop));
                            }
                            _ => return Err(ParseError::IllegalCharacter('.')),
                        }
                    }
                    Some('0'..='9') => {
                        self.accept_digits();
                        self.optional_exponent()?;

                        // The SourceCharacter immediately following a
                        // NumericLiteral must not be an IdentifierStart or
                        // DecimalDigit.
                        if let Some(ch) = self.peek() {
                            if is_identifier_start(ch) || ch.is_digit(10) {
                                return Err(ParseError::IllegalCharacter(ch));
                            }
                        }

                        // Don't have to push_matching since push_different is never called.
                        return Ok((Some(builder.into_cow(&self)), TerminalId::NumericLiteral));
                    }
                    _ => return Ok((None, TerminalId::FullStop)),
                },

                '/' => match self.peek() {
                    // Comments take priority over regexps
                    // Single-line comment
                    Some('/') => {
                        self.chars.next();
                        self.skip_single_line_comment();
                        builder = AutoCow::new(&self);
                        continue;
                    }
                    // Multiline comment
                    Some('*') => {
                        self.chars.next();
                        while let Some(ch) = self.chars.next() {
                            // TODO: ASI
                            if ch == '*' && self.peek() == Some('/') {
                                self.chars.next();
                                break;
                            }
                        }
                        builder = AutoCow::new(&self);
                        continue;
                    }
                    _ => {
                        if parser.can_accept_terminal(TerminalId::RegularExpressionLiteral) {
                            return self.regular_expression_literal();
                        }
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((None, TerminalId::SolidusEqualsSign));
                            }
                            _ => return Ok((None, TerminalId::Solidus)),
                        }
                    }
                },

                '<' => match self.peek() {
                    Some('<') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((None, TerminalId::LessThanSignLessThanSignEqualsSign));
                            }
                            _ => return Ok((None, TerminalId::LessThanSignLessThanSign)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::LessThanSignEqualsSign));
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
                            builder = AutoCow::new(&self);
                            continue;
                        }
                        return Ok((None, TerminalId::LessThanSign));
                    }
                    _ => return Ok((None, TerminalId::LessThanSign)),
                },

                '=' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((None, TerminalId::EqualsSignEqualsSignEqualsSign));
                            }
                            _ => return Ok((None, TerminalId::EqualsSignEqualsSign)),
                        }
                    }
                    Some('>') => {
                        self.chars.next();
                        return Ok((None, TerminalId::Arrow));
                    }
                    _ => return Ok((None, TerminalId::EqualsSign)),
                },

                '>' => match self.peek() {
                    Some('>') => {
                        self.chars.next();
                        match self.peek() {
                            Some('>') => {
                                self.chars.next();
                                match self.peek() {
                                    Some('=') => {
                                        self.chars.next();
                                        return Ok((None, TerminalId::GreaterThanSignGreaterThanSignGreaterThanSignEqualsSign));
                                    }
                                    _ => return Ok((None, TerminalId::GreaterThanSignGreaterThanSignGreaterThanSign)),
                                }
                            }
                            Some('=') => {
                                self.chars.next();
                                return Ok((None, TerminalId::GreaterThanSignGreaterThanSignEqualsSign));
                            }
                            _ => return Ok((None, TerminalId::GreaterThanSignGreaterThanSign)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::GreaterThanSignEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::GreaterThanSign)),
                },

                '^' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::CircumflexAccentEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::CircumflexAccent)),
                },

                '|' => match self.peek() {
                    Some('|') => {
                        self.chars.next();
                        return Ok((None, TerminalId::VerticalLineVerticalLine));
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((None, TerminalId::VerticalLineEqualsSign));
                    }
                    _ => return Ok((None, TerminalId::VerticalLine)),
                },

                '(' => return Ok((None, TerminalId::LeftParenthesis)),
                ')' => return Ok((None, TerminalId::RightParenthesis)),
                ',' => return Ok((None, TerminalId::Comma)),
                ':' => return Ok((None, TerminalId::Colon)),
                ';' => return Ok((None, TerminalId::Semicolon)),
                '?' => return Ok((None, TerminalId::QuestionMark)),
                '[' => return Ok((None, TerminalId::LeftSquareBracket)),
                ']' => return Ok((None, TerminalId::RightSquareBracket)),
                '{' => return Ok((None, TerminalId::LeftCurlyBracket)),
                '}' => return Ok((None, TerminalId::RightCurlyBracket)),
                '~' => return Ok((None, TerminalId::Tilde)),

                // Idents
                '$' | '_' | 'a'..='z' | 'A'..='Z' => {
                    builder.push_matching(c);
                    return self.identifier(parser, builder);
                }

                other => {
                    if is_identifier_start(other) {
                        builder.push_matching(other);
                        return self.identifier(parser, builder);
                    }
                    return Err(ParseError::IllegalCharacter(other));
                }
            }
        }
        Ok((None, TerminalId::End))
    }
}

struct AutoCow<'a> {
    start: &'a str,
    value: Option<String>,
}

impl<'a> AutoCow<'a> {
    fn new(lexer: &Lexer<'a>) -> Self {
        AutoCow {
            start: lexer.chars.as_str(),
            value: None,
        }
    }

    // Push a char that matches lexer.chars.next()
    fn push_matching(&mut self, c: char) {
        if let Some(text) = &mut self.value {
            text.push(c);
        }
    }

    // Push a char that does not match lexer.chars.next() (for example, string escapes)
    fn push_different(&mut self, lexer: &Lexer<'a>, c: char) {
        self.get_mut_string(lexer).push(c);
    }

    // Force allocation of a String and return the reference to it
    fn get_mut_string<'b, 'c>(&'b mut self, lexer: &'c Lexer<'a>) -> &'b mut String {
        if self.value.is_none() {
            self.value =
                Some(self.start[..self.start.len() - lexer.chars.as_str().len()].to_string());
        }
        self.value.as_mut().unwrap()
    }

    fn into_cow(self, lexer: &Lexer<'a>) -> Cow<'a, str> {
        match self.value {
            Some(owned) => Cow::Owned(owned),
            None => Cow::Borrowed(&self.start[..self.start.len() - lexer.chars.as_str().len()]),
        }
    }
}
