use test_iterators::{Tree, FormattedTree, FormattedTreeType};

fn main() {
    let mut t = Tree::branch(1,
        Tree::branch(2,
            Tree::leaf(3),
            Tree::empty(),
        ),
        Tree::branch(4,
            Tree::empty(),
            Tree::branch(5, 
                Tree::leaf(6), 
                Tree::empty()
            )
        )
    );

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
