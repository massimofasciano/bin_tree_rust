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

/// some tests
#[cfg(test)]
mod test {
    use crate::{BinTree, tree, leaf, BinTreeError};

    fn test_tree() -> BinTree<i32> {
        tree(1,
            tree(2,
                leaf(3),
                ()
            ),
            tree(4,
                (),
                tree(5,
                    leaf(6),
                    ()
        )))
    }

    #[test]
    fn take_replace_swap_test() {
        let mut t = test_tree();

        let value1 = std::mem::take(t.get_mut(&5).unwrap());
        assert_eq!(value1,5);
        assert_eq!(t.to_string(),"(((3) <= 2) <= 1 => (4 => ((6) <= 0)))");

        let value2 = std::mem::replace(t.get_mut(&4).unwrap(),value1);
        assert_eq!(value2,4);
        assert_eq!(t.to_string(),"(((3) <= 2) <= 1 => (5 => ((6) <= 0)))");

        assert_eq!(t.swap(&3, &1).is_ok(),true);
        assert_eq!(t.swap(&5, &6).is_ok(),true);
        assert_eq!(t.swap(&0, &6).is_ok(),true);
        assert_eq!(t.swap(&1, &1),Err(BinTreeError::SwapSame));
        assert_eq!(t.swap(&1, &10),Err(BinTreeError::SwapNotFound2));
        assert_eq!(t.swap(&10, &1),Err(BinTreeError::SwapNotFound1));
        assert_eq!(t.to_string(),"(((1) <= 2) <= 3 => (0 => ((5) <= 6)))");
        *t.get_mut(&0).unwrap() = 4;
        assert_eq!(t.to_vec(),vec![1,2,3,4,5,6]);
    }
}