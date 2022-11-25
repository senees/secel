/*
 * MIT License
 *
 * Copyright (c) 2022 senees
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

//! Simply Enough Condition Expression Language (SECEL)
//!
//! Grammar:
//!
//! ```text
//!       statement = if_expression
//!                 ;
//!
//!   if_expression = `if` `(` condition `;` expression `;` expression `)`
//!                 ;
//!
//!       condition = disjunction { `or` disjunction }
//!                 ;
//!
//!     disjunction = conjunction { `and` conjunction }
//!                 ;
//!    
//!     conjunction = `(` condition `)`
//!                 | comparison
//!                 ;
//!
//!      comparison = value (`=` | `<>` | `>` | `<` | `>=` | `<=`) value
//!                 ;
//!
//!      expression = value
//!                 | if_expression
//!                 ;
//!
//!           value = NUMBER
//!                 | NULL
//!                 ;
//! ```

use crate::ast::AstNode;
use crate::errors::{Result, SecelError};
use crate::lexer::{Lexer, Token};

pub struct Parser {
  lexer: Lexer,
  trace: bool,
}

impl Parser {
  ///
  pub fn new(input: &str) -> Self {
    Self {
      lexer: Lexer::new(input),
      trace: false,
    }
  }
  ///
  pub fn parse(&mut self) -> Result<AstNode> {
    self.parse_statement()
  }
  ///
  fn parse_statement(&mut self) -> Result<AstNode> {
    self.trace("statement");
    self.parse_if_expression()
    // TODO make sure the input is empty (EOF)
  }
  ///
  fn parse_if_expression(&mut self) -> Result<AstNode> {
    self.trace("if-expression");
    self.consume_token(Token::If)?;
    self.consume_token(Token::LeftParen)?;
    let comparison = self.parse_condition()?;
    self.consume_token(Token::Semicolon)?;
    let left_op = self.parse_expression()?;
    self.consume_token(Token::Semicolon)?;
    let right_op = self.parse_expression()?;
    self.consume_token(Token::RightParen)?;
    Ok(AstNode::If(Box::new(comparison), Box::new(left_op), Box::new(right_op)))
  }
  ///
  fn parse_condition(&mut self) -> Result<AstNode> {
    self.trace("condition");
    let mut left_node = self.parse_disjunction()?;
    let position = self.lexer.get_position();
    let mut consumed_or = false;
    while self.consume_token(Token::Or).is_ok() {
      consumed_or = true;
      let right_node = self.parse_disjunction()?;
      left_node = AstNode::Or(Box::new(left_node), Box::new(right_node));
    }
    if consumed_or {
      return Ok(left_node);
    }
    self.lexer.set_position(position);
    Ok(left_node)
  }
  ///
  fn parse_disjunction(&mut self) -> Result<AstNode> {
    self.trace("disjunction");
    let mut left_node = self.parse_conjunction()?;
    let position = self.lexer.get_position();
    let mut consumed_and = false;
    while self.consume_token(Token::And).is_ok() {
      consumed_and = true;
      let right_node = self.parse_conjunction()?;
      left_node = AstNode::And(Box::new(left_node), Box::new(right_node));
    }
    if consumed_and {
      return Ok(left_node);
    }
    self.lexer.set_position(position);
    Ok(left_node)
  }
  ///
  fn parse_conjunction(&mut self) -> Result<AstNode> {
    self.trace("conjunction");
    let position = self.lexer.get_position();
    if let result @ Ok(_) = self.parse_comparison() {
      return result;
    }
    self.lexer.set_position(position);
    self.consume_token(Token::LeftParen)?;
    let node = self.parse_condition()?;
    self.consume_token(Token::RightParen)?;
    Ok(node)
  }
  ///
  fn parse_comparison(&mut self) -> Result<AstNode> {
    self.trace("comparison");
    let left_op = self.parse_value()?;
    let comparison_token = self.lexer.next_token();
    let right_op = self.parse_value()?;
    match comparison_token {
      Token::Eq => Ok(AstNode::Eq(Box::new(left_op), Box::new(right_op))),
      Token::Nq => Ok(AstNode::Nq(Box::new(left_op), Box::new(right_op))),
      Token::Ge => Ok(AstNode::Ge(Box::new(left_op), Box::new(right_op))),
      Token::Gt => Ok(AstNode::Gt(Box::new(left_op), Box::new(right_op))),
      Token::Le => Ok(AstNode::Le(Box::new(left_op), Box::new(right_op))),
      Token::Lt => Ok(AstNode::Lt(Box::new(left_op), Box::new(right_op))),
      other => Err(SecelError::new(&format!("expected comparison token, but encountered {:?}", other))),
    }
  }
  ///
  fn parse_expression(&mut self) -> Result<AstNode> {
    self.trace("expression");
    let position = self.lexer.get_position();
    if let result @ Ok(_) = self.parse_value() {
      return result;
    }
    self.lexer.set_position(position);
    if let result @ Ok(_) = self.parse_if_expression() {
      return result;
    }
    self.lexer.set_position(position);
    Err(SecelError::new("expected 'value' or 'if expression`"))
  }
  ///
  fn parse_value(&mut self) -> Result<AstNode> {
    self.trace("value");
    let position = self.lexer.get_position();
    match self.lexer.next_token() {
      Token::Null => Ok(AstNode::Null),
      Token::Number(n) => Ok(AstNode::Number(n)),
      other => {
        self.lexer.set_position(position);
        Err(SecelError::new(&format!("expected null or number but encountered {:?}", other)))
      }
    }
  }
  ///
  fn consume_token(&mut self, expected: Token) -> Result<()> {
    let position = self.lexer.get_position();
    let token = self.lexer.next_token();
    if token == expected {
      Ok(())
    } else {
      self.lexer.set_position(position);
      Err(SecelError::new(&format!("expected token '{:?}', actual token: '{:?}'", expected, token)))
    }
  }
  ///
  fn trace(&self, name: &str) {
    if self.trace {
      print!("{:14}", name);
      self.lexer.trace();
      println!();
    }
  }
}
