use derive_more::Display;

/// implementation of a simple binary tree (with optional balancing)
pub mod bin_tree;
pub use crate::bin_tree::*;

/// a basic ordered set container shows how to encapsulate a type inside another
pub mod ordered_set;
pub use crate::ordered_set::*;

/// map (kv store) implemented over a binary tree
pub mod map;
pub use crate::map::*;

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

