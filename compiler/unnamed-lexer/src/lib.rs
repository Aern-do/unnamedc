pub mod cursor;
pub mod error;
pub mod token;

use unicode_xid::UnicodeXID;
use unnamed_common::Source;

pub use cursor::Cursor;
pub use error::{Error, ErrorKind};
pub use token::{Base, Token, TokenData, TokenKind};

#[derive(Debug, Clone)]
pub struct Lexer<'s> {
    cursor: Cursor<'s>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: Source<'s>) -> Self {
        Self {
            cursor: Cursor::new(source),
        }
    }

    pub fn is_number_start(&mut self) -> Result<bool, Error> {
        let char = self.cursor.peek()?;

        Ok(char.is_ascii_digit())
    }

    pub fn is_number(&mut self, base: Base) -> Result<bool, Error> {
        let char = self.cursor.peek()?;

        Ok(base.is_valid_digit(char) || char == '_')
    }

    pub fn is_string_start(&mut self) -> Result<bool, Error> {
        let char = self.cursor.peek()?;

        Ok(char == '"')
    }

    pub fn is_ident_start(&mut self) -> Result<bool, Error> {
        let char = self.cursor.peek()?;

        Ok(char.is_xid_start() || char == '_')
    }

    pub fn is_ident_continue(&mut self) -> Result<bool, Error> {
        let char = self.cursor.peek()?;

        Ok(char.is_xid_continue())
    }

    pub fn skip_whitespaces(&mut self) -> Result<(), Error> {
        while !self.cursor.is_eof() && self.cursor.peek()?.is_whitespace() {
            self.cursor.next_char()?;
        }
        self.cursor.consume();

        Ok(())
    }

    pub fn parse_number(slice: &'s str, base: Base) -> u64 {
        slice
            .chars()
            .filter(|char| *char != '_')
            .map(|digit| digit.to_digit(base.radix()).expect("invalid digit") as u64)
            .reduce(|acc, digit| acc * (base.radix() as u64) + digit)
            .unwrap_or_default()
    }

    pub fn number(&mut self) -> Result<Token<'s>, Error> {
        let (base_span, base) = if self.cursor.peek()? == '0' {
            self.cursor.next_char()?;

            let base = match self.cursor.peek()? {
                'x' | 'X' => Base::Hexadecimal,
                'o' | 'O' => Base::Octal,
                'b' | 'B' => Base::Binary,
                _ => {
                    let (slice, span) = self.cursor.consume();
                    return Ok(
                        Token::new(span, slice, TokenKind::Int).with_data(TokenData::Number(0))
                    );
                }
            };
            self.cursor.next_char()?;
            let (.., span) = self.cursor.consume();

            (span, base)
        } else {
            (self.cursor.span(), Base::Decimal)
        };

        while !self.cursor.is_eof() && self.is_number(base)? {
            self.cursor.next_char()?;
        }

        let (slice, span) = self.cursor.consume();
        let span = base_span + span;

        let value = Self::parse_number(slice, base);

        Ok(Token::new(span, self.cursor.slice_at(span), TokenKind::Int)
            .with_data(TokenData::Number(value)))
    }

    pub fn string(&mut self) -> Result<Token<'s>, Error> {
        self.cursor.next_char()?;
        let left_delimiter_span = self.cursor.span();

        let mut content = String::new();
        while !self.cursor.is_eof() && self.cursor.peek()? != '"' {
            let char = self.cursor.next_char()?;

            if char == '\\' {
                match self.cursor.next_char()? {
                    'n' => content.push('\n'),
                    't' => content.push('\t'),
                    '\\' => content.push('\\'),
                    '"' => content.push('"'),
                    _ => return Err(Error::invalid_escape_sequence(self.cursor.current_span()?)),
                }
            } else {
                content.push(char);
            }
        }

        self.cursor
            .next_char()
            .map_err(|_| Error::unclosed_string_literal(left_delimiter_span))?;

        let (slice, span) = self.cursor.consume();

        Ok(Token::new(span, slice, TokenKind::Str).with_data(TokenData::String(content)))
    }

    pub fn ident(&mut self) -> Result<Token<'s>, Error> {
        self.cursor.next_char()?;
        while !self.cursor.is_eof() && self.is_ident_continue()? {
            self.cursor.next_char()?;
        }

        let (slice, span) = self.cursor.consume();

        let keyword = match slice {
            "func" => TokenKind::FuncKw,
            "trait" => TokenKind::TraitKw,
            "impl" => TokenKind::ImplKw,
            "while" => TokenKind::WhileKw,
            "let" => TokenKind::LetKw,
            "if" => TokenKind::IfKw,
            "else" => TokenKind::ElseKw,
            "return" => TokenKind::ReturnKw,
            _ => return Ok(Token::new(span, slice, TokenKind::Ident)),
        };

        Ok(Token::new(span, slice, keyword))
    }

    pub fn operator(&mut self) -> Result<Token<'s>, Error> {
        let (kind, chars) = match (self.cursor.peek()?, self.cursor.lookahead(1)) {
            ('+', _) => (TokenKind::Add, 1),
            ('-', _) => (TokenKind::Sub, 1),
            ('*', _) => (TokenKind::Mul, 1),
            ('/', _) => (TokenKind::Div, 1),

            ('=', Some('=')) => (TokenKind::Eq, 2),
            ('=', _) => (TokenKind::Asgmt, 1),

            ('!', Some('=')) => (TokenKind::Neq, 2),

            ('<', Some('=')) => (TokenKind::LtEq, 2),
            ('<', Some('<')) => (TokenKind::BitShl, 2),
            ('<', _) => (TokenKind::Lt, 1),

            ('>', Some('=')) => (TokenKind::GtEq, 2),
            ('>', Some('>')) => (TokenKind::BitShr, 2),
            ('>', _) => (TokenKind::Gt, 1),

            ('&', Some('&')) => (TokenKind::And, 2),
            ('|', Some('|')) => (TokenKind::Or, 2),

            ('|', _) => (TokenKind::BitOr, 1),
            ('&', _) => (TokenKind::BitAnd, 1),
            ('^', _) => (TokenKind::BitXor, 1),

            (',', _) => (TokenKind::Comma, 1),
            ('(', _) => (TokenKind::LeftParen, 1),
            (')', _) => (TokenKind::RightParen, 1),
            ('{', _) => (TokenKind::LeftBrace, 1),
            ('}', _) => (TokenKind::RightBrace, 1),
            ('[', _) => (TokenKind::LeftSquare, 1),
            (']', _) => (TokenKind::RightSquare, 1),

            (';', _) => (TokenKind::Semicolon, 1),
            (':', Some(':')) => (TokenKind::DoubleColon, 2),
            (':', _) => (TokenKind::Colon, 1),

            _ => {
                self.cursor.next_char()?;
                let (.., span) = self.cursor.consume();

                return Err(Error::invalid_token(span));
            }
        };

        self.cursor.skip(chars)?;
        let (slice, span) = self.cursor.consume();

        Ok(Token::new(span, slice, kind))
    }

    pub fn token(&mut self) -> Result<Token<'s>, Error> {
        if self.is_number_start()? {
            self.number()
        } else if self.is_string_start()? {
            self.string()
        } else if self.is_ident_start()? {
            self.ident()
        } else {
            self.operator()
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token<'s>>, Error> {
        self.skip_whitespaces()?;

        if self.cursor.is_eof() {
            Ok(None)
        } else {
            self.token().map(Some)
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Result<Token<'s>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().transpose()
    }
}
