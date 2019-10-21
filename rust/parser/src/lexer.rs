//! JavaScript lexer.

use crate::parser::Parser;
use bumpalo::{collections::String, Bump};
use generated_parser::{ParseError, Result, TerminalId, Token};
use std::convert::TryFrom;
use std::str::Chars;

pub struct Lexer<'alloc> {
    allocator: &'alloc Bump,

    /// Length of the input text, in UTF-8 bytes.
    source_length: usize,

    /// Iterator over the remaining not-yet-parsed input.
    chars: Chars<'alloc>,

    /// True if the current position is before the first
    /// token of a line (or on a line with no tokens).
    is_on_new_line: bool,
}

impl<'alloc> Lexer<'alloc> {
    pub fn new(allocator: &'alloc Bump, chars: Chars<'alloc>) -> Lexer<'alloc> {
        let source_length = chars.as_str().len();
        Lexer {
            allocator,
            source_length,
            chars,
            is_on_new_line: true,
        }
    }

    fn is_looking_at(&self, s: &str) -> bool {
        self.chars.as_str().starts_with(s)
    }

    pub fn offset(&self) -> usize {
        self.source_length - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.as_str().chars().next()
    }

    pub fn next<'parser>(&mut self, parser: &Parser<'parser>) -> Result<'alloc, Token<'alloc>> {
        let (offset, value, terminal_id) = self.advance_impl(parser)?;
        let is_on_new_line = self.is_on_new_line;
        self.is_on_new_line = false;
        Ok(Token {
            terminal_id,
            offset,
            is_on_new_line,
            value,
        })
    }

    fn unexpected_err(&mut self) -> ParseError<'alloc> {
        if let Some(ch) = self.peek() {
            ParseError::IllegalCharacter(ch)
        } else {
            ParseError::UnexpectedEnd
        }
    }
}

// ----------------------------------------------------------------------------
// 11.1 Unicode Format-Control Characters

/// U+200C ZERO WIDTH NON-JOINER, abbreviated in the spec as <ZWNJ>.
/// Specially permitted in identifiers.
const ZWNJ: char = '\u{200c}';

/// U+200D ZERO WIDTH JOINER, abbreviated as <ZWJ>.
/// Specially permitted in identifiers.
const ZWJ: char = '\u{200d}';

/// U+FEFF ZERO WIDTH NO-BREAK SPACE, abbreviated <ZWNBSP>.
/// Considered a whitespace character in JS.
const ZWNBSP: char = '\u{feff}';

// ----------------------------------------------------------------------------
// 11.2 White Space

/// U+0009 CHARACTER TABULATION, abbreviated <TAB>.
const TAB: char = '\u{9}';

/// U+000B VERTICAL TAB, abbreviated <VT>.
const VT: char = '\u{b}';

/// U+000C FORM FEED, abbreviated <FF>.
const FF: char = '\u{c}';

/// U+0020 SPACE, abbreviated <SP>.
const SP: char = '\u{20}';

/// U+00A0 NON-BREAKING SPACE, abbreviated <NBSP>.
const NBSP: char = '\u{a0}';

// ----------------------------------------------------------------------------
// 11.3 Line Terminators

///  U+000A LINE FEED, abbreviated in the spec as <LF>.
const LF: char = '\u{a}';

/// U+000D CARRIAGE RETURN, abbreviated in the spec as <CR>.
const CR: char = '\u{d}';

/// U+2028 LINE SEPARATOR, abbreviated <LS>.
const LS: char = '\u{2028}';

/// U+2029 PARAGRAPH SEPARATOR, abbreviated <PS>.
const PS: char = '\u{2029}';

// ----------------------------------------------------------------------------
// 11.4 Comments
//
// Comment::
//     MultiLineComment
//     SingleLineComment

