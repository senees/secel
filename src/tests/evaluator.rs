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

//! Evaluator tests.

use crate::parser::Parser;
use crate::values::Value;
use crate::{evaluator, IndexKey};
use rust_decimal::Decimal;
use std::collections::HashMap;

fn eq(input: &str, values: &[Value], expected: Value) {
  let node = Parser::new(input).parse().unwrap();
  let evaluator = evaluator::build_evaluator(&node).unwrap();
  let mut results = HashMap::new();
  for (i, value) in values.iter().enumerate() {
    results.insert((i + 1) as IndexKey, *value);
  }
  assert_eq!(expected, evaluator(&results));
}

#[test]
fn test_0001() {
  let p1 = Value::Number(Decimal::new(100, 0));
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(110, 0));
  eq("if(1=2;1;2)", &[p1, p2], p1);
  eq("if(1=2;1;2)", &[p1, p3], p3);
}

#[test]
fn test_0002() {
  let p1 = Value::Number(Decimal::new(100, 0));
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(110, 0));
  eq("if(1<>2;1;2)", &[p1, p3], p1);
  eq("if(1<>2;1;2)", &[p1, p2], p2);
}

#[test]
fn test_0003() {
  let p1 = Value::Number(Decimal::new(100, 0));
  let p2 = Value::Number(Decimal::new(110, 0));
  eq("if(1>2;1;2)", &[p1, p2], p2);
  eq("if(1>2;1;2)", &[p2, p1], p2);
}

#[test]
fn test_0004() {
  let p1 = Value::Number(Decimal::new(100, 0));
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(110, 0));
  eq("if(1>=2;1;2)", &[p1, p3], p3);
  eq("if(1>=2;1;2)", &[p3, p1], p3);
  eq("if(1>=2;1;2)", &[p2, p1], p2);
}

#[test]
fn test_0005() {
  let p1 = Value::Number(Decimal::new(100, 0));
  let p2 = Value::Number(Decimal::new(110, 0));
  eq("if(1<2;1;2)", &[p1, p2], p1);
  eq("if(1<2;1;2)", &[p2, p1], p1);
}

#[test]
fn test_0006() {
  let p1 = Value::Number(Decimal::new(100, 0));
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(110, 0));
  eq("if(1<=2;1;2)", &[p3, p1], p1);
  eq("if(1<=2;1;2)", &[p1, p2], p1);
  eq("if(1<=2;1;2)", &[p1, p3], p1);
}

#[test]
fn test_0007() {
  let p1 = Value::Null;
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(110, 0));
  eq("if(1=null;2;1)", &[p1, p2], p2);
  eq("if(null=1;2;1)", &[p1, p2], p2);
  eq("if(1=null;2;1)", &[p2, p3], p2);
  eq("if(null=1;2;1)", &[p2, p3], p2);
  eq("if(null=null;1;2)", &[p2, p3], p2);
}

#[test]
fn test_0008() {
  let p1 = Value::Null;
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(110, 0));
  eq("if(1<>null;1;2)", &[p2, p3], p2);
  eq("if(null<>1;1;2)", &[p2, p3], p2);
  eq("if(1<>null;1;2)", &[p1, p3], p3);
  eq("if(null<>1;1;2)", &[p1, p3], p3);
  eq("if(null<>null;1;2)", &[p2, p3], p3);
}

#[test]
fn test_0009() {
  let p1 = Value::Number(Decimal::new(101, 0));
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(201, 0));
  let p4 = Value::Number(Decimal::new(200, 0));
  let p5 = Value::Number(Decimal::new(512, 0));
  eq("if(1>2 and 3>4;5;1)", &[p1, p2, p3, p4, p5], p5);
  eq("if(1>2 and 3>4;5;2)", &[p2, p1, p3, p4, p5], p1);
  eq("if(1>2 and 3>4;5;3)", &[p1, p2, p4, p3, p5], p4);
}

#[test]
fn test_0010() {
  let p1 = Value::Number(Decimal::new(101, 0));
  let p2 = Value::Number(Decimal::new(100, 0));
  let p3 = Value::Number(Decimal::new(201, 0));
  let p4 = Value::Number(Decimal::new(200, 0));
  let p5 = Value::Number(Decimal::new(512, 0));
  eq("if(1>2 or 3>4;5;1)", &[p1, p2, p3, p4, p5], p5);
  eq("if(1>2 or 3>4;5;2)", &[p2, p1, p3, p4, p5], p5);
  eq("if(1>2 or 3>4;5;3)", &[p2, p1, p4, p3, p5], p4);
}
