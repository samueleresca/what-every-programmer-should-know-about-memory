use cache_size::l1_cache_line_size;

/// Returns the result of the matrix multiplication.
///
/// # Arguments
///
/// * `n` - The dimension of the matrix.
pub fn non_optimized(n: usize) -> Vec<Vec<f64>> {
    let m1 = generate_matrix(n);
    let m2 = m1.clone();

    let mut res = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }

    res
}

/// Returns the result of a matrix multiplication.
/// It transpose the second matrix to optimize the sequential memory access.
///
/// # Arguments
///
/// * `n` - The dimension of the matrix.
pub fn optimized(n: usize) -> Vec<Vec<f64>> {
    let m1 = generate_matrix(n);
    let m2 = m1.clone();

    let mut tmp = vec![vec![0.0; n]; n];
    let mut res = vec![vec![0.0; n]; n];

    // Transpose the second matrix.
    for i in 0..n {
        for j in 0..n {
            tmp[i][j] = m2[j][i];
        }
    }

    // Perform the multiplication.
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] += m1[i][k] * tmp[j][k];
            }
        }
    }

    res
}

/// Returns the result of a matrix multiplication.
/// Uses loop tiling to optimize the matrix multiplication.
/// The function relies on cache_size crate to get the cache line size.
///
/// # Arguments
///
/// * `n` - The dimension of the matrix.
pub fn optimized_tiled(n: usize) -> Vec<Vec<f64>> {
    let m1 = generate_matrix(n);
    let m2 = m1.clone();
    let mut res = vec![vec![0.0; n]; n];

    // Get the cache line size
    let block_size: usize = l1_cache_line_size().unwrap() / std::mem::size_of::<f64>();

    // Loop through each block
    for i in (0..n).step_by(block_size) {
        for j in (0..n).step_by(block_size) {
            for k in (0..n).step_by(block_size) {
                // Loop through each element in the block
                for ii in i..std::cmp::min(i + block_size, n) {
                    for jj in j..std::cmp::min(j + block_size, n) {
                        for kk in k..std::cmp::min(k + block_size, n) {
                            // Do the actual multiplication
                            res[ii][jj] += m1[ii][kk] * m2[kk][jj];
                        }
                    }
                }
            }
        }
    }

    res
}

/// Initializes a matrix with values from 0 to n - 1.
/// # Arguments
///
/// * `n` - The dimension of the matrix.
///
/// # Returns a matrix with values from 0 to n - 1.
pub fn generate_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; n]; n];
    let mut value = 0.0;
    for r in 0..n {
        for c in 0..n {
            matrix[r][c] = value;
            value += 1.0;
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use crate::l1d_optimization::{generate_matrix, non_optimized, optimized, optimized_tiled};

    #[test]
    fn generate_matrix_works() {
        let result = generate_matrix(3);
        assert_eq!(result[0][0], 0.0);
        assert_eq!(result[1][1], 4.0);
        assert_eq!(result[2][2], 8.0);
    }

    #[test]
    fn non_optimized_works() {
        let result = non_optimized(3);
        assert_eq!(result[0][0], 15.0);
        assert_eq!(result[1][1], 54.0);
        assert_eq!(result[2][2], 111.0);
    }

    #[test]
    fn optimized_works() {
        let result = optimized(3);
        assert_eq!(result[0][0], 15.0);
        assert_eq!(result[1][1], 54.0);
        assert_eq!(result[2][2], 111.0);
    }

    #[test]
    fn optimized_loop_tiling_works() {
        let result = optimized_tiled(3);
        assert_eq!(result[0][0], 15.0);
        assert_eq!(result[1][1], 54.0);
        assert_eq!(result[2][2], 111.0);
    }

    #[test]
    fn optimized_and_nonoptimized_results_are_equal() {
        let result = non_optimized(3);
        let result_optimized = optimized(3);
        assert_eq!(result, result_optimized);
    }
}
