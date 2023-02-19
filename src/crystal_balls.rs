pub fn crystal_balls(breaks: &Vec<i32>) -> i32 {
    let len = breaks.len();
    let sqrt_of_len = (len as f32).sqrt() as usize;
    let mut i = sqrt_of_len - 1;
    let mut idx = -1;

    loop {
        if i > len {
            break;
        }

        if breaks[i] == 1 {
            break;
        }

        i = i + sqrt_of_len;
    }

    i -= sqrt_of_len;
    let mut j = 0;

    loop {
        if j > i && i > len {
            break;
        }

        if breaks[i] == 1 {
            idx = i as i32;
            break;
        }

        j = j + 1;
        i = i + 1;
    }

    idx
}
