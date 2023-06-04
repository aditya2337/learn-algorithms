pub fn climb_stairs(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut dp = vec![0; n as usize + 1];

    for i in (0..dp.len()).rev() {
        if i == dp.len() - 1 || i == (dp.len() - 1)- 1 {
            dp[i] = 1;
        } else {
            dp[i] = dp[i + 1] + dp[i + 2];
        }
    }

    println!("{:?}", dp);

    dp[0]
}

#[cfg(test)]
mod tests {
    use crate::dynamic_programming::climbing_stairs::climb_stairs;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 3)]
    #[case(4, 5)]
    fn climb_stairs_test_1(#[case] n: i32, #[case] expected: i32) {
        assert_eq!(climb_stairs(n), expected);
    }
}
