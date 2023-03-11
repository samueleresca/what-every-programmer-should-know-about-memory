cargo build --release
perf stat -e cycles,cache-references,L1-dcache-load-misses,L1-dcache-loads,L1-dcache-stores ../target/release/l1d-optimization $1