impl<'alloc> Lexer<'alloc> {
    /// Skip a *MultiLineComment*.
    ///
    /// ```text
    /// MultiLineComment ::
    ///     `/*` MultiLineCommentChars? `*/`
    ///
    /// MultiLineCommentChars ::
    ///     MultiLineNotAsteriskChar MultiLineCommentChars?
    ///     `*` PostAsteriskCommentChars?
    ///
    /// PostAsteriskCommentChars ::
    ///     MultiLineNotForwardSlashOrAsteriskChar MultiLineCommentChars?
    ///     `*` PostAsteriskCommentChars?
    ///
    /// MultiLineNotAsteriskChar ::
    ///     SourceCharacter but not `*`
    ///
    /// MultiLineNotForwardSlashOrAsteriskChar ::
    ///     SourceCharacter but not one of `/` or `*`
    /// ```
    ///
    /// (B.1.3 splits MultiLineComment into two nonterminals: MultiLineComment
    /// and SingleLineDelimitedComment. The point of that is to help specify
    /// that a SingleLineHTMLCloseComment must occur at the start of a line. We
    /// use `is_on_new_line` for that.)
    ///
    fn skip_multi_line_comment(&mut self, builder: &mut AutoCow<'alloc>) {
        while let Some(ch) = self.chars.next() {
            match ch {
                '*' if self.peek() == Some('/') => {
                    self.chars.next();
                    *builder = AutoCow::new(&self);
                    return;
                }
                CR | LF | PS | LS => {
                    self.is_on_new_line = true;
                }
                _ => {}
            }
        }
    }

    /// Skip a *SingleLineComment* and the following *LineTerminatorSequence*,
    /// if any.
    ///
    /// ```text
    /// SingleLineComment ::
    ///     `//` SingleLineCommentChars?
    ///
    /// SingleLineCommentChars ::
    ///     SingleLineCommentChar SingleLineCommentChars?
    ///
    /// SingleLineCommentChar ::
    ///     SourceCharacter but not LineTerminator
    /// ```
    fn skip_single_line_comment(&mut self, builder: &mut AutoCow<'alloc>) {
        while let Some(ch) = self.chars.next() {
            match ch {
                CR | LF | LS | PS => break,
                _ => continue,
            }
        }
        *builder = AutoCow::new(&self);
        self.is_on_new_line = true;
    }
}

// ----------------------------------------------------------------------------
// 11.6 Names and Keywords

/// True if `c` is a one-character *IdentifierStart*.
/// (TODO: Handle *UnicodeEscapeSequence* elsewhere.)
///
/// ```text
/// IdentifierStart ::
///     UnicodeIDStart
///     `$`
///     `_`
///     `\` UnicodeEscapeSequence
///
/// UnicodeIDStart ::
///     > any Unicode code point with the Unicode property "ID_Start"
/// ```
fn is_identifier_start(c: char) -> bool {
    // TODO - Adjust this to match the Unicode ID_Start property (#23).
    c == '$' || c == '_' || c.is_alphabetic()
}

/// True if `c` is a one-character *IdentifierPart*.
/// (TODO: Handle *UnicodeEscapeSequence* elsewhere.)
///
/// ```text
/// IdentifierPart ::
///     UnicodeIDContinue
///     `$`
///     `\` UnicodeEscapeSequence
///     <ZWNJ>
///     <ZWJ>
///
/// UnicodeIDContinue ::
///     > any Unicode code point with the Unicode property "ID_Continue"
/// ```
fn is_identifier_part(c: char) -> bool {
    if (c as u32) < 128 {
        match c {
            '$' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        }
    } else {
        // TODO - Adjust this to match the Unicode ID_Continue property (#23).
        c.is_alphabetic() || c == ZWNJ || c == ZWJ
    }
}

