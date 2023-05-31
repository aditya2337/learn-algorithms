pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let len = nums.len() + 1;
    let mut r = vec![vec![0; len]; len];

    for i in 1..len {
        for j in (i + 1)..len {
            let new_val: i32;
            if nums[i - 1] > nums[j - 1] {
                new_val = 1 + r[i - 1][j - 1];
            } else {
                new_val = std::cmp::max(r[i - 1][j], r[i][j - 1]);
            }
            if new_val > max {
                max = new_val.clone();
            }
            r[i][j] = new_val;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::dynamic_programming::longest_increasing_subsequence::length_of_lis;

    #[test]
    fn length_of_lis_test_1() {
        let arr = vec![10, 9, 2, 5, 3, 7, 101, 18];

        assert_eq!(length_of_lis(arr), 4);
    }

    #[test]
    fn length_of_lis_test_2() {
        let arr = vec![0, 1, 0, 3, 2, 3];

        assert_eq!(length_of_lis(arr), 4);
    }

    #[test]
    fn length_of_lis_test_3() {
        let arr = vec![7, 7, 7, 7, 7, 7, 7];

        assert_eq!(length_of_lis(arr), 1);
    }
}
