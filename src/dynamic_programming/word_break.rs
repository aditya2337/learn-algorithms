pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut result = false;

    for i in 0..word_dict.len() {
        let mut word = word_dict[i].clone();

        if word == s {
            result = true;
            break;
        }

        for j in (i + 1)..word_dict.len() - 1 {
            word = format!("{}{}", word, word_dict[j].clone());
            if word == s {
                result = true;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::dynamic_programming::word_break::word_break;
    use rstest::rstest;

    #[rstest]
    #[case(String::from("leetcode"), vec![String::from("leet"),String::from("code")], true)]
    fn word_break_test(
        #[case] s: String,
        #[case] word_dict: Vec<String>,
        #[case] expected_output: bool,
    ) {
        assert_eq!(word_break(s, word_dict), expected_output);
    }
}