impl<'alloc> Lexer<'alloc> {
    /// Scan an IdentifierName.
    ///
    /// ```text
    /// IdentifierName ::
    ///     IdentifierStart
    ///     IdentifierName IdentifierPart
    /// ```
    ///
    /// TODO: Implement *UnicodeEscapeSequence*.
    fn identifier(
        &mut self,
        offset: usize,
        mut builder: AutoCow<'alloc>,
    ) -> Result<'alloc, (usize, Option<&'alloc str>, TerminalId)> {
        while let Some(ch) = self.peek() {
            if !is_identifier_part(ch) {
                break;
            }
            self.chars.next();
            builder.push_matching(ch);
        }
        let text = builder.finish(&self);

        let id = match &text as &str {
            "as" => TerminalId::As,
            "async" => TerminalId::Async,
            "await" => TerminalId::Await,
            "break" => TerminalId::Break,
            "case" => TerminalId::Case,
            "catch" => TerminalId::Catch,
            "class" => TerminalId::Class,
            "const" => TerminalId::Const,
            "continue" => TerminalId::Continue,
            "debugger" => TerminalId::Debugger,
            "default" => TerminalId::Default,
            "delete" => TerminalId::Delete,
            "do" => TerminalId::Do,
            "else" => TerminalId::Else,
            "export" => TerminalId::Export,
            "extends" => TerminalId::Extends,
            "finally" => TerminalId::Finally,
            "for" => TerminalId::For,
            "from" => TerminalId::From,
            "function" => TerminalId::Function,
            "get" => TerminalId::Get,
            "if" => TerminalId::If,
            "import" => TerminalId::Import,
            "in" => TerminalId::In,
            "instanceof" => TerminalId::Instanceof,
            "let" => TerminalId::Let,
            "new" => TerminalId::New,
            "of" => TerminalId::Of,
            "return" => TerminalId::Return,
            "set" => TerminalId::Set,
            "static" => TerminalId::Static,
            "super" => TerminalId::Super,
            "switch" => TerminalId::Switch,
            "target" => TerminalId::Target,
            "this" => TerminalId::This,
            "throw" => TerminalId::Throw,
            "try" => TerminalId::Try,
            "typeof" => TerminalId::Typeof,
            "var" => TerminalId::Var,
            "void" => TerminalId::Void,
            "while" => TerminalId::While,
            "with" => TerminalId::With,
            "yield" => TerminalId::Yield,
            "null" => TerminalId::NullLiteral,
            "true" | "false" => TerminalId::BooleanLiteral,
            _ => TerminalId::Name,
        };

        Ok((offset, Some(text), id))
    }
}

// ----------------------------------------------------------------------------
// 11.8.3 Numeric Literals

impl<'alloc> Lexer<'alloc> {
    /// Advance over decimal digits in the input, returning true if any were
    /// found.
    ///
    /// ```text
    /// DecimalDigits ::
    ///     DecimalDigit
    ///     DecimalDigits DecimalDigit
    ///
    /// DecimalDigit :: one of
    ///     `0` `1` `2` `3` `4` `5` `6` `7` `8` `9`
    /// ```
    fn decimal_digits(&mut self) -> bool {
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

    /// Skip an ExponentPart, if present.
    ///
    /// ```text
    /// ExponentPart ::
    ///     ExponentIndicator SignedInteger
    ///
    /// ExponentIndicator :: one of
    ///     `e` `E`
    ///
    /// SignedInteger ::
    ///     DecimalDigits
    ///     `+` DecimalDigits
    ///     `-` DecimalDigits
    /// ```
    fn optional_exponent(&mut self) -> Result<'alloc, ()> {
        if let Some('e') | Some('E') = self.peek() {
            self.chars.next().unwrap();
            if let Some('+') | Some('-') = self.peek() {
                self.chars.next().unwrap();
            }
            if !self.decimal_digits() {
                // require at least one digit
                return Err(self.unexpected_err());
            }
        }
        Ok(())
    }

