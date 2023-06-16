mod binary_search;
mod binary_tree;
mod crystal_balls;
mod heap;
mod linear_search;
mod linked_lists;
mod matrix_multiply;
mod path_finding;
mod queue;
mod search_insert_pos;
mod dynamic_programming;
mod sorting;
mod graphs;

fn search_algorithms() {
    let haystack: Vec<i32> = vec![1, 2, 4, 5, 6, 6, 7, 8];

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
}

fn main() {
    search_algorithms();
}
