use bintree_iterators::{FormattedBinTree, FormattedBinTreeType, tree, leaf, BinTree};

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

    let mut t = BinTree::empty();
    t.extend_sorted_unique(vec![18,6,3,8,5,11,1,7,3,5,2,8,10,3,6,9,3,2]);
    assert_eq!(t.to_string(),"((((1 => (2)) <= 3 => (5)) <= 6 => ((7) <= 8 => (((9) <= 10) <= 11))) <= 18)");
    for i in t.to_vec() {
        println!("assert_eq!(t.remove_sorted(&{}),true);",i);
    }
}
