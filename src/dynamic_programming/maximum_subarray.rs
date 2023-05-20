pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut r = vec![0; nums.len()];
    let mut max = r[0];

    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in 0..i {
            sum = nums[i] + r[i - j];
        }
        r[i] = sum;
    }

    println!("{:?}", r);

    max
}

#[cfg(test)]
mod tests {
    use crate::dynamic_programming::maximum_subarray::max_sub_array;

    #[test]
    fn max_sub_array_test() {
        let arr1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

        let result = max_sub_array(arr1);
        assert_eq!(result, 6);
    }
}
