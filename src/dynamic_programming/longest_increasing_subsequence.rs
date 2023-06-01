pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut max = 1;
    let mut lis = vec![1; nums.len()];

    for i in (0..nums.len()).rev() {
        for j in (i + 1)..nums.len() {
            if nums[j] > nums[i] {
                lis[i] = std::cmp::max(lis[i], 1 + lis[j]);
                if lis[i] > max {
                    max = lis[i];
                }
            }
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

        #[test]
    fn length_of_lis_test_4() {
        let arr = vec![-2,-1];

        assert_eq!(length_of_lis(arr), 2);
    }
}
