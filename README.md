# What every programmer should known about memory

This repository contains the rust implementation of the code available at [Ulrich Drepper's "What every programmer should know about memory" - What Programmers Can Do](https://lwn.net/Articles/255364/)

Every folder in the project represent a code sample:

- [Bypass cache](./bypass-cache/). 
- [Optimizing L1 Data Cache Access](./l1d-optimization/).

## Requirements

This repo requires:

- Rust
- perf tool installed 




The matrix multiplication above is an example of an operation that can be optimized by taking advantage of the sequential access. The `m1` is accessed sequentially (by accessing the matrix per row), while the `m2` is not accessed sequentially (the implementation iterates on the columns first).

It is possible to transpose the `m2` matrix and access it sequentially. This is done in the `optimized` function below:

<script src="https://gist.github.com/samueleresca/d8af00ccb0bc0b6bee01098219b0961a.js"></script>

Translating and multiply the `m2` matrix results in better performances. In specific, the table below shows the `cycles` and `instruction` perf events resulting from the execution of the `non_optimized` and `optimized` functions (Again, the code has been compiled in release mode).

|  Event Name      | *non optimized*    | *optimized* |
| ----------- | ----------- | ----------- | 
| `cycles`      | 13,665,923,747    | 9,471,167,309     |
| `instructions` | 26,474,014,440   | 24,976,442,438    |
| `ins. per cycle` | 1.94 | 2.64|


The `ins. per cycle` is a metric that shows how many instructions are executed per cycle. The higher the value, the better the performance. The `optimized` function has a higher `ins. per cycle` value because it is taking advantage of the sequential access for both the `m1` and `m2` matrices.

Another approach to speed up the matrix multiplication is by maximizing the L1d cache usage. This can be done by using techniques like the *loop tiling*. The code below shows the implementation of the matrix multiplication with the loop tiling technique:


The implementation above improves cache locality by computing the multiplications in block that are small enough to fit into the L1d cache. Therefore, the L1d cache usage is maximized and cache misses are reduced.
The table below shows some of the perf events resulting from the execution of the three functions: `non_optimized`, `optimized` and `optimized_tiled`.
Further optimization can be done by using SIMD instructions not covered in this post.