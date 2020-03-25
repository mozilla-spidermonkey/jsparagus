//! Parse NumericLiteral.

#[derive(Debug)]
pub enum NumericLiteralBase {
    Decimal,
    Binary,
    Octal,
    Hex,
}

// To avoid allocating extra buffer when '_' is present, integer cases are
// handled without Rust standard `parse` function.
fn parse_decimal_int(s: &str) -> f64 {
    debug_assert!(!s.is_empty());

    let src = s.as_bytes();

    let mut result = 0.0;
    for &c in src {
        match c {
            b'0'..=b'9' => {
                let n = c - b'0';
                result = result * 10.0 + n as f64;
            }
            b'_' => {}
            _ => panic!("invalid syntax"),
        }
    }
    result
}

fn parse_binary(s: &str) -> f64 {
    debug_assert!(!s.is_empty());

    let src = s.as_bytes();

    let mut result = 0.0;
    for &c in src {
        match c {
            b'0'..=b'1' => {
                let n = c - b'0';
                result = result * 2.0 + n as f64;
            }
            b'_' => {}
            _ => panic!("invalid syntax"),
        }
    }
    result
}

fn parse_octal(s: &str) -> f64 {
    debug_assert!(!s.is_empty());

    let src = s.as_bytes();

    let mut result = 0.0;
    for &c in src {
        match c {
            b'0'..=b'7' => {
                let n = c - b'0';
                result = result * 8.0 + n as f64;
            }
            b'_' => {}
            _ => panic!("invalid syntax"),
        }
    }
    result
}

fn parse_hex(s: &str) -> f64 {
    debug_assert!(!s.is_empty());

    let src = s.as_bytes();

    let mut result = 0.0;
    for &c in src {
        match c {
            b'0'..=b'9' => {
                let n = c - b'0';
                result = result * 16.0 + n as f64;
            }
            b'A'..=b'F' => {
                let n = c - b'A' + 10;
                result = result * 16.0 + n as f64;
            }
            b'a'..=b'f' => {
                let n = c - b'a' + 10;
                result = result * 16.0 + n as f64;
            }
            b'_' => {}
            _ => panic!("invalid syntax"),
        }
    }
    result
}

/// Parse integer NumericLiteral.
///
/// NonDecimalIntegerLiteral should contain the leading '0x' etc.
///
/// FIXME: LegacyOctalIntegerLiteral is not supported.
pub fn parse_int(s: &str, kind: NumericLiteralBase) -> f64 {
    match kind {
        NumericLiteralBase::Decimal => parse_decimal_int(s),
        NumericLiteralBase::Binary => parse_binary(&s[2..]),
        NumericLiteralBase::Octal => parse_octal(&s[2..]),
        NumericLiteralBase::Hex => parse_hex(&s[2..]),
    }
}

fn parse_float_with_underscore(s: &str) -> f64 {
    let filtered: String = s.chars().filter(|c| *c != '_').collect();

    filtered
        .parse::<f64>()
        .expect("DecimalLiteral should be accepted")
}

/// Parse non-integer NumericLiteral.
pub fn parse_float(s: &str) -> f64 {
    println!("parse_float: {}", s);
    if s.contains('_') {
        return parse_float_with_underscore(s);
    }

    s.parse::<f64>().expect("DecimalLiteral should be accepted")
}
