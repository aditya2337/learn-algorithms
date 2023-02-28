fn new_sort(list: &mut Vec<i32>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let partition_idx = partition(list, lo, hi);

    let new_high = if partition_idx <= lo {
        lo
    } else {
        partition_idx - 1
    };

    new_sort(list, lo, new_high);
    new_sort(list, partition_idx + 1, hi);
}

pub fn partition(list: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    let pivot = list[hi];
    let mut idx = lo;
    let mut i = lo;

    loop {
        if i > hi {
            break;
        }

        let item = list[i];
        if item < pivot {
            let temp = list[idx];
            list[idx] = list[i];
            list[i] = temp;
            idx += 1;
        }
        i += 1;
    }

    list[hi] = list[idx];
    list[idx] = pivot;
    idx
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut unsorted_list: Vec<i32> = vec![12, 3, 5, 6, 4, 2, 32, 9, 18, 43, 1, 8, 7];
        let hi = unsorted_list.len() - 1;
        new_sort(&mut unsorted_list, 0, hi);

        assert_eq!(
            unsorted_list,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 18, 32, 43]
        );
    }
}
