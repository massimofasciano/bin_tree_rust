/// implementation of a simple binary tree (struct and private methods)
pub mod bin_tree;
pub use bin_tree::*;

/// various tools for the binary tree (no access to the internal struct members)
pub mod bin_tree_utils;
pub use bin_tree_utils::*;

/// iterators (owned,ref,mut) over a binary tree with 4 traversal methods
pub mod bin_tree_iter;
pub use bin_tree_iter::*;

/// a formatted tree container shows how to implement custom display behavior
pub mod bin_tree_formatted;
pub use bin_tree_formatted::*;

/// a basic ordered set container shows how to encapsulate a type inside another
pub mod ordered_set;
pub use ordered_set::*;

