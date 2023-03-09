pub fn sort(arr: &mut Vec<i32>) {
    let mut i = 2;
    loop {
        if i > arr.len() {
            break;
        }
        let mut j = i - 1;
        let key = arr[i - 1];

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn test_insertion_sort() {
        let mut list = vec![5, 2, 4, 6, 1, 3];
        sort(&mut list);
        assert_eq!(list, vec![1, 2, 3, 4, 5, 6]);
    }
}
