pub fn srt<'a>(unsorted_list: &'a mut Vec<i32>) -> &'a Vec<i32> {

    let mut i = 0;

    loop {
        if i >= unsorted_list.len()  {
            break;
        }

        let mut j = 0;

        loop {
            if j >= unsorted_list.len() - 1 - i {
                break;
            }

            if unsorted_list[j] > unsorted_list[j + 1] {
                swap(j, j + 1, unsorted_list);
            }

            j = j + 1;
        }

        i = i + 1;
    }



    unsorted_list
}

fn swap(previous_idx: usize, next_idx: usize, list: &mut Vec<i32>) -> &Vec<i32> {
    let temp = list[previous_idx];
    list[previous_idx] = list[next_idx];
    list[next_idx] = temp;
    list
}
