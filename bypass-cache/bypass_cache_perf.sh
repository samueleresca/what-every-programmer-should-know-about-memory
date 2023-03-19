cargo build --release
perf stat -d -e cache-misses,cache-references,l1d.replacement,L1-dcache-loads,L1-dcache-load-misses,L1-dcache-stores,l2_lines_in.all ../target/release/bypass-cache $1