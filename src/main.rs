use bin_tree::{FormattedBinTree, FormattedBinTreeType, tree, leaf, OrderedSetBinTree};

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
