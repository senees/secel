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

//!

use rust_decimal::Decimal;
use std::fmt;

/// Value definition.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Value {
  /// Value representing a `NULL`.
  Null,
  /// Value representing a boolean.
  Bool(bool),
  /// Value representing a decimal number.
  Number(Decimal),
}

impl fmt::Display for Value {
  /// Implements [Display](std::fmt::Display) for [Value].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Null => write!(f, "Null"),
      Value::Bool(v) => write!(f, "Bool: {}", v),
      Value::Number(v) => write!(f, "Number: {}", v),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rust_decimal::Decimal;

  #[test]
  fn test_display() {
    assert_eq!("Null", format!("{}", Value::Null));
    assert_eq!("Bool: true", format!("{}", Value::Bool(true)));
    assert_eq!("Bool: false", format!("{}", Value::Bool(false)));
    assert_eq!("Number: 1.11", format!("{}", Value::Number(Decimal::new(111, 2))));
  }

  #[test]
  fn test_debug() {
    assert_eq!("Null", format!("{:?}", Value::Null));
    assert_eq!("Bool(true)", format!("{:?}", Value::Bool(true)));
    assert_eq!("Bool(false)", format!("{:?}", Value::Bool(false)));
    let n = Decimal::new(111, 2);
    assert_eq!("Number(1.11)", format!("{:?}", Value::Number(n)));
  }

  #[test]
  #[allow(clippy::clone_on_copy)]
  fn test_comparison() {
    assert!((Value::Null == Value::Null));
    assert!((Value::Bool(true) == Value::Bool(true)));
    assert!((Value::Bool(true) != Value::Bool(false)));
    let n1 = Decimal::new(111, 2);
    let n2 = Decimal::new(222, 2);
    assert!((Value::Number(n1) == Value::Number(n1)));
    assert!((Value::Number(n1) != Value::Number(n2)));
    assert!((Value::Number(n1).clone() != Value::Number(n2).clone()));
  }
}
