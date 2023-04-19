fn sort(a: Vec<usize>) -> Vec<usize> {
    let mut k = 0;
    let mut b = vec![0; a.len()];

    for i in 0..a.len() {
        k = if a[i] > k { a[i] } else { k }
    }

    let mut c = vec![0; k + 1];

    for i in 0..a.len() {
        c[a[i]] = c[a[i]] + 1;
    }

    for i in 1..=k {
        c[i] = c[i] + c[i - 1];
    }

    let mut i = a.len();

    while i > 0 {
        let j = i - 1;
        b[c[a[j]] - 1] = a[j];
        c[a[j]] = c[a[j]] - 1;
        i -= 1;
    }

    b
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn test_count_sort() {
        let arr = vec![2, 5, 3, 0, 2, 3, 0, 3];
        let arr = sort(arr);

        assert_eq!(arr, vec![0, 0, 2, 2, 3, 3, 3, 4]);
    }
}
