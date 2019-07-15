use crate::ast::Token;
use crate::parser_runtime::TokenStream;

pub struct Lexer<Iter: Iterator<Item = char>> {
    chars: std::iter::Peekable<Iter>,
    current: Option<Token>,
}

impl<Iter> Lexer<Iter>
where
    Iter: Iterator<Item = char>,
{
    pub fn new(chars: Iter) -> Lexer<Iter> {
        Lexer {
            chars: chars.into_iter().peekable(),
            current: None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c == '#' {
                while let Some(c) = self.chars.next() {
                    if c == '\n' {
                        break;
                    }
                }
            } else if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn take_while<P: Fn(char) -> bool>(&mut self, predicate: P, out: &mut String) {
        while let Some(&c) = self.chars.peek() {
            if !predicate(c) {
                break;
            }
            out.push(c);
            self.chars.next();
        }
    }

    fn is_identifier_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_identifier_part(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn settle(&mut self) {
        assert!(self.current.is_none());
        self.skip_whitespace();
        let token = match self.chars.next() {
            None => Token::End,
            Some(';') => Token::Semicolon,
            Some('{') => Token::OpenBrace,
            Some('}') => Token::CloseBrace,
            Some('?') => Token::QuestionMark,
            Some('=') => match self.chars.peek() {
                Some('>') => {
                    self.chars.next();
                    Token::Arrow
                }
                _ => Token::EqualSign,
            },

            Some('"') => {
                let mut s = String::new();
                while let Some(c) = self.chars.next() {
                    if c == '"' {
                        break;
                    } else if c == '\n' || c == '\\' {
                        panic!(
                            "not implemented: syntax error, string contains newline or backslash"
                        );
                    } else {
                        s.push(c);
                    }
                }
                if self.chars.peek().is_none() {
                    panic!("not implemented: syntax error, unterminated string");
                }
                Token::String(s)
            }

            Some(c) => {
                if Lexer::<Iter>::is_identifier_start(c) {
                    let mut id = String::new();
                    id.push(c);
                    self.take_while(Lexer::<Iter>::is_identifier_part, &mut id);
                    if id == "goal" {
                        Token::Goal
                    } else if id == "nt" {
                        Token::Nt
                    } else if id == "token" {
                        Token::Token
                    } else if id == "var" {
                        Token::Var
                    } else {
                        Token::Identifier(id)
                    }
                } else {
                    panic!("not implemented: syntax error, illegal character {:?}", c);
                }
            }
        };
        self.current = Some(token);
    }
}

impl<Iter> TokenStream for Lexer<Iter>
where
    Iter: Iterator<Item = char>,
{
    type Token = Token;
    fn peek(&mut self) -> &Self::Token {
        self.settle();
        self.current.as_ref().unwrap()
    }
    fn take(&mut self) -> Self::Token {
        self.current.take().unwrap()
    }
    fn token_as_index(t: &Self::Token) -> usize {
        t.get_id() as usize
    }
}
