use crate::{BinTree, let_node_ref_mut};

/// This macro should only used in the specific context of the pop functions in this crate
/// and is not meant as a general purpose tool. It is used to avoid code repetition.
macro_rules! take_value_replace_tree {
    ($dest_tree:expr , $value:expr , $source_tree:expr) => {
        {
            let value_taken = std::mem::take($value);
            let source_tree_taken = std::mem::take($source_tree);
            *$dest_tree = source_tree_taken;
            value_taken
        }
    };
}
// make it pub to this module only for the tests submodule
pub(self) use take_value_replace_tree;

impl<Item: Default> BinTree<Item> {
    /// pop the top item from the tree
    pub fn pop(&mut self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let mut p;
            p = self.left_mut().unwrap().pop();
            if p.is_none() {
                p = self.right_mut().unwrap().pop();
            }
            Some(match p {
                None => {
                    take_value_replace_tree!(self,self.value_mut().unwrap(),&mut Self::new())
                },
                Some(value) => {
                    std::mem::replace(self.value_mut().unwrap(), value)
                },
            })

        }
    }
    /// pop the top value from a sorted tree and preserves order
    pub fn pop_sorted(&mut self) -> Option<Item> where Item : PartialOrd {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if left.is_empty() && right.is_empty() {
                Some(take_value_replace_tree!(self,value,&mut Self::new()))
            } else if right.is_empty() {
                Some(take_value_replace_tree!(self,value,left))
            } else if left.is_empty() {
                Some(take_value_replace_tree!(self,value,right))
            } else {
                let min_right = right.min_tree_mut().expect("min right should always return some tree");
                let min_right_value = min_right.value_mut().expect("min right should always return some item");
                std::mem::swap(value,min_right_value);
                min_right.pop_sorted()
            }
        }
    }
    /// pop the top value from a sorted tree and preserves order (using key function)
    pub fn pop_sorted_with_key<F,Key>(&mut self, key: &F) -> Option<Item> where
        Key : PartialOrd,
        F : Fn(&Item) -> &Key,
    {
        if self.is_empty() {
            None
        } else {
            let_node_ref_mut!(self => value, left, right);
            if left.is_empty() && right.is_empty() {
                Some(take_value_replace_tree!(self,value,&mut Self::new()))
            } else if right.is_empty() {
                Some(take_value_replace_tree!(self,value,left))
            } else if left.is_empty() {
                Some(take_value_replace_tree!(self,value,right))
            } else {
                let min_right = right.min_tree_mut_with_key(key).expect("min right should always return some tree");
                let min_right_value = min_right.value_mut().expect("min right should always return some item");
                std::mem::swap(value,min_right_value);
                min_right.pop_sorted_with_key(key)
            }
        }
    }
}

/// some tests
#[cfg(test)]
mod test {
    use crate::{tree, leaf, let_node_ref_mut};
    use super::take_value_replace_tree;

    #[test]
    fn test_macro_take_value_replace_tree() {
        let mut bt = tree(4,leaf(1),tree(7,leaf(6),()));
        assert_eq!(bt.to_string(),"((1) <= 4 => ((6) <= 7))");

        let_node_ref_mut!(bt => value, _left, right);

        // the first parameter contains the other 2, so the macro will work (see description)
        #[allow(invalid_value)]
        let value = take_value_replace_tree!(&mut bt, value, right);

        assert_eq!(value, 4);
        assert_eq!(bt.to_string(),"((6) <= 7)");
    }    
}