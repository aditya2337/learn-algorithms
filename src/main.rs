mod binary_search;
mod bubble_sort;
mod crystal_balls;
mod linear_search;
mod linked_lists;

fn i32_vector_to_string(i32_vec: &Vec<i32>) -> String {
    let string_vec: Vec<String> = i32_vec.iter().map(|x| x.to_string()).collect();
    string_vec.join(",")
}

fn search_algorithms() {
    let haystack: Vec<i32> = vec![1, 2, 4, 5, 6, 6, 7, 8];
    let mut unsorted_list: Vec<i32> = vec![1, 11, 9, 5, 2, 24, 4, 8];

    println!(
        "Linear search: {}",
        linear_search::linear_search(&haystack, &4)
    );

    println!(
        "Binary search: {}",
        binary_search::binary_search(&haystack, &4)
    );
    println!(
        "Binary search: {}",
        crystal_balls::crystal_balls(&vec![0, 0, 1, 1, 1, 1])
    );
    println!(
        "Bubble sortedList: {}",
        i32_vector_to_string(&bubble_sort::srt(&mut unsorted_list))
    );

    let mut ll = linked_lists::LinkedList::<i32>::new();
    println!("Linked list length is: {:?}", ll.get_length());
    ll.append(2);
    ll.append(2);
    println!("Linked list length is: {:?}", ll);
    println!("Linked list length is: {:?}", ll.get_length());
    ll.insert_at(3, 0);
    println!("Linked list length is: {:?}", ll);
    println!("Linked list length is: {:?}", ll.get_length());
}

fn main() {
    search_algorithms();
}
