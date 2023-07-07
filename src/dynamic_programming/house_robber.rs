pub fn rob(nums: Vec<i32>) -> i32 {
    let mut rob_1 = 0;
    let mut rob_2 = 0;

    for n in nums.iter() {
        let temp = std::cmp::max(rob_1 + n, rob_2);
        rob_1 = rob_2;
        rob_2 = temp;
    }

    rob_2
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,1], 4)]
    #[case(vec![2,7,9,3,1], 12)]
    #[case(vec![2,1,1,2], 4)]
    fn rob_test(#[case] nums: Vec<i32>, #[case] expected_output: i32) {
        assert_eq!(
            crate::dynamic_programming::house_robber::rob(nums),
            expected_output
        );
    }
}
