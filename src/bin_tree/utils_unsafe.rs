use crate::{BinTree, Result, BinTreeError};

impl<Item: PartialEq> BinTree<Item> {
    /// swap 2 items in the tree (lookup by value)
    pub fn swap(&mut self, value1 : &Item, value2 : &Item) -> Result<()> {
        let opt1 = self.get_mut(value1);
        if opt1.is_none() { return Err(BinTreeError::SwapNotFound1) }
        let ptr1 = opt1.unwrap() as * mut Item;
        let opt2 = self.get_mut(value2);
        if opt2.is_none() { return Err(BinTreeError::SwapNotFound2) }
        let ptr2 = opt2.unwrap() as * mut Item;
        if ptr1 != ptr2 {
            unsafe {
                std::ptr::swap(ptr1,ptr2);
            }
            Ok(())
        } else {
            Err(BinTreeError::SwapSame)
        }
    }
}

