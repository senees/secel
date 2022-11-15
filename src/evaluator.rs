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

//! Evaluator implementation.

use crate::ast::AstNode;
use crate::errors::Result;
use crate::values::Value;
use std::collections::HashMap;

/// Type alias for the key that indexes values.
pub type IndexKey = u8;

/// Type alias for the index of values.
pub type IndexedValues = HashMap<IndexKey, Value>;

/// Type alias for the evaluator of the expression.
pub type Evaluator = Box<dyn Fn(&IndexedValues) -> Value>;

/// Builds an expression evaluator for given [AstNode].
pub fn build_evaluator(node: &AstNode) -> Result<Evaluator> {
  match node {
    AstNode::And(lhs, rhs) => build_and(lhs, rhs),
    AstNode::Eq(lhs, rhs) => build_eq(lhs, rhs),
    AstNode::Ge(lhs, rhs) => build_ge(lhs, rhs),
    AstNode::Gt(lhs, rhs) => build_gt(lhs, rhs),
    AstNode::If(mhs, lhs, rhs) => build_if(mhs, lhs, rhs),
    AstNode::Le(lhs, rhs) => build_le(lhs, rhs),
    AstNode::Lt(lhs, rhs) => build_lt(lhs, rhs),
    AstNode::Nq(lhs, rhs) => build_nq(lhs, rhs),
    AstNode::Null => build_null(),
    AstNode::Number(mhs) => build_number(*mhs),
    AstNode::Or(lhs, rhs) => build_or(lhs, rhs),
  }
}

/// Builds an evaluator for `and` operator.
fn build_and(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| {
    if let Value::Bool(lhv) = lhe(iv) {
      if let Value::Bool(rhv) = rhe(iv) {
        return Value::Bool(lhv && rhv);
      }
    }
    Value::Null
  }))
}

/// Builds an evaluator for `=` operator.
fn build_eq(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| match lhe(iv) {
    Value::Number(lhv) => match rhe(iv) {
      Value::Number(rhv) => Value::Bool(lhv == rhv),
      Value::Null => Value::Bool(false),
      _ => Value::Null,
    },
    Value::Null => match rhe(iv) {
      Value::Number(_) => Value::Bool(false),
      Value::Null => Value::Bool(true),
      _ => Value::Null,
    },
    _ => Value::Null,
  }))
}

/// Builds an evaluator for `>` operator.
fn build_ge(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| {
    if let Value::Number(lhv) = lhe(iv) {
      if let Value::Number(rhv) = rhe(iv) {
        return Value::Bool(lhv >= rhv);
      }
    }
    Value::Null
  }))
}

/// Builds an evaluator for `>=` operator.
fn build_gt(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| {
    if let Value::Number(lhv) = lhe(iv) {
      if let Value::Number(rhv) = rhe(iv) {
        return Value::Bool(lhv > rhv);
      }
    }
    Value::Null
  }))
}

/// Builds an evaluator for `if` expression.
fn build_if(mhs: &AstNode, lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let mhe = build_evaluator(mhs)?;
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| {
    if let Value::Bool(mhv) = mhe(iv) {
      return if mhv { lhe(iv) } else { rhe(iv) };
    }
    Value::Null
  }))
}

/// Builds an evaluator for `<` operator.
fn build_le(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| {
    if let Value::Number(lhv) = lhe(iv) {
      if let Value::Number(rhv) = rhe(iv) {
        return Value::Bool(lhv <= rhv);
      }
    }
    Value::Null
  }))
}

/// Builds an evaluator for `<=` operator.
fn build_lt(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| {
    if let Value::Number(lhv) = lhe(iv) {
      if let Value::Number(rhv) = rhe(iv) {
        return Value::Bool(lhv < rhv);
      }
    }
    Value::Null
  }))
}

