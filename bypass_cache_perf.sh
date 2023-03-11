

perf stat -e cycles,cache-references,L1-dcache-loads,L1-dcache-stores,LLC-loads,LLC-stores target/release/bypass-cache $1