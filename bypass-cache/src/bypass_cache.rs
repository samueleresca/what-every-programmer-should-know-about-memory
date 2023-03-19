/// Returns an initialized matrix. The matrix is initialized in a standard way.
///
/// # Arguments
///
/// * `n` - The dimension of the matrix.
pub fn standard_initialize(n: usize) -> Vec<Vec<i32>> {
    let mut data = vec![vec![0i32; n]; n];

    for r in 0..n {
        for c in 0..n {
            data[r][c] = c as i32;
        }
    }
    data
}

use std::arch::x86_64::_mm_stream_si32;

/// Returns an initialized matrix. The matrix is initialized using the *non-temporal* write operations.
///
/// The new content is directly written to the memory.
///
/// # Arguments
///
/// * `n` - The dimension of the matrix.
pub fn nocache_initialize(n: usize) -> Vec<Vec<i32>> {
    let mut data = vec![vec![0i32; n]; n];

    for r in 0..n {
        let row = data[r].as_mut_ptr();
        for c in 0..n {
            unsafe {
                _mm_stream_si32(row.add(c), c as i32);
            }
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use crate::bypass_cache::{nocache_initialize, standard_initialize};

    #[test]
    fn nocache_writes_works() {
        let result = nocache_initialize(3);
        assert_eq!(result[0][0], 0);
        assert_eq!(result[0][1], 1);
        assert_eq!(result[0][2], 2);
        assert_eq!(result[2][2], 2);
    }

    #[test]
    fn standard_writes_works() {
        let result = standard_initialize(3);
        assert_eq!(result[0][0], 0);
        assert_eq!(result[0][1], 1);
        assert_eq!(result[0][2], 2);
        assert_eq!(result[2][2], 2);
    }
}
