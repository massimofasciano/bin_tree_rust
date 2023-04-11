# bintree_iterators

A small Rust project that illustrates iteration using a simple binary tree.

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

Access methods hide the internal implementation of the tree. Only the bin_tree module has access to the internals. 
The bulk of the code (in bin_tree_utils and bin_tree_iter) only uses access methods.

```rust
use bintree_iterators::{FormattedBinTree, FormattedBinTreeType, tree, leaf, OrderedSetBinTree};

fn main() {
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
        
    println!("{}",t);
    // (((3) <= 2) <= 1 => (4 => ((6) <= 5)))

    let ft = FormattedBinTree::new(&t,FormattedBinTreeType::PrettyIndent("--"));
    println!("{}", ft);
    // ------@  
    // ----5    
    // --------@
    // ------6  
    // --------@
    // --4      
    // ----@    
    // 1        
    // ----@    
    // --2
    // ------@
    // ----3
    // ------@

    println!("{}",t.contains_sorted(&2));
    // false (because the tree is not sorted)
    println!("{}",t.contains(&2));
    // true
    println!("{}",t.contains_sorted(&5));
    // true (happy accident: tree is partially sorted to the right)

    for i in t.iter() {
        print!("{} ",i)
    }
    println!("");
    // 3 2 1 4 6 5

    for i in ft.iter() {
        println!("|{i}|")
    }
    // |3|
    // |2|
    // |1|
    // |4|
    // |6|
    // |5|

    for i in t.iter_mut() {
        print!("{}",i);
        if *i % 3 == 0 {
            *i /= 3;
            println!(" => {}",i);
        } else {
            println!("");
        }
    }
    // 3 => 1
    // 2
    // 1
    // 4
    // 6 => 2
    // 5

    for i in t {
        print!("{} ",i);
    }
    println!("");
    // 1 2 1 4 2 5

    let v = vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2];
    let mut t = v.into_iter().collect::<OrderedSetBinTree<_>>();
    println!("{}",t);
    // [1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 18]
    println!("{}",t.inner());
    // ((((1 => (2)) <= 3 => (5)) <= 6 => ((7) <= 8 => (((9) <= 10) <= 11))) <= 18)
    println!("{}",t.contains(&2));
    // true
    println!("{}",t.contains(&10));
    // true
    println!("{}",t.contains(&4));
    // false
    println!("{}",t.remove(&7));
    // true
    println!("{}",t);
    // [1, 2, 3, 5, 6, 8, 9, 10, 11, 18]
    println!("{}",t.inner());
    // ((((1 => (2)) <= 3 => (5)) <= 6 => (8 => (((9) <= 10) <= 11))) <= 18)
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
