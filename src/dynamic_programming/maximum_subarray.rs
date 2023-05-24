pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    // max_sub_array_iterative(nums)
    max_sub_array_recursive(nums)
}
pub fn max_sub_array_iterative(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];

    for i in 0..nums.len() {
        let mut r = vec![0; nums.len() - i];
        r[0] = nums[i];
        max = std::cmp::max(max, r[0]);
        for j in (i + 1)..nums.len() {
            let r_idx = j - i;
            let sum = nums[j] + r[r_idx - 1];
            max = std::cmp::max(max, sum);
            r[r_idx] = sum;
        }
    }

    max
}

pub fn max_sub_array_recursive(nums: Vec<i32>) -> i32 {
    fn solve(a: &Vec<i32>, idx: usize, must_pick: bool) -> i32 {
        if idx >= a.len() {
            return match must_pick {
                true => 0,
                false => i32::MIN,
            };
        };
        if must_pick {
            return std::cmp::max(0, a[idx] + solve(a, idx + 1, true));
        }

        return std::cmp::max(solve(a, idx + 1, false), a[idx] + solve(a, idx + 1, true));
    }

    solve(&nums, 0, false)
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
