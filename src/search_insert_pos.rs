pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    search_idx(0, nums.len() - 1, target, nums)
}

fn search_idx(start_idx: usize, end_idx: usize, target: i32, nums: Vec<i32>) -> i32 {
    if target == nums[start_idx] {
        return start_idx as i32;
    }

    if target == nums[end_idx] {
        return end_idx as i32;
    }
    let diff = end_idx - start_idx;

    if diff <= 1 {
        if target < nums[start_idx] {
            return start_idx as i32;
        }

        if target > nums[start_idx] && target < nums[end_idx] {
            return end_idx as i32;
        }

        return (end_idx + 1) as i32;
    }

    let mid = (start_idx + end_idx) / 2;

    if nums[mid] == target {
        return mid as i32;
    }

    if target < nums[mid] {
        search_idx(start_idx, mid, target, nums)
    } else {
        search_idx(mid, end_idx, target, nums)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search() {
        let nums = vec![1, 3, 5, 6];

        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
