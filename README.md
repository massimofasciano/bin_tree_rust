# test_iterators

A small Rust project that illustrates iteration using a simple binary tree

```rust
use test_iterators::{FormattedTree, FormattedTreeType, tree, leaf};

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

    let ft = FormattedTree::new(&t,FormattedTreeType::PrettyIndent("--"));
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
        print!("{} ",i)
    }
    println!("");
    // 1 2 1 4 2 5
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
