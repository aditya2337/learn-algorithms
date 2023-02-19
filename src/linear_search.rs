pub fn linear_search(haystack: &Vec<i32>, needle: &i32) -> bool {
    let mut found = false;
    for item in haystack.iter()  {
       if item == needle {
          found = true;
          break;
       };
    }

    found
}

