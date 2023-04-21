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

impl<Item> BinTree<Item> {
    /// splits a node into value, left and right (by ref mut)
    pub fn node_mut(&mut self) -> Option<(&mut Item, &mut BinTree<Item>, &mut BinTree<Item>)> {
        if self.is_empty() {
            None
        } else {
            let value_mut: &mut Item;
            let left_mut: &mut BinTree<Item>;
            let right_mut: &mut BinTree<Item>;
            unsafe {
                value_mut = &mut *(self.value_mut().expect("not empty") as *mut _);
                left_mut = &mut *(self.left_mut().expect("not empty") as *mut _);
                right_mut = &mut *(self.right_mut().expect("not empty") as *mut _);
            }
            Some((value_mut,left_mut,right_mut))
        }
    }
}

