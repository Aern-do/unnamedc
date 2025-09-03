pub mod cursor;
pub mod error;
pub mod token;

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

    pub fn token(&mut self) -> Result<Token<'s>, Error> {
        if self.is_number_start()? {
            self.number()
        } else {
            self.cursor.next_char()?;
            self.cursor.consume();
            Err(Error::invalid_token(self.cursor.current_span()?))
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
