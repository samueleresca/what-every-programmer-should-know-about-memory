# Optimizing L1 Data Cache Access


perf stat -e  cycles ./target/release/l1d-optimization optimized
perf stat -e  cycles ./target/release/l1d-optimization non_optimized