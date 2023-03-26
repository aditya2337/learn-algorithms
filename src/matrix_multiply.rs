pub fn matrix_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if a.len() == 0 && b.len() == 0 {
        return vec![];
    }
    let a_col_length = a[0].len();
    let b_row_length = b.len();



    if a_col_length != b_row_length {
        panic!("assertion failed");
    }

    let b_col_length = b[0].len();
    let a_row_length = a.len();

    let mut res = vec![vec![0; b_col_length]; a_row_length];

    for i in 0..a_row_length {
        for j in 0..b_col_length {
            for k in 0..a_col_length {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let a = vec![vec![1, 2, 3]];
        let b = vec![vec![5, 6], vec![7, 8], vec![4, 5]];
        let expected = vec![vec![31, 37]];
        assert_eq!(matrix_multiply(&a, &b), expected);
    }

    #[test]
    fn test_matrix_multiply_empty() {
        let a = vec![];
        let b = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(matrix_multiply(&a, &b), expected);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_matrix_multiply_wrong_size() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![5, 6]];
        let _result = matrix_multiply(&a, &b);
    }
}

