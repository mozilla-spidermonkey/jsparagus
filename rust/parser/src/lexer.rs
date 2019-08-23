//! JavaScript lexer.

use crate::errors::{ParseError, Result};
use crate::parser::Parser;
use generated_parser::{TerminalId, Token};

pub struct Lexer<Iter: Iterator<Item = char>> {
    chars: std::iter::Peekable<Iter>,
}

impl<Iter: Iterator<Item = char>> Lexer<Iter> {
    pub fn new(chars: Iter) -> Lexer<Iter> {
        Lexer {
            chars: chars.peekable(),
        }
    }

    pub fn next(&mut self, parser: &Parser) -> Result<Token> {
        let mut text = String::new();
        let mut saw_newline = false;
        self.advance_impl(parser, &mut text, &mut saw_newline)
            .map(|terminal_id| Token {
                terminal_id,
                saw_newline,
                value: if !text.is_empty() || terminal_id == TerminalId::StringLiteral {
                    Some(text)
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

    fn optional_exponent(&mut self, text: &mut String) -> Result<()> {
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

    fn unexpected_err(&mut self) -> ParseError {
        if let Some(&ch) = self.chars.peek() {
            ParseError::IllegalCharacter(ch)
        } else {
            ParseError::UnexpectedEnd
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

    fn regular_expression_literal(&mut self, text: &mut String) -> Result<TerminalId> {
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

    fn advance_impl(
        &mut self,
        parser: &Parser,
        text: &mut String,
        saw_newline: &mut bool,
    ) -> Result<TerminalId> {
        while let Some(c) = self.chars.next() {
            match c {
                // WhiteSpace
                '\u{9}' | '\u{b}' | '\u{c}' | '\u{20}' | '\u{a0}' | '\u{feff}' => {
                    continue;
                }
                // LineTerminator
                '\u{a}' | '\u{d}' | '\u{2028}' | '\u{2029}' => {
                    *saw_newline = true;
                    continue;
                }
                // Idents
                '$' | '_' | 'a'..='z' | 'A'..='Z' => {
                    let mut var = String::new();
                    var.push(c);
                    while let Some(&ch) = self.chars.peek() {
                        match ch {
                            '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                self.chars.next();
                                var.push(ch);
                            }
                            _ => break,
                        }
                    }
                    if parser.can_accept_terminal(TerminalId::IdentifierName) {
                        *text = var;
                        return Ok(TerminalId::IdentifierName);
                    }
                    match &var as &str {
                        "as" => return Ok(TerminalId::As),
                        "async" => return Ok(TerminalId::Async),
                        "await" => return Ok(TerminalId::Await),
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
                        "from" => return Ok(TerminalId::From),
                        "function" => return Ok(TerminalId::Function),
                        "get" => return Ok(TerminalId::Get),
                        "if" => return Ok(TerminalId::If),
                        "import" => return Ok(TerminalId::Import),
                        "in" => return Ok(TerminalId::In),
                        "instanceof" => return Ok(TerminalId::Instanceof),
                        "let" => return Ok(TerminalId::Let),
                        "new" => return Ok(TerminalId::New),
                        "of" => return Ok(TerminalId::Of),
                        "return" => return Ok(TerminalId::Return),
                        "set" => return Ok(TerminalId::Set),
                        "static" => return Ok(TerminalId::Static),
                        "super" => return Ok(TerminalId::Super),
                        "switch" => return Ok(TerminalId::Switch),
                        "target" => return Ok(TerminalId::Target),
                        "this" => return Ok(TerminalId::This),
                        "throw" => return Ok(TerminalId::Throw),
                        "try" => return Ok(TerminalId::Try),
                        "typeof" => return Ok(TerminalId::Typeof),
                        "var" => return Ok(TerminalId::Var),
                        "void" => return Ok(TerminalId::Void),
                        "while" => return Ok(TerminalId::While),
                        "with" => return Ok(TerminalId::With),
                        "yield" => return Ok(TerminalId::Yield),
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
                    // The SourceCharacter immediately following a NumericLiteral must not be an IdentifierStart or DecimalDigit.
                    if let Some(&ch) = self.chars.peek() {
                        if let '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' = ch {
                            return Err(ParseError::IllegalCharacter(ch));
                        }
                    }
                    return Ok(TerminalId::NumericLiteral);
                }
                // Strings
                '"' | '\'' => {
                    // include quotes?
                    while let Some(ch) = self.chars.next() {
                        if ch == c {
                            break;
                        }
                        text.push(ch)
                    }
                    return Ok(TerminalId::StringLiteral);
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
                        return Ok(TerminalId::HyphenMinusHyphenMinus);
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
                        // The SourceCharacter immediately following a NumericLiteral must not be an IdentifierStart or DecimalDigit.
                        if let Some(&ch) = self.chars.peek() {
                            if let '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' = ch {
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
                        while let Some(ch) = self.chars.next() {
                            match ch {
                                '\u{a}' | '\u{d}' | '\u{2028}' | '\u{2029}' => break,
                                _ => continue,
                            }
                        }
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
                x => return Err(ParseError::IllegalCharacter(x)),
            }
        }
        Ok(TerminalId::End)
    }
}
