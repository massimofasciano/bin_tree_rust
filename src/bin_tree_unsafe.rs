use crate::BinTree;

impl<Item: PartialEq> BinTree<Item> {
    /// swap 2 items in the tree (lookup by value)
    pub fn swap(&mut self, value1 : &Item, value2 : &Item) -> bool {
        let opt1 = self.get_mut(value1);
        if opt1.is_none() { return false }
        let ptr1 = opt1.unwrap() as * mut Item;
        let opt2 = self.get_mut(value2);
        if opt2.is_none() { return false }
        let ptr2 = opt2.unwrap() as * mut Item;
        let mut1;
        let mut2;
        if ptr1 != ptr2 {
            unsafe {
                mut1 = &mut *ptr1;
                mut2 = &mut *ptr2;
            }
            std::mem::swap(mut1, mut2);
            true
        } else {
            false
        }
    }
}

/// some tests
#[cfg(test)]
mod test {
    use crate::{BinTree, tree, leaf};

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

        t.swap(&3, &1);
        t.swap(&5, &6);
        t.swap(&0, &6);
        assert_eq!(t.to_string(),"(((1) <= 2) <= 3 => (0 => ((5) <= 6)))");
        *t.get_mut(&0).unwrap() = 4;
        assert_eq!(t.to_vec(),vec![1,2,3,4,5,6]);
    }
}