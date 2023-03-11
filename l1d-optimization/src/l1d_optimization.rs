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

pub fn optimized(n: usize) -> Vec<Vec<f64>> {
    let m1 = generate_matrix(n);
    let m2 = m1.clone();

    let mut tmp = vec![vec![0.0; n]; n];
    let mut res = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..n {
            tmp[i][j] = m2[j][i];
        }
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] += m1[i][k] * tmp[j][k];
            }
        }
    }

    res
}

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
    use crate::l1d_optimization::{generate_matrix, non_optimized, optimized};

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
    fn optimized_and_nonoptimized_results_are_equal() {
        let result = non_optimized(3);
        let result_optimized = optimized(3);
        assert_eq!(result, result_optimized);
    }
}
