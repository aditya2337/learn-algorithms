pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut dp: Vec<i32> = vec![-1; amount as usize + 1];

    cal_min_coins(&coins, amount, 0, &mut dp)
}

fn cal_min_coins(coins: &Vec<i32>, amount: i32, total_coins: i32, dp: &mut Vec<i32>) -> i32 {
    if amount < 0 {
        return -1;
    }

    if dp[amount as usize] > 0 {
        return dp[amount as usize];
    }

    if amount == 0 {
        return 0;
    }

    let mut min_coins = -1;

    for &coin in coins {
        if coin <= amount {
            let result = cal_min_coins(coins, amount - coin, total_coins, dp);

            if result != -1 && (min_coins < 0 || result < min_coins) {
                min_coins = result + 1;
            }
        }
    }

    dp[amount as usize] = min_coins;

    min_coins
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use crate::dynamic_programming::coin_change::coin_change;

    #[rstest]
    #[case(vec![1,2,5], 11, 3)]
    #[case(vec![1,3, 4, 5], 7, 2)]
    #[case(vec![2], 3, -1)]
    #[case(vec![1], 0, 0)]
    #[case(vec![1,2,5], 100, 20)]
    fn coin_change_test(
        #[case] coins: Vec<i32>,
        #[case] amount: i32,
        #[case] expected_output: i32,
    ) {
        assert_eq!(coin_change(coins, amount), expected_output);
    }
}
