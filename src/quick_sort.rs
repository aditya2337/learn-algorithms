fn sort(list: Vec<i32>) -> Vec<i32> {
    if list.len() < 2 {
        return list;
    }

    let pivot = list.len() / 2;
    let item_at_pivot = list[pivot];

    // all items smaller than the pivot go to left
    let mut left_arr: Vec<i32> = vec![];
    let mut right_arr: Vec<i32> = vec![];

    for (pos, e) in list.iter().enumerate() {
        if pos != pivot {
            if e < &item_at_pivot {
                // push to leftArr
                left_arr.push(e.clone());
            } else {
                // push to rightArr
                right_arr.push(e.clone());
            }
        }
    }

    return vec![sort(left_arr), vec![item_at_pivot], sort(right_arr)].concat();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_quick_sort() {
        let unsorted_list: Vec<i32> = vec![12, 3, 5, 6, 4, 2, 32, 9, 18, 43, 1, 8, 7];
        let sorted_list = sort(unsorted_list);

        assert_eq!(
            sorted_list,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 18, 32, 43]
        );
    }
}
