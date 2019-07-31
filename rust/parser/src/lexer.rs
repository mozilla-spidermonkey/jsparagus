use crate::errors::{ParseError, Result};
use crate::parser::Parser;
use generated_parser::{TerminalId, Token};

pub struct Lexer<Iter: Iterator<Item = char>> {
    chars: std::iter::Peekable<Iter>,
}

impl<Iter> Lexer<Iter>
where
    Iter: Iterator<Item = char>,
{
    pub fn new(chars: Iter) -> Lexer<Iter> {
        Lexer {
            chars: chars.into_iter().peekable(),
        }
    }

    pub fn next(&mut self, parser: &Parser) -> Result<Token> {
        let mut text = String::new();
        let mut saw_newline = false;
        self.advance_impl(parser, &mut text, &mut saw_newline)
            .map(|terminal_id| {
                Token {
                    terminal_id,
                    saw_newline,
                    value: if text.len() == 0 { None } else { Some(text) },
                }
            })
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
                        "true" => return Ok(TerminalId::BooleanLiteral),
                        "false" => return Ok(TerminalId::BooleanLiteral),
                        _ => {
                            *text = var;
                            return Ok(TerminalId::Identifier);
                        }
                    }
                }
                // Numbers
                '0'..='9' => {
                    text.push(c);
                    while let Some(next) = self.chars.peek() {
                        match next {
                            '0'..='9' => {
                                self.chars.next();
                                text.push(c);
                            }
                            _ => break,
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
                    _ => return Ok(TerminalId::FullStop),
                },

                '/' => match self.chars.peek() {
                    // Comments take priority over regexes
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
                            text.push(c);
                            loop {
                                if let Some(ch) = self.chars.next() {
                                    text.push(ch);
                                    if ch == '/' {
                                        break;
                                    }
                                } else {
                                    return Err(ParseError::UnterminatedRegExp);
                                }
                            }
                            while let Some(&ch) = self.chars.peek() {
                                match ch {
                                    '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                                        self.chars.next();
                                        text.push(ch);
                                    }
                                    _ => break,
                                }
                            }
                            return Ok(TerminalId::RegularExpressionLiteral);
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
