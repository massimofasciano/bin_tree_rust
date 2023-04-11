# bin_tree

A small Rust project that illustrates iteration and other operations on binary trees.

I used the following data structures to represent the binary tree:

```rust
#[repr(transparent)]
pub struct BinTree<Item> {
    pub root: Option<Box<BinTreeNode<Item>>>
}

pub struct BinTreeNode<Item> {
    pub value : Item,
    pub left : BinTree<Item>,
    pub right : BinTree<Item>,
}
```

Each node of the tree contains a value and two children (left and right tree).
A binary tree is an optional boxed node. The Option takes care of representing the empty tree.
The Box is necessary because the size of the tree is not known in advance and requires heap allocation.
A leaf is a node with 2 empty children.

Where possible, access methods hide the internal implementation of the tree. 
Only the bin_tree module directly uses the internals. 
The bulk of the code (in bin_tree_utils and bin_tree_iter) only uses access methods.
The struct fields are public because in some situations, the borrow checker is too coarse
and mutable references to parts of a struct are not possible with access methods
(self is borrowed in full).
By using 3 node access macro variants (let_node_ref, let_node_ref_mut and let_node_move), the code can
split a node into the 3 base parts in a more flexible and fine-grained way without having direct
knowledge of the internals of the node struct.

```rust
use bin_tree::{FormattedBinTree, FormattedBinTreeType, tree, leaf, OrderedSetBinTree, BinTreeMap};

#[test]
fn demo() {
    let mut t = 
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
        )));
        
    assert_eq!(format!("{}",t),
        "(((3) <= 2) <= 1 => (4 => ((6) <= 5)))");

    // wrapper class to provide different display methods
    let ft = FormattedBinTree::new(&t,FormattedBinTreeType::PrettyIndent("--"));
    assert_eq!(format!("{}", ft),"\
    ------@\n\
    ----5\n\
    --------@\n\
    ------6\n\
    --------@\n\
    --4\n\
    ----@\n\
    1\n\
    ----@\n\
    --2\n\
    ------@\n\
    ----3\n\
    ------@\n\
    ");

    // false (because the tree is not sorted and this method depends on it for the search)
    // by wrapping in an ordered set or map, we avoid these issues (see below)
    assert_eq!(t.contains_sorted(&2),false);
    
    assert_eq!(t.contains(&2),true);
    
    // true (happy accident: tree is partially sorted to the right)
    // by wrapping in an ordered set or map, we avoid these issues (see below)
    assert_eq!(t.contains_sorted(&5),true);
    
    // binary tree iter_mut
    for i in t.iter_mut() {
        print!("{}",i);
        if *i % 3 == 0 {
            *i /= 3;
        }
    }
    assert_eq!(t.to_vec(),vec![1,2,1,4,2,5]);
    
    // binary tree ordered set
    let v = vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2];
    let mut t = v.into_iter().collect::<OrderedSetBinTree<_>>();
    assert_eq!(t.to_string(),"[1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 18]");
    assert_eq!(t.inner().to_string(),
        "((((1 => (2)) <= 3 => (5)) <= 6 => ((7) <= 8 => (((9) <= 10) <= 11))) <= 18)");
    
    assert_eq!(t.contains(&2),true);
    assert_eq!(t.contains(&10),true);
    assert_eq!(t.contains(&4),false);
    
    assert_eq!(t.remove(&7),true);
    assert_eq!(format!("{}",t),"[1, 2, 3, 5, 6, 8, 9, 10, 11, 18]");
    assert_eq!(t.to_tree_string(),
        "((((1 => (2)) <= 3 => (5)) <= 6 => (8 => (((9) <= 10) <= 11))) <= 18)");

    // binary tree map
    let mut t : BinTreeMap<char, usize> = BinTreeMap::new();
    t.insert('a', 782);
    t.insert('b', 1782);
    t.insert('c', 500);
    assert_eq!(t.to_string(),"[('a', 782), ('b', 1782), ('c', 500)]");
    assert_eq!(t.contains_key(&'b'),true);
    assert_eq!(t.contains_key(&'z'),false);
    assert_eq!(t.get(&'a'),Some(&782));
    assert_eq!(t.swap(&'a', &'c'),true);
    assert_eq!(t.to_string(),"[('a', 500), ('b', 1782), ('c', 782)]");
    *t.get_mut(&'b').unwrap() += 8;
    assert_eq!(t.to_string(),"[('a', 500), ('b', 1790), ('c', 782)]");
    assert_eq!(t.remove(&'b'),Some(1790));
    assert_eq!(t.to_string(),"[('a', 500), ('c', 782)]");
}
```

An excerpt from the implementation of the mutable iterator:

```rust
impl<'a,T> Iterator for BinTreeIterMut<'a,T> {
    type Item = &'a mut T;

    /// a deque is used to push and pop from both ends according to the specified traversal behavior
    fn next(&mut self) -> Option<Self::Item> {
        let pop = match self.traversal {
            DepthFirst(_) => self.data.pop_back(),
            BreadthFirst => self.data.pop_front(),
        };
        use IterMutData::*;
        match pop {
            None => None, // no more work
            Some(Value(item)) => Some(item),
            Some(Tree(tree)) => {
                if tree.is_empty() {
                    self.next()
                } else {
                    let_node_ref_mut!(tree => value, left, right);
                    match self.traversal {
                        DepthFirst(InOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.next()
                        },
                        DepthFirst(PreOrder) => {
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Value(value));
                            self.next()
                        },
                        DepthFirst(PostOrder) => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(right));
                            self.data.push_back(Tree(left));
                            self.next()

                        },
                        BreadthFirst => {
                            self.data.push_back(Value(value));
                            self.data.push_back(Tree(left));
                            self.data.push_back(Tree(right));
                            self.next()
                        },
                    }
                }
            }
        }
    }
}
```
