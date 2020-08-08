//! Syntax tree implementation for cell.
//!
//! The syntax tree uses [rowan] to create ast nodes
//! or tokens and thus has the following properties:
//!
//! - Loseless syntax tree
//! - Transformations from token to ast is extremly cheap
//! - Intuitive tree traversal

pub mod ast;
