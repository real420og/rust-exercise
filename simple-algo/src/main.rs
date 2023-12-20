use std::collections::HashMap;
use simple_algo::quick_sort::quick_sort;
use simple_algo::heap_sort::heap_sort;
use simple_algo::binary_search::binary_search;
use simple_algo::merge_sort::merge_sort;
use simple_algo::topological_sort::topological_sort;

fn main() {
    let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
    quick_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
    heap_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
    merge_sort(&mut arr);
    println!("{:?}", arr);

    let arr = vec![2, 9, 10, 11, 12, 27, 38, 47, 54];
    let key = binary_search(&arr, 54);
    println!("{:?}", key);


    let mut graph = HashMap::new();
    graph.insert(1, vec![3]);
    graph.insert(2, vec![8]);
    graph.insert(3, vec![7]);
    graph.insert(4, vec![6]);
    graph.insert(5, vec![8]);
    graph.insert(6, vec![]);
    graph.insert(7, vec![5,4]);
    graph.insert(8, vec![6]);
    graph.insert(9, vec![2,5]);

    if let Some(sorted_nodes) = topological_sort(&graph) {
        println!("{:?}", sorted_nodes)
    }
}