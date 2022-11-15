/*
 * MIT License
 *
 * Copyright (c) 2022 seenees
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Lexer implementation.

use crate::IndexKey;

/// Token definition.
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
  And,
  Eof,
  Eq,
  Ge,
  Gt,
  If,
  Le,
  LeftParen,
  Lt,
  Number(IndexKey),
  Null,
  Nq,
  Or,
  RightParen,
  Semicolon,
  Undef,
}

/// Single space character.
const WS: char = ' ';

/// Buffer size for lexer input.
const BUF_SIZE: usize = 4;

/// The lexer.
pub struct Lexer {
  /// Input characters.
  input: Vec<char>,
  /// Current position in input.
  position: usize,
}

impl Lexer {
  ///
  pub fn new(input: &str) -> Self {
    Self {
      input: input.chars().collect(),
      position: 0,
    }
  }
  ///
  pub fn get_position(&self) -> usize {
    self.position
  }
  ///
  pub fn trace(&self) {
    print!("{:>4} | {}", self.position, self.input[self.position..].to_vec().iter().collect::<String>());
  }
  ///
  pub fn set_position(&mut self, position: usize) {
    if (0..self.input.len()).contains(&position) {
      self.position = position;
    }
  }
  /// Returns the next token starting from current position.
  pub fn next_token(&mut self) -> Token {
    let chars = self.read_input();
    match chars {
      ['n', 'u', 'l', 'l'] => {
        self.position += 4;
        Token::Null
      }
      ['a', 'n', 'd', _] => {
        self.position += 3;
        Token::And
      }
      ['i', 'f', _, _] => {
        self.position += 2;
        Token::If
      }
      ['o', 'r', _, _] => {
        self.position += 2;
        Token::Or
      }
      ['<', '=', _, _] => {
        self.position += 2;
        Token::Le
      }
      ['>', '=', _, _] => {
        self.position += 2;
        Token::Ge
      }
      ['<', '>', _, _] => {
        self.position += 2;
        Token::Nq
      }
      ['=', _, _, _] => {
        self.position += 1;
        Token::Eq
      }
      ['<', _, _, _] => {
        self.position += 1;
        Token::Lt
      }
      ['>', _, _, _] => {
        self.position += 1;
        Token::Gt
      }
      [';', _, _, _] => {
        self.position += 1;
        Token::Semicolon
      }
      ['(', _, _, _] => {
        self.position += 1;
        Token::LeftParen
      }
      [')', _, _, _] => {
        self.position += 1;
        Token::RightParen
      }
      [ch, _, _, _] if is_non_zero_digit(ch) => {
        let digits = self.consume_digits();
        if let Ok(number) = digits.parse::<IndexKey>() {
          Token::Number(number)
        } else {
          Token::Undef
        }
      }
      [WS, WS, WS, WS] => Token::Eof,
      _ => Token::Undef,
    }
  }
  /// Reads characters from input.
  fn read_input(&mut self) -> [char; BUF_SIZE] {
    self.consume_whitespace();
    let mut buffer: [char; BUF_SIZE] = [WS; BUF_SIZE];
    for (offset, value) in buffer.iter_mut().enumerate() {
      if let Some(ch) = self.char_at(offset) {
        *value = ch
      }
    }
    buffer
  }
  /// Consumes whitespace characters.
  fn consume_whitespace(&mut self) {
    while let Some(ch) = self.char_at(0) {
      if is_whitespace(ch) {
        self.position += 1;
      } else {
        break;
      }
    }
  }
  /// Consumes all digits.
  fn consume_digits(&mut self) -> String {
    let mut digits = "".to_string();
    while let Some(ch) = self.char_at(0) {
      if is_digit(ch) {
        digits.push(ch);
        self.position += 1;
      } else {
        break;
      }
    }
    digits
  }
  /// Returns the character at the current cursor position advanced with specified offset.
  fn char_at(&self, offset: usize) -> Option<char> {
    if self.position + offset < self.input.len() {
      Some(self.input[self.position + offset])
    } else {
      None
    }
  }
}

/// Returns `true` when the specified character is a whitespace character.
fn is_whitespace(ch: char) -> bool {
  matches!(ch, '\u{0009}'..='\u{000D}' | '\u{0020}')
}

/// Returns `true` when the specified character is an ASCII digit.
fn is_digit(ch: char) -> bool {
  ch.is_ascii_digit()
}

/// Returns `true` when the specified character is a non-zero ASCII digit.
fn is_non_zero_digit(ch: char) -> bool {
  ch.is_ascii_digit() && ch != '0'
}

#[cfg(test)]
mod tests {
  use super::*;

  fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens = vec![];
    loop {
      let token = lexer.next_token();

      match token {
        t @ Token::Eof | t @ Token::Undef => {
          tokens.push(t);
          break;
        }
        _ => tokens.push(token),
      }
    }
    tokens
  }

  #[test]
  fn test_0001() {
    assert!((Token::If == Token::If));
    assert!((Token::If != Token::Eq));
  }

  #[test]
  fn test_0002() {
    assert_eq!(
      &[
        Token::If,
        Token::LeftParen,
        Token::Number(1),
        Token::Gt,
        Token::Number(2),
        Token::Semicolon,
        Token::Number(1),
        Token::Semicolon,
        Token::Number(2),
        Token::RightParen,
        Token::Eof
      ],
      tokenize("if ( 1 > 2 ; 1 ; 2 ) ").as_slice()
    );
  }

  #[test]
  fn test_0003() {
    assert_eq!(
      &[
        Token::If,
        Token::LeftParen,
        Token::Number(1),
        Token::Eq,
        Token::Null,
        Token::Semicolon,
        Token::Number(1),
        Token::Semicolon,
        Token::Number(2),
        Token::RightParen,
        Token::Eof
      ],
      tokenize("if(1=null;1;2)").as_slice()
    );
  }

  #[test]
  fn test_0004() {
    assert_eq!(
      &[
        Token::If,
        Token::LeftParen,
        Token::Number(255),
        Token::Eq,
        Token::Null,
        Token::Semicolon,
        Token::Number(1),
        Token::Semicolon,
        Token::Number(2),
        Token::RightParen,
        Token::Eof
      ],
      tokenize("if(255=null;1;2)").as_slice()
    );
  }

  #[test]
  fn test_0005() {
    assert_eq!(&[Token::If, Token::LeftParen, Token::Undef], tokenize("if(256=null;1;2)").as_slice());
  }

  #[test]
  fn test_0006() {
    assert_eq!(&[Token::Undef], tokenize(":").as_slice());
  }

  #[test]
  fn test_0007() {
    assert_eq!(
      &[
        Token::If,
        Token::LeftParen,
        Token::Number(1),
        Token::Lt,
        Token::Number(2),
        Token::Or,
        Token::Number(1),
        Token::Gt,
        Token::Number(3),
        Token::Or,
        Token::Number(1),
        Token::Gt,
        Token::Number(4),
        Token::Semicolon,
        Token::Number(1),
        Token::Semicolon,
        Token::Null,
        Token::RightParen,
        Token::Eof
      ],
      tokenize("if(1<2 or 1>3 or 1>4;1;null)").as_slice()
    );
  }
}
