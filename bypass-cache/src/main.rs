use std::env;

use bypass_cache::{nocache_writes, standard_writes};

mod bypass_cache;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = 1_000_000;

    match args.get(1).map(|x| x.as_str()) {
        Some("nocache") => nocache_writes(n),
        Some("standard") => standard_writes(n),
        _ => panic!("No valid arg provided"),
    };
}