/// Builds an evaluator for `<>` operator.
fn build_nq(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| match lhe(iv) {
    Value::Number(lhv) => match rhe(iv) {
      Value::Number(rhv) => Value::Bool(lhv != rhv),
      Value::Null => Value::Bool(true),
      _ => Value::Null,
    },
    Value::Null => match rhe(iv) {
      Value::Number(_) => Value::Bool(true),
      Value::Null => Value::Bool(false),
      _ => Value::Null,
    },
    _ => Value::Null,
  }))
}

/// Builds an evaluator for `or` operator.
fn build_or(lhs: &AstNode, rhs: &AstNode) -> Result<Evaluator> {
  let lhe = build_evaluator(lhs)?;
  let rhe = build_evaluator(rhs)?;
  Ok(Box::new(move |iv: &IndexedValues| {
    if let Value::Bool(lhv) = lhe(iv) {
      if let Value::Bool(rhv) = rhe(iv) {
        return Value::Bool(lhv || rhv);
      }
    }
    Value::Null
  }))
}

/// Builds an evaluator for `Null` node.
fn build_null() -> Result<Evaluator> {
  Ok(Box::new(move |_: &IndexedValues| Value::Null))
}

/// Builds an evaluator for `Number` node.
fn build_number(key: IndexKey) -> Result<Evaluator> {
  Ok(Box::new(
    move |iv: &IndexedValues| if let Some(value) = iv.get(&key) { *value } else { Value::Null },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::IndexedValues;
  use rust_decimal::Decimal;

  #[test]
  fn test_build_if() {
    let mut m = IndexedValues::new();
    m.insert(1, Value::Bool(true));
    m.insert(2, Value::Bool(true));
    let b = build_if(&AstNode::Number(255), &AstNode::Number(1), &AstNode::Number(2)).unwrap();
    assert_eq!(Value::Null, b(&m));
  }

  #[test]
  fn test_build_ge() {
    let mut r = IndexedValues::new();
    r.insert(1, Value::Bool(true));
    r.insert(2, Value::Bool(false));
    let b = build_ge(&AstNode::Number(1), &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_gt() {
    let mut r = IndexedValues::new();
    r.insert(1, Value::Bool(true));
    r.insert(2, Value::Bool(false));
    let b = build_gt(&AstNode::Number(1), &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_le() {
    let mut r = IndexedValues::new();
    r.insert(1, Value::Bool(true));
    r.insert(2, Value::Bool(false));
    let b = build_le(&AstNode::Number(1), &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_lt() {
    let mut r = IndexedValues::new();
    r.insert(1, Value::Bool(true));
    r.insert(2, Value::Bool(false));
    let b = build_lt(&AstNode::Number(1), &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_eq() {
    let mut r = IndexedValues::new();
    r.insert(1, Value::Bool(true));
    r.insert(2, Value::Number(Decimal::new(123, 2)));
    let b = build_eq(&AstNode::Null, &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
    let b = build_eq(&AstNode::Number(2), &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
    let b = build_eq(&AstNode::Number(1), &AstNode::Number(2)).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_nq() {
    let mut r = IndexedValues::new();
    r.insert(1, Value::Bool(true));
    r.insert(2, Value::Number(Decimal::new(123, 2)));
    let b = build_nq(&AstNode::Null, &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
    let b = build_nq(&AstNode::Number(2), &AstNode::Number(1)).unwrap();
    assert_eq!(Value::Null, b(&r));
    let b = build_nq(&AstNode::Number(1), &AstNode::Number(2)).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_and() {
    let r = IndexedValues::new();
    let b = build_and(&AstNode::Null, &AstNode::Null).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_or() {
    let r = IndexedValues::new();
    let b = build_or(&AstNode::Null, &AstNode::Null).unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_null() {
    let r = IndexedValues::new();
    let b = build_null().unwrap();
    assert_eq!(Value::Null, b(&r));
  }

  #[test]
  fn test_build_number() {
    let mut r = IndexedValues::new();
    let b = build_number(1).unwrap();
    assert_eq!(Value::Null, b(&r));
    r.insert(1, Value::Number(Decimal::new(123, 2)));
    assert_eq!(Value::Number(Decimal::new(123, 2)), b(&r));
  }
}
