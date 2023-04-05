/// implementation of a simple binary tree with various useful methods
/// to test external iteration of a recursive structure in Rust
/// 
/// a formatted tree container shows how to implement custom display behavior
pub mod bin_tree;
pub use bin_tree::*;

/// iterators (owned,ref,mut) over a binary tree using a deque
/// with 4 traversal methods
pub mod iter_deque;
pub use iter_deque::*;

/// a basic ordered set container shows how to encapsulate a type inside another
pub mod ordered_set;
pub use ordered_set::*;
