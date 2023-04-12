use derive_more::Display;

/// implementation of a simple binary tree (struct and private methods)
pub mod bin_tree_types;
pub use crate::bin_tree_types::*;

/// various tools for the binary tree
pub mod bin_tree_utils;
pub use crate::bin_tree_utils::*;

/// tools for the binary tree that use bits of unsafe code
pub mod bin_tree_unsafe;
pub use crate::bin_tree_unsafe::*;

/// iterators (owned,ref,mut) over a binary tree with 4 traversal methods
pub mod bin_tree_iter;
pub use crate::bin_tree_iter::*;

/// a formatted tree container shows how to implement custom display behavior
pub mod bin_tree_formatted;
pub use crate::bin_tree_formatted::*;

/// a basic ordered set container shows how to encapsulate a type inside another
pub mod bin_tree_ordered_set;
pub use crate::bin_tree_ordered_set::*;

/// map (kv store) implemented over a binary tree
pub mod bin_tree_map;
pub use crate::bin_tree_map::*;

/// map (kv store) implemented over a binary tree (methods that use bits of unsafe code)
pub mod bin_tree_map_unsafe;
pub use crate::bin_tree_map_unsafe::*;

#[macro_export]
/// split a non-empty tree into the 3 components (value, left, right): &mut version
macro_rules! let_node_ref_mut {
    ($tree:expr => $value:ident , $left:ident, $right:ident) => {
        let crate::BinTreeNode{value:$value,left:$left,right:$right} = $tree.root.as_deref_mut().expect("tree should not be empty");
    };
}

#[macro_export]
/// split a non-empty tree into the 3 components (value, left, right): & version
macro_rules! let_node_ref {
    ($tree:expr => $value:ident , $left:ident, $right:ident) => {
        let crate::BinTreeNode{value:$value,left:$left,right:$right} = $tree.root.as_deref().expect("tree should not be empty");
    };
}

#[macro_export]
/// split a non-empty tree into the 3 components (value, left, right): move version
macro_rules! let_node_move {
    ($tree:expr => $value:ident , $left:ident, $right:ident) => {
        let crate::BinTreeNode{value:$value,left:$left,right:$right} = *($tree.root).expect("tree should not be empty");
    };
}

pub type Result<T> = std::result::Result<T, BinTreeError>;

#[derive(Debug,Display,PartialEq)]
pub enum BinTreeError {
    SwapSame,
    SwapNotFound1,
    SwapNotFound2,
}

impl std::error::Error for BinTreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use BinTreeError::*;
        match *self {
            SwapNotFound1 => None,
            SwapNotFound2 => None,
            SwapSame => None,
        }
    }
}

/// import this to use the macros in other crates
pub mod macros {
    pub use let_node_move;
    pub use let_node_ref;
    pub use let_node_ref_mut;
    pub use crate::BinTreeNode;
}
