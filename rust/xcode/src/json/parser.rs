use serde_json::{Map, Number, Value};

use crate::util::{buffer_to_value, JS_MAX_SAFE_INTEGER};

const CHAR_SPACE: u8 = b' ';
const CHAR_TAB: u8 = b'\t';
const CHAR_NEWLINE: u8 = b'\n';
const CHAR_CARRIAGE: u8 = b'\r';
const CHAR_OPEN_BRACE: u8 = b'{';
const CHAR_CLOSE_BRACE: u8 = b'}';
const CHAR_OPEN_PAREN: u8 = b'(';
const CHAR_CLOSE_PAREN: u8 = b')';
const CHAR_SEMICOLON: u8 = b';';
const CHAR_COMMA: u8 = b',';
const CHAR_EQUALS: u8 = b'=';
const CHAR_DOUBLE_QUOTE: u8 = b'"';
const CHAR_SINGLE_QUOTE: u8 = b'\'';
const CHAR_BACKSLASH: u8 = b'\\';
const CHAR_SLASH: u8 = b'/';
const CHAR_ASTERISK: u8 = b'*';
const CHAR_LESS_THAN: u8 = b'<';
const CHAR_GREATER_THAN: u8 = b'>';

pub struct JsonParser<'a> {
    text: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            bytes: text.as_bytes(),
            pos: 0,
        }
    }

    pub fn parse(mut self) -> anyhow::Result<Value> {
        self.skip_whitespace_and_comments();
        let Some(&code) = self.bytes.get(self.pos) else {
            return Err(self.error("Expected '{' or '('"));
        };

        match code {
            CHAR_OPEN_BRACE => Ok(Value::Object(self.parse_object()?)),
            CHAR_OPEN_PAREN => Ok(Value::Array(self.parse_array()?)),
            _ => Err(self.error("Expected '{' or '('")),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.bytes.len() {
            let code = self.bytes[self.pos];

            if is_whitespace(code) {
                self.pos += 1;
                continue;
            }

            if code == CHAR_SLASH {
                let next = self.bytes.get(self.pos + 1).copied();
                if next == Some(CHAR_SLASH) {
                    self.pos += 2;
                    while self.pos < self.bytes.len() && self.bytes[self.pos] != CHAR_NEWLINE {
                        self.pos += 1;
                    }
                    continue;
                }
                if next == Some(CHAR_ASTERISK) {
                    self.pos += 2;
                    while self.pos + 1 < self.bytes.len() {
                        if self.bytes[self.pos] == CHAR_ASTERISK
                            && self.bytes[self.pos + 1] == CHAR_SLASH
                        {
                            self.pos += 2;
                            break;
                        }
                        self.pos += 1;
                    }
                    continue;
                }
            }

            break;
        }
    }

    fn parse_object(&mut self) -> anyhow::Result<Map<String, Value>> {
        self.pos += 1; // {
        let mut obj = Map::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.pos >= self.bytes.len() {
                return Err(self.error("Unexpected end of input in object"));
            }

            let code = self.bytes[self.pos];
            if code == CHAR_CLOSE_BRACE {
                self.pos += 1;
                return Ok(obj);
            }

            let key = self.parse_identifier_as_string()?;
            self.skip_whitespace_and_comments();

            if self.bytes.get(self.pos).copied() != Some(CHAR_EQUALS) {
                return Err(self.error("Expected '='"));
            }
            self.pos += 1;

            self.skip_whitespace_and_comments();

            let value = self.parse_value()?;
            obj.insert(key, value);

            self.skip_whitespace_and_comments();
            if self.bytes.get(self.pos).copied() != Some(CHAR_SEMICOLON) {
                return Err(self.error("Expected ';'"));
            }
            self.pos += 1;
        }
    }

    fn parse_array(&mut self) -> anyhow::Result<Vec<Value>> {
        self.pos += 1; // (
        let mut arr = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            if self.pos >= self.bytes.len() {
                return Err(self.error("Unexpected end of input in array"));
            }

            let code = self.bytes[self.pos];
            if code == CHAR_CLOSE_PAREN {
                self.pos += 1;
                return Ok(arr);
            }

            arr.push(self.parse_value()?);
            self.skip_whitespace_and_comments();

            if self.bytes.get(self.pos).copied() == Some(CHAR_COMMA) {
                self.pos += 1;
            }
        }
    }

    fn parse_value(&mut self) -> anyhow::Result<Value> {
        self.skip_whitespace_and_comments();
        let Some(&code) = self.bytes.get(self.pos) else {
            return Err(self.error("Unexpected end of input while parsing value"));
        };

        match code {
            CHAR_OPEN_BRACE => Ok(Value::Object(self.parse_object()?)),
            CHAR_OPEN_PAREN => Ok(Value::Array(self.parse_array()?)),
            CHAR_LESS_THAN => self.parse_data_literal(),
            CHAR_DOUBLE_QUOTE | CHAR_SINGLE_QUOTE => Ok(Value::String(self.parse_quoted_string()?)),
            _ => self.parse_string_literal(),
        }
    }

    fn parse_identifier_as_string(&mut self) -> anyhow::Result<String> {
        let Some(&code) = self.bytes.get(self.pos) else {
            return Err(self.error("Expected string literal"));
        };

        if code == CHAR_DOUBLE_QUOTE || code == CHAR_SINGLE_QUOTE {
            return self.parse_quoted_string();
        }

        self.parse_string_literal_raw()
    }

    fn parse_string_literal_raw(&mut self) -> anyhow::Result<String> {
        let start = self.pos;
        while self.pos < self.bytes.len() && is_string_char(self.bytes[self.pos]) {
            self.pos += 1;
        }

        if self.pos == start {
            return Err(self.error("Expected string literal"));
        }

        Ok(self.text[start..self.pos].to_string())
    }

    fn parse_string_literal(&mut self) -> anyhow::Result<Value> {
        let literal = self.parse_string_literal_raw()?;
        Ok(parse_type(&literal))
    }

    fn parse_quoted_string(&mut self) -> anyhow::Result<String> {
        let quote = self.bytes[self.pos];
        self.pos += 1;

        let start = self.pos;
        let mut has_escape = false;

        while self.pos < self.bytes.len() {
            let code = self.bytes[self.pos];
            if code == quote {
                if !has_escape {
                    let result = self.text[start..self.pos].to_string();
                    self.pos += 1;
                    return Ok(result);
                }
                break;
            }
            if code == CHAR_BACKSLASH {
                has_escape = true;
                self.pos += 2;
            } else {
                self.pos += 1;
            }
        }

        if self.pos >= self.bytes.len() {
            return Err(self.error("Unterminated string"));
        }

        let raw = self.text[start..self.pos].to_string();
        self.pos += 1;
        self.unescape_string(&raw)
    }

    fn unescape_string(&self, input: &str) -> anyhow::Result<String> {
        let mut out = String::new();
        let bytes = input.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let code = bytes[i];
            if code == CHAR_BACKSLASH && i + 1 < bytes.len() {
                let next = bytes[i + 1];

                if let Some(escaped) = escape_code(next) {
                    out.push(escaped);
                    i += 2;
                    continue;
                }

                if next == b'U' && i + 5 < bytes.len() {
                    let hex = &input[i + 2..i + 6];
                    if is_hex_string(hex) {
                        let cp = u32::from_str_radix(hex, 16)
                            .map_err(|_| self.error("Invalid unicode escape"))?;
                        if let Some(ch) = char::from_u32(cp) {
                            out.push(ch);
                        }
                        i += 6;
                        continue;
                    }
                }

                if is_octal(next) {
                    let mut octal = String::new();
                    let mut j = i + 1;
                    while j < bytes.len() && j < i + 4 && is_octal(bytes[j]) {
                        octal.push(bytes[j] as char);
                        j += 1;
                    }
                    let value = u8::from_str_radix(&octal, 8)
                        .map_err(|_| self.error("Invalid octal escape"))?;
                    let mapped = if value >= 0x80 {
                        next_step_mapping(value)
                    } else {
                        value as u32
                    };
                    if let Some(ch) = char::from_u32(mapped) {
                        out.push(ch);
                    }
                    i = j;
                    continue;
                }

                out.push(bytes[i] as char);
                out.push(bytes[i + 1] as char);
                i += 2;
            } else {
                out.push(bytes[i] as char);
                i += 1;
            }
        }

        Ok(out)
    }

    fn parse_data_literal(&mut self) -> anyhow::Result<Value> {
        self.pos += 1; // <
        let mut hex = String::new();

        while self.pos < self.bytes.len() {
            let code = self.bytes[self.pos];
            if code == CHAR_GREATER_THAN {
                self.pos += 1;
                if !hex.len().is_multiple_of(2) {
                    return Err(
                        self.error("Data literal must contain an even number of hex digits")
                    );
                }
                let bytes =
                    decode_hex(&hex).ok_or_else(|| self.error("Invalid hex data literal"))?;
                return Ok(buffer_to_value(&bytes));
            }

            if is_whitespace(code) {
                self.pos += 1;
                continue;
            }

            if is_hex(code) {
                hex.push(code as char);
                self.pos += 1;
                continue;
            }

            return Err(self.error("Invalid character in data literal"));
        }

        Err(self.error("Unterminated data literal"))
    }

    fn error(&self, message: &str) -> anyhow::Error {
        let mut line = 1usize;
        let mut col = 1usize;
        for &b in &self.bytes[..self.pos.min(self.bytes.len())] {
            if b == CHAR_NEWLINE {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        anyhow::anyhow!("{} at line {}, column {}", message, line, col)
    }
}

pub fn parse(text: &str) -> anyhow::Result<Value> {
    JsonParser::new(text).parse()
}

fn decode_hex(hex: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        let hi = from_hex(bytes[i])?;
        let lo = from_hex(bytes[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Some(out)
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn parse_type(literal: &str) -> Value {
    if literal.is_empty() {
        return Value::String(literal.to_string());
    }

    let first = literal.as_bytes()[0];
    let all_digits = literal.as_bytes().iter().all(|b| b.is_ascii_digit());
    if all_digits {
        if first == b'0' && literal.len() > 1 {
            return Value::String(literal.to_string());
        }
        if let Ok(value) = literal.parse::<i64>() {
            if value.unsigned_abs() <= JS_MAX_SAFE_INTEGER as u64 {
                return Value::Number(Number::from(value));
            }
        }
        return Value::String(literal.to_string());
    }

    let mut has_dot = false;
    let mut is_number = true;
    for (idx, &b) in literal.as_bytes().iter().enumerate() {
        match b {
            b'.' => {
                if has_dot {
                    is_number = false;
                    break;
                }
                has_dot = true;
            }
            b'+' | b'-' => {
                if idx != 0 {
                    is_number = false;
                    break;
                }
            }
            b'0'..=b'9' => {}
            _ => {
                is_number = false;
                break;
            }
        }
    }

    if is_number && has_dot {
        if literal.as_bytes().last() == Some(&b'0') {
            return Value::String(literal.to_string());
        }
        if let Ok(value) = literal.parse::<f64>() {
            if let Some(num) = Number::from_f64(value) {
                return Value::Number(num);
            }
        }
    }

    Value::String(literal.to_string())
}

fn is_whitespace(c: u8) -> bool {
    matches!(c, CHAR_SPACE | CHAR_TAB | CHAR_NEWLINE | CHAR_CARRIAGE)
}

fn is_string_char(c: u8) -> bool {
    matches!(
        c,
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'$' | b'/' | b':' | b'.' | b'-'
    )
}

fn is_hex(c: u8) -> bool {
    c.is_ascii_hexdigit()
}

fn is_octal(c: u8) -> bool {
    matches!(c, b'0'..=b'7')
}

fn is_hex_string(s: &str) -> bool {
    s.len() == 4 && s.as_bytes().iter().all(|b| is_hex(*b))
}

fn escape_code(c: u8) -> Option<char> {
    match c {
        b'a' => Some('\x07'),
        b'b' => Some('\x08'),
        b'f' => Some('\x0c'),
        b'n' => Some('\n'),
        b'r' => Some('\r'),
        b't' => Some('\t'),
        b'v' => Some('\x0b'),
        b'"' => Some('"'),
        b'\'' => Some('\''),
        b'\\' => Some('\\'),
        b'\n' => Some('\n'),
        _ => None,
    }
}

fn next_step_mapping(code: u8) -> u32 {
    match code {
        0x80 => 0x00a0,
        0x81 => 0x00c0,
        0x82 => 0x00c1,
        0x83 => 0x00c2,
        0x84 => 0x00c3,
        0x85 => 0x00c4,
        0x86 => 0x00c5,
        0x87 => 0x00c7,
        0x88 => 0x00c8,
        0x89 => 0x00c9,
        0x8a => 0x00ca,
        0x8b => 0x00cb,
        0x8c => 0x00cc,
        0x8d => 0x00cd,
        0x8e => 0x00ce,
        0x8f => 0x00cf,
        0x90 => 0x00d0,
        0x91 => 0x00d1,
        0x92 => 0x00d2,
        0x93 => 0x00d3,
        0x94 => 0x00d4,
        0x95 => 0x00d5,
        0x96 => 0x00d6,
        0x97 => 0x00d9,
        0x98 => 0x00da,
        0x99 => 0x00db,
        0x9a => 0x00dc,
        0x9b => 0x00dd,
        0x9c => 0x00de,
        0x9d => 0x00b5,
        0x9e => 0x00d7,
        0x9f => 0x00f7,
        0xa0 => 0x00a9,
        0xa1 => 0x00a1,
        0xa2 => 0x00a2,
        0xa3 => 0x00a3,
        0xa4 => 0x2044,
        0xa5 => 0x00a5,
        0xa6 => 0x0192,
        0xa7 => 0x00a7,
        0xa8 => 0x00a4,
        0xa9 => 0x2019,
        0xaa => 0x201c,
        0xab => 0x00ab,
        0xac => 0x2039,
        0xad => 0x203a,
        0xae => 0xfb01,
        0xaf => 0xfb02,
        0xb0 => 0x00ae,
        0xb1 => 0x2013,
        0xb2 => 0x2020,
        0xb3 => 0x2021,
        0xb4 => 0x00b7,
        0xb5 => 0x00a6,
        0xb6 => 0x00b6,
        0xb7 => 0x2022,
        0xb8 => 0x201a,
        0xb9 => 0x201e,
        0xba => 0x201d,
        0xbb => 0x00bb,
        0xbc => 0x2026,
        0xbd => 0x2030,
        0xbe => 0x00ac,
        0xbf => 0x00bf,
        0xc0 => 0x00b9,
        0xc1 => 0x02cb,
        0xc2 => 0x00b4,
        0xc3 => 0x02c6,
        0xc4 => 0x02dc,
        0xc5 => 0x00af,
        0xc6 => 0x02d8,
        0xc7 => 0x02d9,
        0xc8 => 0x00a8,
        0xc9 => 0x00b2,
        0xca => 0x02da,
        0xcb => 0x00b8,
        0xcc => 0x00b3,
        0xcd => 0x02dd,
        0xce => 0x02db,
        0xcf => 0x02c7,
        0xd0 => 0x2014,
        0xd1 => 0x00b1,
        0xd2 => 0x00bc,
        0xd3 => 0x00bd,
        0xd4 => 0x00be,
        0xd5 => 0x00e0,
        0xd6 => 0x00e1,
        0xd7 => 0x00e2,
        0xd8 => 0x00e3,
        0xd9 => 0x00e4,
        0xda => 0x00e5,
        0xdb => 0x00e7,
        0xdc => 0x00e8,
        0xdd => 0x00e9,
        0xde => 0x00ea,
        0xdf => 0x00eb,
        0xe0 => 0x00ec,
        0xe1 => 0x00c6,
        0xe2 => 0x00ed,
        0xe3 => 0x00aa,
        0xe4 => 0x00ee,
        0xe5 => 0x00ef,
        0xe6 => 0x00f0,
        0xe7 => 0x00f1,
        0xe8 => 0x0141,
        0xe9 => 0x00d8,
        0xea => 0x0152,
        0xeb => 0x00ba,
        0xec => 0x00f2,
        0xed => 0x00f3,
        0xee => 0x00f4,
        0xef => 0x00f5,
        0xf0 => 0x00f6,
        0xf1 => 0x00e6,
        0xf2 => 0x00f9,
        0xf3 => 0x00fa,
        0xf4 => 0x00fb,
        0xf5 => 0x0131,
        0xf6 => 0x00fc,
        0xf7 => 0x00fd,
        0xf8 => 0x0142,
        0xf9 => 0x00f8,
        0xfa => 0x0153,
        0xfb => 0x00df,
        0xfc => 0x00fe,
        0xfd => 0x00ff,
        0xfe => 0xfffd,
        0xff => 0xfffd,
        _ => code as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn keeps_numeric_object_keys_as_strings() {
        let input = "{ 123 = abc; 456 = { 789 = def; }; }";
        let result = parse(input).unwrap();
        assert_eq!(result["123"], "abc");
        assert_eq!(result["456"]["789"], "def");
    }
}
