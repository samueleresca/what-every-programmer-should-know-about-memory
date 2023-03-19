use std::env;

use bypass_cache::{nocache_initialize, standard_initialize};

mod bypass_cache;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = 3_000;

    match args.get(1).map(|x| x.as_str()) {
        Some("nocache") => nocache_initialize(n),
        Some("standard") => standard_initialize(n),
        _ => panic!("No valid arg provided"),
    };
}
