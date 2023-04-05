# test_iterators

A small Rust project that illustrates iteration using a simple binary tree

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
    println!("{:?}",t.inner().to_vec());
    // [1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 18]
    println!("{}",t.contains(&2));
    // true
    println!("{}",t.contains(&10));
    // true
    println!("{}",t.contains(&4));
    // false
    println!("{}",t.remove(&7));
    // true
    println!("{:?}",t.inner().to_vec());
    // [1, 2, 3, 5, 6, 8, 9, 10, 11, 18]
}
```

An excerpt of the implementation of the mutable iterator:

```rust
impl<'a,T> Iterator for BinTreeIterMut<'a,T> {
    type Item = &'a mut T;

    /// a deque is used to push and pop from both ends according to the specified traversal behavior
    fn next(&mut self) -> Option<Self::Item> {
        let pop = match self.traversal {
            DepthFirst(_) => self.data.pop_back(),
            BreadthFirst => self.data.pop_front(),
        };
        match pop {
            None => None,
            Some(BinTreeIterMutDataItem::Item(item)) => Some(item),
            Some(BinTreeIterMutDataItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIterMutDataItem::Tree(BinTree::Branch(item, left, right))) => {
                match self.traversal {
                    DepthFirst(InOrder) => {
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.next()
                    },
                    DepthFirst(PreOrder) => {
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.next()
                    },
                    DepthFirst(PostOrder) => {
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.next()

                    },
                    BreadthFirst => {
                        self.data.push_back(BinTreeIterMutDataItem::Item(item));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(left.as_mut()));
                        self.data.push_back(BinTreeIterMutDataItem::Tree(right.as_mut()));
                        self.next()
                    },
                }
            }
        }
    }
}
```
