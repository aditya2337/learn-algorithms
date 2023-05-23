pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];

    for i in 0..nums.len() {
        let mut r = vec![0; nums.len() - i];
        r[0] = nums[i];
        max = std::cmp::max(max, r[0]);
        for j in (i + 1)..nums.len() {
            let a_idx = j - i;
            let sum = nums[j] + r[a_idx - 1];
            max = std::cmp::max(max, sum);
            r[a_idx] = sum;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::dynamic_programming::maximum_subarray::max_sub_array;

    #[test]
    fn max_sub_array_test_1() {
        let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_sub_array(arr);
        assert_eq!(result, 6);
    }

    #[test]
    fn max_sub_array_test_2() {
        let arr = vec![1];
        let result = max_sub_array(arr);
        assert_eq!(result, 1);
    }

    #[test]
    fn max_sub_array_test_3() {
        let arr = vec![5, 4, -1, 7, 8];
        let result = max_sub_array(arr);
        assert_eq!(result, 23);
    }

    #[test]
    fn max_sub_array_test_4() {
        let arr = vec![-2, 1];
        let result = max_sub_array(arr);
        assert_eq!(result, 1);
    }
}
