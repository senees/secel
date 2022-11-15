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

//! Abstract syntax tree implementation.

use crate::IndexKey;
use ascii_tree::{write_tree, Tree};

/// Node of the abstract syntax tree.
#[derive(Debug)]
pub enum AstNode {
  /// Node representing conjunction operator `and`.
  And(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
  /// Node representing comparison operator `=`.
  Eq(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
  /// Node representing comparison operator `>`.
  Ge(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
  /// Node representing comparison operator `>=`.
  Gt(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
  /// Node representing `if` expression.
  If(
    /// Node representing condition expression.
    Box<AstNode>,
    /// Node representing expression invoked when the condition is true.
    Box<AstNode>,
    /// Node representing expression invoked when the condition is false.
    Box<AstNode>,
  ),
  /// Node representing comparison operator `<`.
  Le(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
  /// Node representing comparison operator `<=`.
  Lt(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
  /// Node representing disjunction operator `or`.
  Or(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
  /// Node representing `null` value.
  Null,
  /// Node representing result index.
  Number(IndexKey),
  /// Node representing comparison operator `<>`.
  Nq(
    /// Node representing left-side operand.
    Box<AstNode>,
    /// Node representing right-side operand.
    Box<AstNode>,
  ),
}

impl ToString for AstNode {
  /// Converts [AstNode] into string (ascii tree).
  fn to_string(&self) -> String {
    ast_to_tree(self)
  }
}

/// Converts AST into textual tree.
pub fn ast_to_tree(root: &AstNode) -> String {
  let mut ascii_tree = String::new();
  let tree = ast_node_to_tree(root);
  let _ = write_tree(&mut ascii_tree, &tree);
  let tree = ascii_tree.lines().map(|line| format!("\n      {}", line)).collect::<String>();
  format!("{}\n    ", tree)
}

/// Converts single AST node into tree.
fn ast_node_to_tree(node: &AstNode) -> Tree {
  match node {
    AstNode::And(lhs, rhs) => node_2("And", lhs, rhs),
    AstNode::Eq(lhs, rhs) => node_2("Eq", lhs, rhs),
    AstNode::Ge(lhs, rhs) => node_2("Ge", lhs, rhs),
    AstNode::Gt(lhs, rhs) => node_2("Gt", lhs, rhs),
    AstNode::If(lhs, mid, rhs) => node_3("If", lhs, mid, rhs),
    AstNode::Le(lhs, rhs) => node_2("Le", lhs, rhs),
    AstNode::Lt(lhs, rhs) => node_2("Lt", lhs, rhs),
    AstNode::Null => leaf("Null"),
    AstNode::Number(lhs) => node_and_leaf("Number", &format!("`{}`", lhs)),
    AstNode::Or(lhs, rhs) => node_2("Or", lhs, rhs),
    AstNode::Nq(lhs, rhs) => node_2("Nq", lhs, rhs),
  }
}

///
fn node_2(name: &str, lhs: &AstNode, rhs: &AstNode) -> Tree {
  Tree::Node(name.to_string(), vec![ast_node_to_tree(lhs), ast_node_to_tree(rhs)])
}

///
fn node_3(name: &str, lhs: &AstNode, mid: &AstNode, rhs: &AstNode) -> Tree {
  Tree::Node(name.to_string(), vec![ast_node_to_tree(lhs), ast_node_to_tree(mid), ast_node_to_tree(rhs)])
}

///
fn node_and_leaf(name: &str, leaf: &str) -> Tree {
  Tree::Node(name.to_string(), vec![Tree::Leaf(vec![leaf.to_string()])])
}

///
fn leaf(leaf: &str) -> Tree {
  Tree::Leaf(vec![leaf.to_string()])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_string() {
    assert_eq!(
      r#"
       Null
    "#,
      AstNode::Null.to_string()
    )
  }
}
