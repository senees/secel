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

//! Parser tests.

use crate::ast::ast_to_tree;
use crate::parser::Parser;
use difference::Changeset;

fn eq(input: &str, expected: &str) {
  let node = Parser::new(input).parse().unwrap();
  let actual = ast_to_tree(&node);
  if actual != expected {
    println!("EXPECTED:\n------------------------------------------------------------{}\n", expected);
    println!("ACTUAL:\n------------------------------------------------------------{}\n", actual);
    println!(
      "DIFF:\n------------------------------------------------------------{}\n",
      Changeset::new(expected, &actual, "")
    );
  }
  assert_eq!(expected, actual);
}

#[test]
fn test_0001() {
  eq(
    "if(1=2;1;2)",
    r#"
       If
       ├─ Eq
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Number
       │     └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0002() {
  eq(
    "if(1<>2;1;2)",
    r#"
       If
       ├─ Nq
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Number
       │     └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0003() {
  eq(
    "if(1>2;1;2)",
    r#"
       If
       ├─ Gt
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Number
       │     └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0004() {
  eq(
    "if(1<2;1;2)",
    r#"
       If
       ├─ Lt
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Number
       │     └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0005() {
  eq(
    "if(1>=2;1;2)",
    r#"
       If
       ├─ Ge
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Number
       │     └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0006() {
  eq(
    "if(1<=2;1;2)",
    r#"
       If
       ├─ Le
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Number
       │     └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0007() {
  eq(
    "if(1=null;1;2)",
    r#"
       If
       ├─ Eq
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Null
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0008() {
  eq(
    "if(1<>null;1;if(3>4;3;4))",
    r#"
       If
       ├─ Nq
       │  ├─ Number
       │  │  └─ `1`
       │  └─ Null
       ├─ Number
       │  └─ `1`
       └─ If
          ├─ Gt
          │  ├─ Number
          │  │  └─ `3`
          │  └─ Number
          │     └─ `4`
          ├─ Number
          │  └─ `3`
          └─ Number
             └─ `4`
    "#,
  );
}

#[test]
fn test_0009() {
  eq(
    "if ( 3 = null or 4 = null; 1; 2)",
    r#"
       If
       ├─ Or
       │  ├─ Eq
       │  │  ├─ Number
       │  │  │  └─ `3`
       │  │  └─ Null
       │  └─ Eq
       │     ├─ Number
       │     │  └─ `4`
       │     └─ Null
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0010() {
  eq(
    "if ( 3 <> null and 4 <> null; 1; 2)",
    r#"
       If
       ├─ And
       │  ├─ Nq
       │  │  ├─ Number
       │  │  │  └─ `3`
       │  │  └─ Null
       │  └─ Nq
       │     ├─ Number
       │     │  └─ `4`
       │     └─ Null
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0011() {
  eq(
    "if ( 1 = 2 or 3 <> null and 4 <> null; 1; 2)",
    r#"
       If
       ├─ Or
       │  ├─ Eq
       │  │  ├─ Number
       │  │  │  └─ `1`
       │  │  └─ Number
       │  │     └─ `2`
       │  └─ And
       │     ├─ Nq
       │     │  ├─ Number
       │     │  │  └─ `3`
       │     │  └─ Null
       │     └─ Nq
       │        ├─ Number
       │        │  └─ `4`
       │        └─ Null
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0012() {
  eq(
    "if ( 3 <> null and 4 <> null or 1 = 2; 1; 2 )",
    r#"
       If
       ├─ Or
       │  ├─ And
       │  │  ├─ Nq
       │  │  │  ├─ Number
       │  │  │  │  └─ `3`
       │  │  │  └─ Null
       │  │  └─ Nq
       │  │     ├─ Number
       │  │     │  └─ `4`
       │  │     └─ Null
       │  └─ Eq
       │     ├─ Number
       │     │  └─ `1`
       │     └─ Number
       │        └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0013() {
  eq(
    "if ( 3 <> null and (4 <> null or 1 = 2); 1; 2 )",
    r#"
       If
       ├─ And
       │  ├─ Nq
       │  │  ├─ Number
       │  │  │  └─ `3`
       │  │  └─ Null
       │  └─ Or
       │     ├─ Nq
       │     │  ├─ Number
       │     │  │  └─ `4`
       │     │  └─ Null
       │     └─ Eq
       │        ├─ Number
       │        │  └─ `1`
       │        └─ Number
       │           └─ `2`
       ├─ Number
       │  └─ `1`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0014() {
  eq(
    "if((3<>null);3;2)",
    r#"
       If
       ├─ Nq
       │  ├─ Number
       │  │  └─ `3`
       │  └─ Null
       ├─ Number
       │  └─ `3`
       └─ Number
          └─ `2`
    "#,
  );
}

#[test]
fn test_0015() {
  eq(
    "if(1<2 or 3>4 or 5>=6;7;null)",
    r#"
       If
       ├─ Or
       │  ├─ Or
       │  │  ├─ Lt
       │  │  │  ├─ Number
       │  │  │  │  └─ `1`
       │  │  │  └─ Number
       │  │  │     └─ `2`
       │  │  └─ Gt
       │  │     ├─ Number
       │  │     │  └─ `3`
       │  │     └─ Number
       │  │        └─ `4`
       │  └─ Ge
       │     ├─ Number
       │     │  └─ `5`
       │     └─ Number
       │        └─ `6`
       ├─ Number
       │  └─ `7`
       └─ Null
    "#,
  );
}

#[test]
fn test_0016() {
  eq(
    "if(1<2 or 3>4 or 5>=6 or 7<=8;9;null)",
    r#"
       If
       ├─ Or
       │  ├─ Or
       │  │  ├─ Or
       │  │  │  ├─ Lt
       │  │  │  │  ├─ Number
       │  │  │  │  │  └─ `1`
       │  │  │  │  └─ Number
       │  │  │  │     └─ `2`
       │  │  │  └─ Gt
       │  │  │     ├─ Number
       │  │  │     │  └─ `3`
       │  │  │     └─ Number
       │  │  │        └─ `4`
       │  │  └─ Ge
       │  │     ├─ Number
       │  │     │  └─ `5`
       │  │     └─ Number
       │  │        └─ `6`
       │  └─ Le
       │     ├─ Number
       │     │  └─ `7`
       │     └─ Number
       │        └─ `8`
       ├─ Number
       │  └─ `9`
       └─ Null
    "#,
  );
}

#[test]
fn test_0017() {
  eq(
    "if(1<2 or 3>4 or 5>=6 or 7<=8 and 9=null;10;null)",
    r#"
       If
       ├─ Or
       │  ├─ Or
       │  │  ├─ Or
       │  │  │  ├─ Lt
       │  │  │  │  ├─ Number
       │  │  │  │  │  └─ `1`
       │  │  │  │  └─ Number
       │  │  │  │     └─ `2`
       │  │  │  └─ Gt
       │  │  │     ├─ Number
       │  │  │     │  └─ `3`
       │  │  │     └─ Number
       │  │  │        └─ `4`
       │  │  └─ Ge
       │  │     ├─ Number
       │  │     │  └─ `5`
       │  │     └─ Number
       │  │        └─ `6`
       │  └─ And
       │     ├─ Le
       │     │  ├─ Number
       │     │  │  └─ `7`
       │     │  └─ Number
       │     │     └─ `8`
       │     └─ Eq
       │        ├─ Number
       │        │  └─ `9`
       │        └─ Null
       ├─ Number
       │  └─ `10`
       └─ Null
    "#,
  );
}

#[test]
fn test_0018() {
  eq(
    "if(1<2 and 3>4 and 5>=6;7;null)",
    r#"
       If
       ├─ And
       │  ├─ And
       │  │  ├─ Lt
       │  │  │  ├─ Number
       │  │  │  │  └─ `1`
       │  │  │  └─ Number
       │  │  │     └─ `2`
       │  │  └─ Gt
       │  │     ├─ Number
       │  │     │  └─ `3`
       │  │     └─ Number
       │  │        └─ `4`
       │  └─ Ge
       │     ├─ Number
       │     │  └─ `5`
       │     └─ Number
       │        └─ `6`
       ├─ Number
       │  └─ `7`
       └─ Null
    "#,
  );
}

#[test]
fn test_0019() {
  eq(
    "if(1<2 and 3>4 and 5>=6 and 7<=8;9;null)",
    r#"
       If
       ├─ And
       │  ├─ And
       │  │  ├─ And
       │  │  │  ├─ Lt
       │  │  │  │  ├─ Number
       │  │  │  │  │  └─ `1`
       │  │  │  │  └─ Number
       │  │  │  │     └─ `2`
       │  │  │  └─ Gt
       │  │  │     ├─ Number
       │  │  │     │  └─ `3`
       │  │  │     └─ Number
       │  │  │        └─ `4`
       │  │  └─ Ge
       │  │     ├─ Number
       │  │     │  └─ `5`
       │  │     └─ Number
       │  │        └─ `6`
       │  └─ Le
       │     ├─ Number
       │     │  └─ `7`
       │     └─ Number
       │        └─ `8`
       ├─ Number
       │  └─ `9`
       └─ Null
    "#,
  );
}

#[test]
fn test_0020() {
  eq(
    "if(1<2 and 3>4 and 5>=6 and 7<=8 or 9<>10;11;null)",
    r#"
       If
       ├─ Or
       │  ├─ And
       │  │  ├─ And
       │  │  │  ├─ And
       │  │  │  │  ├─ Lt
       │  │  │  │  │  ├─ Number
       │  │  │  │  │  │  └─ `1`
       │  │  │  │  │  └─ Number
       │  │  │  │  │     └─ `2`
       │  │  │  │  └─ Gt
       │  │  │  │     ├─ Number
       │  │  │  │     │  └─ `3`
       │  │  │  │     └─ Number
       │  │  │  │        └─ `4`
       │  │  │  └─ Ge
       │  │  │     ├─ Number
       │  │  │     │  └─ `5`
       │  │  │     └─ Number
       │  │  │        └─ `6`
       │  │  └─ Le
       │  │     ├─ Number
       │  │     │  └─ `7`
       │  │     └─ Number
       │  │        └─ `8`
       │  └─ Nq
       │     ├─ Number
       │     │  └─ `9`
       │     └─ Number
       │        └─ `10`
       ├─ Number
       │  └─ `11`
       └─ Null
    "#,
  );
}

#[test]
fn test_e_0001() {
  assert!(Parser::new("if(3 1 null;3;2)").parse().is_err());
}

#[test]
fn test_e_0002() {
  assert!(Parser::new("if(3 <> null;>;2)").parse().is_err());
}
