mod avl;
mod redblack;
use std::time::Instant;
//https://nnethercote.github.io/perf-book/benchmarking.html
// https://stackoverflow.com/questions/13322479/benchmarking-programs-in-rust

fn main() {

    let arr:[u32;5] = [10000, 40000, 70000, 100000, 130000];
    for len in arr{
        // let mut avl_tree = avl::new_avl(0, true);
        let mut red_black_tree = redblack::new_rb(0, true);
        let start = Instant::now();
        for i in 1..len{
            red_black_tree = redblack::insert(&mut red_black_tree,i);
            // avl_tree = avl::insert_node(&mut avl_tree,i); // overflows the stack
        } // tree will have 0 to len after the for loop
        
        for j in 0..(len/10) {
            let _ = redblack::find_node(&mut red_black_tree, 0);
        }
        let elapsed = start.elapsed();
        println!("{:.2?}", elapsed);
    }
}
