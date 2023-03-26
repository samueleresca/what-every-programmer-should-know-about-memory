use l1d_optimization::{non_optimized, optimized, optimized_tiled};
use std::env;

mod l1d_optimization;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = 1000;

    match args.get(1).map(|x| x.as_str()) {
        Some("optimized") => optimized(n),
        Some("non_optimized") => non_optimized(n),
        Some("optimized_tiled") => optimized_tiled(n),
        _ => panic!("No valid arg provided"),
    };
}
