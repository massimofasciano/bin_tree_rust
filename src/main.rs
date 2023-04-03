use test_iterators::{Tree, PrettyFormatTree};

fn main() {
    let t = Tree::new_branch(1,
        Tree::new_branch(2,
            Tree::new_item(3),
            Tree::new(),
        ),
        Tree::new_branch(4,
            Tree::new(),
            Tree::new_branch(5, 
                Tree::new_item(6), 
                Tree::new()
            )
        )
    );
    println!("{}",t);
    println!("{}",PrettyFormatTree::new(&t));
    for i in t.iter() {
        println!("{}",i)
    }
}
