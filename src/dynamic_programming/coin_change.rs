pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut min_coins = -1;

    cal_min_coins(&coins, amount, 0, &mut min_coins);

    min_coins
}

fn cal_min_coins(coins: &Vec<i32>, amount: i32, total_coins: i32, min_coins: &mut i32) {
    for coin in coins.iter() {
        let result = amount - coin;
        let new_total = total_coins + 1;
        if result > 0 {
            cal_min_coins(coins, result, new_total, min_coins);
        } else if result == 0 {
            if &new_total < min_coins || min_coins < &mut 0 {
                *min_coins = new_total;
            }
        }
    }
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
    fn coin_change_test(
        #[case] coins: Vec<i32>,
        #[case] amount: i32,
        #[case] expected_output: i32,
    ) {
        assert_eq!(coin_change(coins, amount), expected_output);
    }
}
