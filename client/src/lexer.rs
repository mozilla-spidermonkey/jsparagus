use crate::parser_runtime::{Token, TokenStream};

pub struct Lexer<Iter: Iterator<Item = char>> {
    chars: std::iter::Peekable<Iter>,
    current: Option<Token>,
}

impl<Iter> Lexer<Iter>
where
    Iter: Iterator<Item = char>,
{
    pub fn new(chars: Iter) -> Lexer<Iter> {
        let mut result = Lexer {
            chars: chars.into_iter().peekable(),
            current: None,
        };
        result.current = result.advance();
        result
    }

    fn advance(&mut self) -> Option<Token> {
        while let Some(c) = self.chars.next() {
            match c {
                // WhiteSpace
                '\u{9}' | '\u{b}' | '\u{c}' | '\u{20}' | '\u{a0}' | '\u{feff}' => {
                    continue;
                }
                // LineTerminator
                '\u{a}' | '\u{d}' | '\u{2028}' | '\u{2029}' => {
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
                    match &var as &str {
                        "as" => return Some(Token::As),
                        "async" => return Some(Token::Async),
                        "await" => return Some(Token::Await),
                        "break" => return Some(Token::Break),
                        "case" => return Some(Token::Case),
                        "catch" => return Some(Token::Catch),
                        "class" => return Some(Token::Class),
                        "const" => return Some(Token::Const),
                        "continue" => return Some(Token::Continue),
                        "debugger" => return Some(Token::Debugger),
                        "default" => return Some(Token::Default),
                        "delete" => return Some(Token::Delete),
                        "do" => return Some(Token::Do),
                        "else" => return Some(Token::Else),
                        "export" => return Some(Token::Export),
                        "extends" => return Some(Token::Extends),
                        "finally" => return Some(Token::Finally),
                        "for" => return Some(Token::For),
                        "from" => return Some(Token::From),
                        "function" => return Some(Token::Function),
                        "get" => return Some(Token::Get),
                        "if" => return Some(Token::If),
                        "import" => return Some(Token::Import),
                        "in" => return Some(Token::In),
                        "instanceof" => return Some(Token::Instanceof),
                        "let" => return Some(Token::Let),
                        "new" => return Some(Token::New),
                        "of" => return Some(Token::Of),
                        "return" => return Some(Token::Return),
                        "set" => return Some(Token::Set),
                        "static" => return Some(Token::Static),
                        "super" => return Some(Token::Super),
                        "switch" => return Some(Token::Switch),
                        "target" => return Some(Token::Target),
                        "this" => return Some(Token::This),
                        "throw" => return Some(Token::Throw),
                        "try" => return Some(Token::Try),
                        "typeof" => return Some(Token::Typeof),
                        "var" => return Some(Token::Var),
                        "void" => return Some(Token::Void),
                        "while" => return Some(Token::While),
                        "with" => return Some(Token::With),
                        "yield" => return Some(Token::Yield),
                        "null" => return Some(Token::NullLiteral),
                        "true" => return Some(Token::BooleanLiteral),
                        "false" => return Some(Token::BooleanLiteral),
                        _ => return Some(Token::Identifier),
                    }
                }
                // Numbers
                '0'..='9' => {
                    let mut var = String::new();
                    var.push(c);
                    while let Some(next) = self.chars.peek() {
                        match next {
                            '0'..='9' => {
                                self.chars.next();
                                var.push(c);
                            }
                            _ => break,
                        }
                    }
                    let _ = var;
                    return Some(Token::NumericLiteral);
                }
                // Strings
                '"' | '\'' => {
                    // include quotes?
                    let mut var = String::new();
                    self.chars.next();
                    while let Some(ch) = self.chars.next() {
                        if ch == c {
                            break;
                        }
                        var.push(ch)
                    }
                    let _ = var;
                    return Some(Token::StringLiteral);
                }

                '`' => {
                    // include quotes?
                    let mut var = String::new();
                    self.chars.next();
                    while let Some(ch) = self.chars.next() {
                        if ch == '$' && self.chars.peek() == Some(&'{') {
                            self.chars.next();
                            return Some(Token::TemplateHead);
                        }
                        if ch == '`' {
                            return Some(Token::NoSubstitutionTemplate);
                        }
                        var.push(ch)
                    }
                    return Some(Token::StringLiteral);
                }

                '!' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(Token::ExclamationMarkEqualsSignEqualsSign);
                            }
                            _ => return Some(Token::ExclamationMarkEqualsSign),
                        }
                    }
                    _ => return Some(Token::ExclamationMark),
                },

                '%' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::PercentSignEqualsSign);
                    }
                    _ => return Some(Token::PercentSign),
                },

                '&' => match self.chars.peek() {
                    Some('&') => {
                        self.chars.next();
                        return Some(Token::AmpersandAmpersand);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::AmpersandEqualsSign);
                    }
                    _ => return Some(Token::Ampersand),
                },

                '*' => match self.chars.peek() {
                    Some('*') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(Token::AsteriskAsteriskEqualsSign);
                            }
                            _ => return Some(Token::AsteriskAsterisk),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::AsteriskEqualsSign);
                    }
                    _ => return Some(Token::Asterisk),
                },

                '+' => match self.chars.peek() {
                    Some('+') => {
                        self.chars.next();
                        return Some(Token::PlusSignPlusSign);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::PlusSignEqualsSign);
                    }
                    _ => return Some(Token::PlusSign),
                },

                '-' => match self.chars.peek() {
                    Some('-') => {
                        self.chars.next();
                        return Some(Token::HyphenMinusHyphenMinus);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::HyphenMinusEqualsSign);
                    }
                    _ => return Some(Token::HyphenMinus),
                },

                '.' => match self.chars.peek() {
                    Some('.') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('.') => {
                                self.chars.next();
                                return Some(Token::FullStopFullStopFullStop);
                            }
                            _ => return None,
                        }
                    }
                    _ => return Some(Token::FullStop),
                },

                '/' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::SolidusEqualsSign);
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
                    _ => return Some(Token::Solidus),
                },

                '<' => match self.chars.peek() {
                    Some('<') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(Token::LessThanSignLessThanSignEqualsSign);
                            }
                            _ => return Some(Token::LessThanSignLessThanSign),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::LessThanSignEqualsSign);
                    }
                    _ => return Some(Token::LessThanSign),
                },

                '=' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Some(Token::EqualsSignEqualsSignEqualsSign);
                            }
                            _ => return Some(Token::EqualsSignEqualsSign),
                        }
                    }
                    Some('>') => {
                        self.chars.next();
                        return Some(Token::Arrow);
                    }
                    _ => return Some(Token::EqualsSign),
                },

                '>' => {
                    match self.chars.peek() {
                        Some('>') => {
                            self.chars.next();
                            match self.chars.peek() {
                                Some('>') => {
                                    self.chars.next();
                                    match self.chars.peek() {
                                        Some('=') => {
                                            self.chars.next();
                                            return Some(Token::GreaterThanSignGreaterThanSignGreaterThanSignEqualsSign);
                                        }
                                        _ => return Some(
                                            Token::GreaterThanSignGreaterThanSignGreaterThanSign,
                                        ),
                                    }
                                }
                                Some('=') => {
                                    self.chars.next();
                                    return Some(Token::GreaterThanSignGreaterThanSignEqualsSign);
                                }
                                _ => return Some(Token::GreaterThanSignGreaterThanSign),
                            }
                        }
                        Some('=') => {
                            self.chars.next();
                            return Some(Token::GreaterThanSignEqualsSign);
                        }
                        _ => return Some(Token::GreaterThanSign),
                    }
                }

                '^' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::CircumflexAccentEqualsSign);
                    }
                    _ => return Some(Token::CircumflexAccent),
                },

                '|' => match self.chars.peek() {
                    Some('|') => {
                        self.chars.next();
                        return Some(Token::VerticalLineVerticalLine);
                    }
                    Some('=') => {
                        self.chars.next();
                        return Some(Token::VerticalLineEqualsSign);
                    }
                    _ => return Some(Token::VerticalLine),
                },

                '(' => return Some(Token::LeftParenthesis),
                ')' => return Some(Token::RightParenthesis),
                ',' => return Some(Token::Comma),
                ':' => return Some(Token::Colon),
                ';' => return Some(Token::Semicolon),
                '?' => return Some(Token::QuestionMark),
                '[' => return Some(Token::LeftSquareBracket),
                ']' => return Some(Token::RightSquareBracket),
                '{' => return Some(Token::LeftCurlyBracket),
                '}' => return Some(Token::RightCurlyBracket),
                '~' => return Some(Token::Tilde),
                _ => return None,
            }
        }
        Some(Token::End)
    }
}

impl<Iter> TokenStream for Lexer<Iter>
where
    Iter: Iterator<Item = char>,
{
    type Token = Token;
    fn peek(&mut self) -> &Self::Token {
        self.current.as_ref().unwrap()
    }
    fn take(&mut self) -> Self::Token {
        let result = self.current.take().unwrap();
        self.current = self.advance();
        result
    }
    fn token_as_index(t: &Self::Token) -> usize {
        t.get_id() as usize
    }
}
