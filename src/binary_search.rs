pub fn binary_search(hackstack: &Vec<i32>, needle: &i32) -> bool {
    let mut is_found = false;

    let mut high = hackstack.len() - 1;
    let mut low = 0;

    while low < high {
        let mid = low + (high - low) / 2;
        let value_at_middle = hackstack.get(mid);

        match value_at_middle {
            Some(found_value) => {
                if needle == found_value {
                    is_found = true;
                    break;
                } else if needle > found_value {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            None => {
                break;
            }
        }
    }

    is_found
}
