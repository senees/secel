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

extern crate ascii_tree;
extern crate difference;
extern crate rust_decimal;

mod ast;
mod errors;
mod evaluator;
mod lexer;
mod parser;
mod values;

#[cfg(test)]
mod tests;

pub use ast::AstNode;
pub use evaluator::{Evaluator, IndexKey, IndexedValues};
pub use values::Value;

/// Parses expression, panics on failure.
pub fn parse_expression(input: &str) -> AstNode {
  parser::Parser::new(input).parse().unwrap()
}

/// Builds evaluator, panics on failure.
pub fn build_evaluator(input: &str) -> Evaluator {
  let node = parser::Parser::new(input).parse().unwrap();
  evaluator::build_evaluator(&node).unwrap()
}
