use test_iterators::{Tree, FormattedTree, FormattedTreeType};

fn main() {
    let mut t = Tree::new_branch(1,
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
    // (((3) <= 2) <= 1 => (4 => ((6) <= 5)))

    println!("{}",FormattedTree::new(&t,FormattedTreeType::PrettyIndent("--")));
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
