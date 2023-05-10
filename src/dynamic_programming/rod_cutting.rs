use std::{u8::MAX, cmp};

const PRICES: [u8; 10] = [1, 5, 8, 9, 10, 17, 17, 20, 24, 30];

pub fn rod_cut_recursively(idx: usize) -> u8  {
    if idx == 0 {
        return 0;
    }
    let mut max = 0;


    for i in 0..idx {
        max = cmp::max(PRICES[i], rod_cut_recursively(idx - 1))
    }

    return max;
}

#[cfg(test)]
mod tests {
    use super::rod_cut_recursively;

    #[test]
    fn test_rod_cut() {
        let max = rod_cut_recursively(8);

        assert_eq!(max, 10);
    }
}