    /// ```text
    /// HexDigit :: one of
    ///     `0` `1` `2` `3` `4` `5` `6` `7` `8` `9` `a` `b` `c` `d` `e` `f` `A` `B` `C` `D` `E` `F`
    /// ```
    fn hex_digit(&mut self) -> Result<'alloc, u32> {
        match self.chars.next() {
            None => Err(ParseError::UnterminatedString),
            Some(c @ '0'..='9') => Ok(c as u32 - '0' as u32),
            Some(c @ 'a'..='f') => Ok(10 + (c as u32 - 'a' as u32)),
            Some(c @ 'A'..='F') => Ok(10 + (c as u32 - 'A' as u32)),
            Some(other) => Err(ParseError::IllegalCharacter(other)),
        }
    }

    /// Scan a NumericLiteral (defined in 11.8.3, extended by B.1.1) after
    /// having already consumed the first character, which was `0`.
    ///
    /// ```text
    /// NumericLiteral ::
    ///     DecimalLiteral
    ///     BinaryIntegerLiteral
    ///     OctalIntegerLiteral
    ///     HexIntegerLiteral
    ///     LegacyOctalIntegerLiteral
    /// ```
    fn numeric_literal_starting_with_zero(&mut self) -> Result<'alloc, ()> {
        match self.peek() {
            // BinaryIntegerLiteral ::
            //     `0b` BinaryDigits
            //     `0B` BinaryDigits
            //
            // BinaryDigits ::
            //     BinaryDigit
            //     BinaryDigits BinaryDigit
            //
            // BinaryDigit :: one of
            //     `0` `1`
            Some('b') | Some('B') => {
                self.chars.next().unwrap();
                let mut at_least_one = false;
                while let Some('0'..='1') = self.peek() {
                    at_least_one = true;
                    self.chars.next();
                }
                if !at_least_one {
                    return Err(self.unexpected_err());
                }
            }

            // OctalIntegerLiteral ::
            //     `0o` OctalDigits
            //     `0O` OctalDigits
            //
            // OctalDigits ::
            //     OctalDigit
            //     OctalDigits OctalDigit
            //
            // OctalDigit :: one of
            //     `0` `1` `2` `3` `4` `5` `6` `7`
            //
            Some('o') | Some('O') => {
                self.chars.next().unwrap();
                let mut at_least_one = false;
                while let Some('0'..='7') = self.peek() {
                    at_least_one = true;
                    self.chars.next();
                }
                if !at_least_one {
                    return Err(self.unexpected_err());
                }
            }

            // HexIntegerLiteral ::
            //     `0x` HexDigits
            //     `0X` HexDigits
            //
            // HexDigits ::
            //     HexDigit
            //     HexDigits HexDigit
            //
            // HexDigit :: one of
            //     `0` `1` `2` `3` `4` `5` `6` `7` `8` `9` `a` `b` `c` `d` `e` `f` `A` `B` `C` `D` `E` `F`
            Some('x') | Some('X') => {
                self.chars.next();
                let mut at_least_one = false;
                while let Some('0'..='9') | Some('a'..='f') | Some('A'..='F') = self.peek() {
                    at_least_one = true;
                    self.chars.next();
                }
                if !at_least_one {
                    return Err(self.unexpected_err());
                }
            }

            Some('.') | Some('e') | Some('E') => {
                return self.decimal_literal();
            }

            _ => {
                // This is almost always the token `0` in practice.
                //
                // In nonstrict code, as a legacy feature, other numbers
                // starting with `0` are allowed. If /0[0-7]+/ matches, it's a
                // LegacyOctalIntegerLiteral; but if we see an `8` or `9` in
                // the number, it's decimal. Decimal numbers can have a decimal
                // point and/or ExponentPart; octals can't.
                //
                // Neither is allowed with a BigIntLiteralSuffix `n`.
                //
                // LegacyOctalIntegerLiteral ::
                //     `0` OctalDigit
                //     LegacyOctalIntegerLiteral OctalDigit
                //
                // NonOctalDecimalIntegerLiteral ::
                //     `0` NonOctalDigit
                //     LegacyOctalLikeDecimalIntegerLiteral NonOctalDigit
                //     NonOctalDecimalIntegerLiteral DecimalDigit
                //
                // LegacyOctalLikeDecimalIntegerLiteral ::
                //     `0` OctalDigit
                //     LegacyOctalLikeDecimalIntegerLiteral OctalDigit
                //
                // NonOctalDigit :: one of
                //     `8` `9`
                //

                // TODO: implement `strict_mode` check
                let strict_mode = true;
                if !strict_mode {
                    // TODO: Distinguish between Octal and NonOctalDecimal.
                    // TODO: Support NonOctalDecimal followed by a decimal
                    //       point and/or ExponentPart.
                    self.decimal_digits();
                }
            }
        }

        self.check_after_numeric_literal()
    }

    /// Scan a NumericLiteral (defined in 11.8.3, extended by B.1.1) after
    /// having already consumed the first character, which is a decimal digit.
    ///
    /// This can also be called after having scanned input matching /0[0-7]*/
    /// when the next digit is `8` or `9` (that is, we belatedly realize this
    /// is a DecimalLiteral, after first trying to scan it as a
    /// LegacyOctalIntegerLiteral). TODO - `numeric_literal_starting_with_zero`
    /// isn't actually doing this yet, so we fail to parse literals like
    /// `091.1`.
    ///
    fn decimal_literal(&mut self) -> Result<'alloc, ()> {
        // DecimalLiteral ::
        //     DecimalIntegerLiteral `.` DecimalDigits? ExponentPart?
        //     `.` DecimalDigits ExponentPart?
        //     DecimalIntegerLiteral ExponentPart?
        //
        // DecimalIntegerLiteral ::
        //     `0`   #see `numeric_literal_starting_with_zero`
        //     NonZeroDigit DecimalDigits?
        //     NonOctalDecimalIntegerLiteral  #see `numeric_literal_
        //                                    #     starting_with_zero`
        //
        // NonZeroDigit :: one of
        //     `1` `2` `3` `4` `5` `6` `7` `8` `9`

        self.decimal_digits();
        if self.peek() == Some('.') {
            self.chars.next();
            self.decimal_digits();
        }
        self.optional_exponent()?;
        self.check_after_numeric_literal()
    }

    fn check_after_numeric_literal(&self) -> Result<'alloc, ()> {
        // The SourceCharacter immediately following a
        // NumericLiteral must not be an IdentifierStart or
        // DecimalDigit. (11.8.3)
        if let Some(ch) = self.peek() {
            if is_identifier_start(ch) || ch.is_digit(10) {
                return Err(ParseError::IllegalCharacter(ch));
            }
        }

        Ok(())
    }

    // ----------------------------------------------------------------------------
    // 11.8.4 String Literals

    /// Scan an escape sequence in a string literal.
    fn escape_sequence(&mut self, text: &mut String<'alloc>) -> Result<'alloc, ()> {
        match self.chars.next() {
            None => {
                return Err(ParseError::UnterminatedString);
            }
            Some(c) => match c {
                LF | LS | PS => {
                    // LineContinuation. Ignore it.
                }
                CR => {
                    // LineContinuation. Check for the sequence \r\n; otherwise
                    // ignore it.
                    if self.peek() == Some(LF) {
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
                    text.push(FF);
                }
                'n' => {
                    text.push(LF);
                }
                'r' => {
                    text.push(CR);
                }
                't' => {
                    text.push(TAB);
                }
                'v' => {
                    text.push(VT);
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

    fn string_literal(
        &mut self,
        stop: char,
    ) -> Result<'alloc, (usize, Option<&'alloc str>, TerminalId)> {
        let offset = self.offset() - 1;
        let mut builder = AutoCow::new(&self);
        loop {
            match self.chars.next() {
                None | Some('\r') | Some('\n') => {
                    return Err(ParseError::UnterminatedString);
                }

                Some(c @ '"') | Some(c @ '\'') => {
                    if c == stop {
                        return Ok((
                            offset,
                            Some(builder.finish(&self)),
                            TerminalId::StringLiteral,
                        ));
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

    // ----------------------------------------------------------------------------
    // 11.8.5 Regular Expression Literals

    fn regular_expression_backslash_sequence(
        &mut self,
        text: &mut String<'alloc>,
    ) -> Result<'alloc, ()> {
        text.push('\\');
        match self.chars.next() {
            None | Some(CR) | Some(LF) | Some(LS) | Some(PS) => Err(ParseError::UnterminatedRegExp),
            Some(c) => {
                text.push(c);
                Ok(())
            }
        }
    }

    fn regular_expression_literal(
        &mut self,
        builder: &mut AutoCow<'alloc>,
    ) -> Result<'alloc, (usize, Option<&'alloc str>, TerminalId)> {
        let offset = self.offset();

        loop {
            match self.chars.next() {
                None | Some(CR) | Some(LF) | Some(LS) | Some(PS) => {
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
                            None | Some(CR) | Some(LF) | Some(LS) | Some(PS) => {
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
            offset,
            Some(builder.finish(&self)),
            TerminalId::RegularExpressionLiteral,
        ))
    }

    fn advance_impl<'parser>(
        &mut self,
        parser: &Parser<'parser>,
    ) -> Result<'alloc, (usize, Option<&'alloc str>, TerminalId)> {
        let mut builder = AutoCow::new(&self);
        let mut start = self.offset();
        while let Some(c) = self.chars.next() {
            match c {
                // 11.2 White Space
                //
                // WhiteSpace ::
                //     <TAB>
                //     <VT>
                //     <FF>
                //     <SP>
                //     <NBSP>
                //     <ZWNBSP>
                //     <USP>
                TAB |
                VT |
                FF |
                SP |
                NBSP |
                ZWNBSP |
                '\u{1680}' | // Ogham space mark (in <USP>)
                '\u{2000}' ..= '\u{200a}' | // typesetting spaces (in <USP>)
                '\u{202f}' | // Narrow no-break space (in <USP>)
                '\u{205f}' | // Medium mathematical space (in <USP>)
                '\u{3000}' // Ideographic space (in <USP>)
                    => {
                    // TODO - The spec uses <USP> to stand for any character
                    // with category "Space_Separator" (Zs). New Unicode
                    // standards may add characters to this set. This should therefore be
                    // implemented using the Unicode database somehow.
                    builder = AutoCow::new(&self);
                    start = self.offset();
                    continue;
                }

                // 11.3 Line Terminators
                //
                // LineTerminator ::
                //     <LF>
                //     <CR>
                //     <LS>
                //     <PS>
                LF | CR | LS | PS => {
                    self.is_on_new_line = true;
                    builder = AutoCow::new(&self);
                    start = self.offset();
                    continue;
                }

                '0' => {
                    self.numeric_literal_starting_with_zero()?;

                    // Don't have to push_matching since push_different is never called.
                    return Ok((
                        start,
                        Some(builder.finish(&self)),
                        TerminalId::NumericLiteral,
                    ));
                }

                '1'..='9' => {
                    self.decimal_literal()?;

                    // Don't have to push_matching since push_different is never called.
                    return Ok((
                        start,
                        Some(builder.finish(&self)),
                        TerminalId::NumericLiteral,
                    ));
                }

                '"' | '\'' => {
                    return self.string_literal(c);
                }

                '`' => {
                    let mut builder = AutoCow::new(&self);
                    while let Some(ch) = self.chars.next() {
                        if ch == '$' && self.peek() == Some('{') {
                            self.chars.next();
                            return Ok((start, None, TerminalId::TemplateHead));
                        }
                        if ch == '`' {
                            return Ok((start, None, TerminalId::NoSubstitutionTemplate));
                        }
                        // TODO: Support escape sequences.
                        builder.push_matching(ch);
                    }
                    return Ok((start, Some(builder.finish(&self)), TerminalId::StringLiteral));
                }

                '!' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((start, None, TerminalId::StrictNotEqual));
                            }
                            _ => return Ok((start, None, TerminalId::LaxNotEqual)),
                        }
                    }
                    _ => return Ok((start, None, TerminalId::LogicalNot)),
                },

                '%' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::RemainderAssign));
                    }
                    _ => return Ok((start, None, TerminalId::Remainder)),
                },

                '&' => match self.peek() {
                    Some('&') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::LogicalAnd));
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::BitwiseAndAssign));
                    }
                    _ => return Ok((start, None, TerminalId::BitwiseAnd)),
                },

                '*' => match self.peek() {
                    Some('*') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((start, None, TerminalId::ExponentiateAssign));
                            }
                            _ => return Ok((start, None, TerminalId::Exponentiate)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::MultiplyAssign));
                    }
                    _ => return Ok((start, None, TerminalId::Star)),
                },

                '+' => match self.peek() {
                    Some('+') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::Increment));
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::AddAssign));
                    }
                    _ => return Ok((start, None, TerminalId::Plus)),
                },

                '-' => match self.peek() {
                    Some('-') => {
                        self.chars.next();
                        match self.peek() {
                            Some('>') if self.is_on_new_line => {
                                // B.1.3 SingleLineHTMLCloseComment
                                // TODO: Limit this to Script (not Module).
                                self.skip_single_line_comment(&mut builder);
                                continue;
                            }
                            _ => return Ok((start, None, TerminalId::Decrement)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::SubtractAssign));
                    }
                    _ => return Ok((start, None, TerminalId::Minus)),
                },

                '.' => match self.peek() {
                    Some('.') => {
                        self.chars.next();
                        match self.peek() {
                            Some('.') => {
                                self.chars.next();
                                return Ok((start, None, TerminalId::Ellipsis));
                            }
                            _ => return Err(ParseError::IllegalCharacter('.')),
                        }
                    }
                    Some('0'..='9') => {
                        self.decimal_digits();
                        self.optional_exponent()?;

                        // The SourceCharacter immediately following a
                        // NumericLiteral must not be an IdentifierStart or
                        // DecimalDigit. (11.8.3)
                        if let Some(ch) = self.peek() {
                            if is_identifier_start(ch) || ch.is_digit(10) {
                                return Err(ParseError::IllegalCharacter(ch));
                            }
                        }

                        // Don't have to push_matching since push_different is never called.
                        return Ok((start, Some(builder.finish(&self)), TerminalId::NumericLiteral));
                    }
                    _ => return Ok((start, None, TerminalId::Dot)),
                },

                '/' => match self.peek() {
                    Some('/') => {
                        // SingleLineComment :: `//` SingleLineCommentChars?
                        self.chars.next();
                        self.skip_single_line_comment(&mut builder);
                        start = self.offset();
                        continue;
                    }
                    Some('*') => {
                        self.chars.next();
                        self.skip_multi_line_comment(&mut builder);
                        start = self.offset();
                        continue;
                    }
                    _ => {
                        if parser.can_accept_terminal(TerminalId::RegularExpressionLiteral) {
                            builder.push_matching('/');
                            return self.regular_expression_literal(&mut builder);
                        }
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((start, None, TerminalId::DivideAssign));
                            }
                            _ => return Ok((start, None, TerminalId::Divide)),
                        }
                    }
                },

                '<' => match self.peek() {
                    Some('<') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((start, None, TerminalId::LeftShiftAssign));
                            }
                            _ => return Ok((start, None, TerminalId::LeftShift)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::LessThanOrEqualTo));
                    }
                    Some('!') if self.is_looking_at("!--") => {
                        // B.1.3 SingleLineHTMLOpenComment. Note that the above
                        // `is_looking_at` test peeked ahead at the next three
                        // characters of input. This lookahead is necessary
                        // because `x<!--` has a comment but `x<!-y` does not.
                        //
                        // TODO: Limit this to Script (not Module).
                        self.skip_single_line_comment(&mut builder);
                        start = self.offset();
                        continue;
                    }
                    _ => return Ok((start, None, TerminalId::LessThan)),
                },

                '=' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.peek() {
                            Some('=') => {
                                self.chars.next();
                                return Ok((start, None, TerminalId::StrictEqual));
                            }
                            _ => return Ok((start, None, TerminalId::LaxEqual)),
                        }
                    }
                    Some('>') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::Arrow));
                    }
                    _ => return Ok((start, None, TerminalId::EqualSign)),
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
                                        return Ok((start, None, TerminalId::UnsignedRightShiftAssign));
                                    }
                                    _ => return Ok((start, None, TerminalId::UnsignedRightShift)),
                                }
                            }
                            Some('=') => {
                                self.chars.next();
                                return Ok((start, None, TerminalId::SignedRightShiftAssign));
                            }
                            _ => return Ok((start, None, TerminalId::SignedRightShift)),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::GreaterThanOrEqualTo));
                    }
                    _ => return Ok((start, None, TerminalId::GreaterThan)),
                },

                '^' => match self.peek() {
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::BitwiseXorAssign));
                    }
                    _ => return Ok((start, None, TerminalId::BitwiseXor)),
                },

                '|' => match self.peek() {
                    Some('|') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::LogicalOr));
                    }
                    Some('=') => {
                        self.chars.next();
                        return Ok((start, None, TerminalId::BitwiseOrAssign));
                    }
                    _ => return Ok((start, None, TerminalId::BitwiseOr)),
                },

                '(' => return Ok((start, None, TerminalId::OpenParenthesis)),
                ')' => return Ok((start, None, TerminalId::CloseParenthesis)),
                ',' => return Ok((start, None, TerminalId::Comma)),
                ':' => return Ok((start, None, TerminalId::Colon)),
                ';' => return Ok((start, None, TerminalId::Semicolon)),
                '?' => return Ok((start, None, TerminalId::QuestionMark)),
                '[' => return Ok((start, None, TerminalId::OpenBracket)),
                ']' => return Ok((start, None, TerminalId::CloseBracket)),
                '{' => return Ok((start, None, TerminalId::OpenBrace)),
                '}' => return Ok((start, None, TerminalId::CloseBrace)),
                '~' => return Ok((start, None, TerminalId::BitwiseNot)),

                // Idents
                '$' | '_' | 'a'..='z' | 'A'..='Z' => {
                    builder.push_matching(c);
                    return self.identifier(start, builder);
                }

                other => {
                    if is_identifier_start(other) {
                        builder.push_matching(other);
                        return self.identifier(start, builder);
                    }
                    return Err(ParseError::IllegalCharacter(other));
                }
            }
        }
        Ok((start, None, TerminalId::End))
    }
}

struct AutoCow<'alloc> {
    start: &'alloc str,
    value: Option<String<'alloc>>,
}

impl<'alloc> AutoCow<'alloc> {
    fn new(lexer: &Lexer<'alloc>) -> Self {
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
    #[allow(dead_code)]
    fn push_different(&mut self, lexer: &Lexer<'alloc>, c: char) {
        self.get_mut_string(lexer).push(c);
    }

    // Force allocation of a String and return the reference to it
    fn get_mut_string<'b>(&'b mut self, lexer: &'_ Lexer<'alloc>) -> &'b mut String<'alloc> {
        if self.value.is_none() {
            self.value = Some(String::from_str_in(
                &self.start[..self.start.len() - lexer.chars.as_str().len()],
                lexer.allocator,
            ));
        }
        self.value.as_mut().unwrap()
    }

    fn finish(&mut self, lexer: &Lexer<'alloc>) -> &'alloc str {
        match self.value.take() {
            Some(arena_string) => arena_string.into_bump_str(),
            None => &self.start[..self.start.len() - lexer.chars.as_str().len()],
        }
    }
}
