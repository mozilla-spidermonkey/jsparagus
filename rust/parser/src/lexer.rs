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

    pub fn next(&mut self, parser: &Parser) -> Option<Token> {
        let mut text = String::new();
        let mut saw_newline = false;
        if let Some(terminal_id) = self.advance_impl(parser, &mut text, &mut saw_newline) {
            Some(Token {
                terminal_id,
                saw_newline,
                value: if text.len() == 0 { None } else { Some(text) },
            })
        } else {
            None
        }
    }

    fn advance_impl(
        &mut self,
        parser: &Parser,
        text: &mut String,
        saw_newline: &mut bool,
    ) -> Option<TerminalId> {
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
                        return Some(TerminalId::IdentifierName);
                    }
                    match &var as &str {
                        "as" => return Some(TerminalId::As),
                        "async" => return Some(TerminalId::Async),
                        "await" => return Some(TerminalId::Await),
                        "break" => return Some(TerminalId::Break),
                        "case" => return Some(TerminalId::Case),
                        "catch" => return Some(TerminalId::Catch),
                        "class" => return Some(TerminalId::Class),
                        "const" => return Some(TerminalId::Const),
                        "continue" => return Some(TerminalId::Continue),
                        "debugger" => return Some(TerminalId::Debugger),
                        "default" => return Some(TerminalId::Default),
                        "delete" => return Some(TerminalId::Delete),
                        "do" => return Some(TerminalId::Do),
                        "else" => return Some(TerminalId::Else),
                        "export" => return Some(TerminalId::Export),
                        "extends" => return Some(TerminalId::Extends),
                        "finally" => return Some(TerminalId::Finally),
                        "for" => return Some(TerminalId::For),
                        "from" => return Some(TerminalId::From),
                        "function" => return Some(TerminalId::Function),
                        "get" => return Some(TerminalId::Get),
                        "if" => return Some(TerminalId::If),
                        "import" => return Some(TerminalId::Import),
                        "in" => return Some(TerminalId::In),
                        "instanceof" => return Some(TerminalId::Instanceof),
                        "let" => return Some(TerminalId::Let),
                        "new" => return Some(TerminalId::New),
                        "of" => return Some(TerminalId::Of),
                        "return" => return Some(TerminalId::Return),
                        "set" => return Some(TerminalId::Set),
                        "static" => return Some(TerminalId::Static),
                        "super" => return Some(TerminalId::Super),
                        "switch" => return Some(TerminalId::Switch),
                        "target" => return Some(TerminalId::Target),
                        "this" => return Some(TerminalId::This),
                        "throw" => return Some(TerminalId::Throw),
                        "try" => return Some(TerminalId::Try),
                        "typeof" => return Some(TerminalId::Typeof),
                        "var" => return Some(TerminalId::Var),
                        "void" => return Some(TerminalId::Void),
                        "while" => return Some(TerminalId::While),
                        "with" => return Some(TerminalId::With),
                        "yield" => return Some(TerminalId::Yield),
                        "null" => return Some(TerminalId::NullLiteral),
                        "true" => return Some(TerminalId::BooleanLiteral),
                        "false" => return Some(TerminalId::BooleanLiteral),
                        _ => {
                            *text = var;
                            return Some(TerminalId::Identifier);
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
                    return Some(TerminalId::NumericLiteral);
                }
                // Strings
                '"' | '\'' => {
                    // include quotes?
                    self.chars.next();
                    while let Some(ch) = self.chars.next() {
                        if ch == c {
                            break;
                        }
                        text.push(ch)
                    }
                    return Some(TerminalId::StringLiteral);
                }

                '`' => {
                    // include quotes?
                    self.chars.next();
                    while let Some(ch) = self.chars.next() {
                        if ch == '$' && self.chars.peek() == Some(&'{') {
                            self.chars.next();
                            return Some(TerminalId::TemplateHead);
                        }
                        if ch == '`' {
                            return Some(TerminalId::NoSubstitutionTemplate);
                        }
                        text.push(ch)
                    }
                    return Some(TerminalId::StringLiteral);
                }

                '!' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(TerminalId::ExclamationMarkEqualsSignEqualsSign);
                            }
                            _ => return Some(TerminalId::ExclamationMarkEqualsSign),
                        }
                    }
                    _ => return Some(TerminalId::ExclamationMark),
                },

                '%' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::PercentSignEqualsSign);
                    }
                    _ => return Some(TerminalId::PercentSign),
                },

                '&' => match self.chars.peek() {
                    Some('&') => {
                        self.chars.next();
                        return Some(TerminalId::AmpersandAmpersand);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::AmpersandEqualsSign);
                    }
                    _ => return Some(TerminalId::Ampersand),
                },

                '*' => match self.chars.peek() {
                    Some('*') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(TerminalId::AsteriskAsteriskEqualsSign);
                            }
                            _ => return Some(TerminalId::AsteriskAsterisk),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::AsteriskEqualsSign);
                    }
                    _ => return Some(TerminalId::Asterisk),
                },

                '+' => match self.chars.peek() {
                    Some('+') => {
                        self.chars.next();
                        return Some(TerminalId::PlusSignPlusSign);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::PlusSignEqualsSign);
                    }
                    _ => return Some(TerminalId::PlusSign),
                },

                '-' => match self.chars.peek() {
                    Some('-') => {
                        self.chars.next();
                        return Some(TerminalId::HyphenMinusHyphenMinus);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::HyphenMinusEqualsSign);
                    }
                    _ => return Some(TerminalId::HyphenMinus),
                },

                '.' => match self.chars.peek() {
                    Some('.') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('.') => {
                                self.chars.next();
                                return Some(TerminalId::FullStopFullStopFullStop);
                            }
                            _ => return None,
                        }
                    }
                    _ => return Some(TerminalId::FullStop),
                },

                '/' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::SolidusEqualsSign);
                    }
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
                            if ch == '*' && self.chars.peek() == Some(&'/') {
                                self.chars.next();
                                break;
                            }
                        }
                        continue;
                    }
                    _ => return Some(TerminalId::Solidus),
                },

                '<' => match self.chars.peek() {
                    Some('<') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(TerminalId::LessThanSignLessThanSignEqualsSign);
                            }
                            _ => return Some(TerminalId::LessThanSignLessThanSign),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::LessThanSignEqualsSign);
                    }
                    _ => return Some(TerminalId::LessThanSign),
                },

                '=' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(TerminalId::EqualsSignEqualsSignEqualsSign);
                            }
                            _ => return Some(TerminalId::EqualsSignEqualsSign),
                        }
                    }
                    Some('>') => {
                        self.chars.next();
                        return Some(TerminalId::Arrow);
                    }
                    _ => return Some(TerminalId::EqualsSign),
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
                                        return Some(TerminalId::GreaterThanSignGreaterThanSignGreaterThanSignEqualsSign);
                                    }
                                    _ => return Some(
                                        TerminalId::GreaterThanSignGreaterThanSignGreaterThanSign,
                                    ),
                                }
                            }
                            Some('=') => {
                                self.chars.next();
                                return Some(TerminalId::GreaterThanSignGreaterThanSignEqualsSign);
                            }
                            _ => return Some(TerminalId::GreaterThanSignGreaterThanSign),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::GreaterThanSignEqualsSign);
                    }
                    _ => return Some(TerminalId::GreaterThanSign),
                },

                '^' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::CircumflexAccentEqualsSign);
                    }
                    _ => return Some(TerminalId::CircumflexAccent),
                },

                '|' => match self.chars.peek() {
                    Some('|') => {
                        self.chars.next();
                        return Some(TerminalId::VerticalLineVerticalLine);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(TerminalId::VerticalLineEqualsSign);
                    }
                    _ => return Some(TerminalId::VerticalLine),
                },

                '(' => return Some(TerminalId::LeftParenthesis),
                ')' => return Some(TerminalId::RightParenthesis),
                ',' => return Some(TerminalId::Comma),
                ':' => return Some(TerminalId::Colon),
                ';' => return Some(TerminalId::Semicolon),
                '?' => return Some(TerminalId::QuestionMark),
                '[' => return Some(TerminalId::LeftSquareBracket),
                ']' => return Some(TerminalId::RightSquareBracket),
                '{' => return Some(TerminalId::LeftCurlyBracket),
                '}' => return Some(TerminalId::RightCurlyBracket),
                '~' => return Some(TerminalId::Tilde),
                _ => return None,
            }
        }
        Some(TerminalId::End)
    }
}
