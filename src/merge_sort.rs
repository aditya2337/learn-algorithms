pub fn sort(arr: &mut Vec<i32>, p: usize, r: usize) {
    // pre

    if p >= r {
        return;
    }

    // recurse
    let q = (p + r) / 2;
    sort(arr, p, q);
    sort(arr, q + 1, r);
    // post
    merge(arr, p, q, r);
}

fn merge(arr: &mut Vec<i32>, p: usize, q: usize, r: usize) {
    let left_arr = arr[p..=q].to_vec();
    let right_arr = arr[q + 1..=r].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < left_arr.len() && j < right_arr.len() {
        if left_arr[i] < right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
            k += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
            k += 1;
        }
    }

    while i < left_arr.len() {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < right_arr.len() {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{merge, sort};

    #[test]
    fn test_merge_sort() {
        let mut list = vec![5, 2, 4, 6, 1, 3];
        let p = 0;
        let r = list.len() - 1;
        sort(&mut list, p, r);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge() {
        let mut list = vec![1, 3, 6, 2, 4, 5];
        let p = 0;
        let q = (list.len() - 1) / 2;
        let r = 5;

        merge(&mut list, p, q, r);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6]);
    }
}
