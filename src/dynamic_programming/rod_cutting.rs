use std::cmp;

const PRICES: [u8; 40] = [
    1, 5, 8, 9, 10, 17, 17, 20, 24, 30, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30, 1, 5, 8, 9, 10, 17, 17,
    20, 24, 30, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30,
];

pub fn rod_cut_recursively(idx: usize) -> u8 {
    if idx == 0 {
        return 0;
    }
    let mut max = u8::MIN;

    for i in 1..idx {
        max = cmp::max(max, PRICES[i] + rod_cut_recursively(idx - i - 1))
    }

    return max;
}

pub fn rod_cut_top_down(idx: usize) -> u8 {
    let mut r = vec![u8::MIN; idx + 1];
    return memoized_cut_rod_aux(idx, &mut r);
}

fn memoized_cut_rod_aux(idx: usize, r: &mut Vec<u8>) -> u8 {
    if r[idx] > u8::MIN {
        return r[idx];
    }

    if idx == 0 {
        return 0;
    }

    let mut q = r[idx];

    for i in 1..idx {
        q = cmp::max(q, PRICES[i] + memoized_cut_rod_aux(idx - i - 1, r))
    }
    r[idx] = q;

    return q;
}

#[cfg(test)]
mod tests {
    use crate::dynamic_programming::rod_cutting::rod_cut_top_down;

    use super::rod_cut_recursively;

    #[test]
    fn test_rod_cut() {
        let max = rod_cut_recursively(4);
        assert_eq!(max, 10);

        // Takes about 4 s
        // let max = rod_cut_recursively(40);
        // assert_eq!(max, 120);

        // Runs in some ms
        let dp_max = rod_cut_top_down(40);
        assert_eq!(dp_max, 120);
    }
}
