use super::common::escape_html;
use std::fmt;
use std::str::Chars;

#[derive(Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_next()
    }
}

impl<'a> fmt::Display for Lexer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<pre>")?;
        for token in self.clone() {
            write!(
                f,
                r#"<span class="{}">{}</span>"#,
                token.kind.to_class(),
                escape_html(&token.text)
            )?;
        }
        write!(f, "</pre>")
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars(),
        }
    }
}

impl<'a> Lexer<'a> {
    fn read_next(&mut self) -> Option<Token> {
        use TokenKind::*;

        let start = self.current_pos();
        let next = self.chars.next();
        if next.is_none() {
            return None;
        }

        let kind = match next.unwrap() {
            '(' | ')' | '[' | ']' | '.' => Delim,
            '\'' | '`' | ',' => Quote,
            '0'..='9' => self.read_number(),
            '-' => self.read_hyphen(),
            '#' => self.read_hash(),
            '"' => self.read_string(),
            ';' => self.read_comment(),
            c if is_symbol_start(c) => self.read_symbol(),
            c if is_whitespace(c) => self.read_whitespace(),
            _ => self.read_unknown(),
        };

        let end = self.current_pos();
        Some(Token::new(kind, &self.input[start..end]))
    }

    fn read_number(&mut self) -> TokenKind {
        self.skip_while(is_digit);
        TokenKind::Number
    }

    fn read_hyphen(&mut self) -> TokenKind {
        if let Some(c) = self.peek_char() {
            if is_digit(c) {
                self.read_number()
            } else {
                self.read_symbol()
            }
        } else {
            TokenKind::Symbol
        }
    }

    fn read_hash(&mut self) -> TokenKind {
        match self.peek_char() {
            Some('t') | Some('f') => {
                self.chars.next();
                TokenKind::Bool
            }
            _ => self.read_unknown(),
        }
    }

    fn read_string(&mut self) -> TokenKind {
        let mut escape_next = false;
        while let Some(c) = self.chars.next() {
            match c {
                '\\' if !escape_next => {
                    escape_next = true;
                    continue;
                }
                '"' if !escape_next => {
                    break;
                }
                _ => {}
            }
            escape_next = false;
        }

        TokenKind::String
    }

    fn read_comment(&mut self) -> TokenKind {
        self.skip_while(|c| match c {
            '\n' | '\r' => false,
            _ => true,
        });
        TokenKind::Comment
    }

    fn read_symbol(&mut self) -> TokenKind {
        self.skip_while(is_symbol_continue);
        TokenKind::Symbol
    }

    fn read_whitespace(&mut self) -> TokenKind {
        self.skip_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn read_unknown(&mut self) -> TokenKind {
        self.skip_while(is_unknown);
        TokenKind::Unknown
    }

    fn skip_while(&mut self, pred: impl Fn(char) -> bool) {
        while let Some(c) = self.peek_char() {
            if !pred(c) {
                break;
            }

            self.chars.next();
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn current_pos(&self) -> usize {
        self.input.len() - self.chars.as_str().len()
    }
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_symbol_start(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' => true,
        '+' | '-' | '*' | '/' | '^' | '=' | '<' | '>' | '?' | '!' | ':' | '$' => true,
        _ => false,
    }
}

fn is_symbol_continue(c: char) -> bool {
    match c {
        c if is_symbol_start(c) => true,
        c if is_digit(c) => true,
        _ => false,
    }
}

fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}

fn is_unknown(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' => false,
        '\'' | '`' | ',' => false,
        '#' => false,
        '"' => false,
        ';' => false,
        c if is_symbol_start(c) => false,
        c if is_symbol_continue(c) => false,
        c if is_digit(c) => false,
        c if is_whitespace(c) => false,
        _ => true,
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    text: String,
}

impl Token {
    fn new(kind: TokenKind, text: impl Into<String>) -> Self {
        Token {
            kind,
            text: text.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Delim,
    Quote,
    Number,
    Bool,
    Symbol,
    String,
    Comment,
    Whitespace,
    Unknown,
}

impl TokenKind {
    fn to_class(&self) -> &'static str {
        use TokenKind::*;

        match self {
            Delim => "delim",
            Quote => "quote",
            Number => "number",
            Bool => "bool",
            Symbol => "symbol",
            String => "string",
            Comment => "comment",
            Whitespace => "whitespace",
            Unknown => "unknown",
        }
    }
}
