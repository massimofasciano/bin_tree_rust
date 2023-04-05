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
    t.extend_sorted_unique(vec![4,5,7,6,3,1,2]);
    // t.extend_sorted_unique(vec![3,1,2]);
    println!("{:?}",t.min_mut());
    println!("{:?}",t.max_mut());
    println!("{:?}",t.min());
    println!("{:?}",t.max());
    println!("{}",t);
    t.remove_sorted(&5);
    // t.pop_sorted();
    println!("{}",t);
}
