use std::collections::{HashMap, HashSet, LinkedList};

pub fn topological_sort(graph: &HashMap<i32, Vec<i32>>) -> Option<LinkedList<i32>> {
    let mut result = LinkedList::new();
    let mut visited = HashSet::new();
    let mut loop_set = HashSet::new();

    let mut keys: Vec<&i32> = graph.keys().collect();
    keys.sort();

    for &node in keys {
        if visited.contains(&node) {
            continue;
        }

        if !dfs(node, graph, &mut result, &mut visited, &mut loop_set) {
            return None;
        }
    }

    Some(result)
}

fn dfs(
    node: i32,
    graph: &HashMap<i32, Vec<i32>>,
    result: &mut LinkedList<i32>,
    visited: &mut HashSet<i32>,
    loop_set: &mut HashSet<i32>,
) -> bool {
    loop_set.insert(node);
    visited.insert(node);

    if let Some(childs) = graph.get(&node) {
        for &child in childs {
            if loop_set.contains(&child) {
                return false;
            }

            if visited.contains(&child) {
                continue;
            }

            if !dfs(child, graph, result, visited, loop_set) {
                return false;
            }
        }
    }

    loop_set.remove(&node);
    result.push_front(node);
    true
}

#[test]
fn test_topological_sort() {
    let mut graph = HashMap::new();
    graph.insert(7, vec![11, 8]);
    graph.insert(5, vec![11]);
    graph.insert(3, vec![8, 10]);
    graph.insert(11, vec![2, 9, 10]);
    graph.insert(8, vec![9]);
    graph.insert(2, vec![]);
    graph.insert(9, vec![]);
    graph.insert(10, vec![]);

    let sorted_nodes = topological_sort(&graph);

    let mut expected = LinkedList::new();
    expected.push_back(7);
    expected.push_back(5);
    expected.push_back(11);
    expected.push_back(3);
    expected.push_back(10);
    expected.push_back(8);
    expected.push_back(9);
    expected.push_back(2);

    assert_eq!(sorted_nodes, Some(expected));

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

    let sorted_nodes = topological_sort(&graph);

    let mut expected = LinkedList::new();
    expected.push_back(9);
    expected.push_back(2);
    expected.push_back(1);
    expected.push_back(3);
    expected.push_back(7);
    expected.push_back(4);
    expected.push_back(5);
    expected.push_back(8);
    expected.push_back(6);

    assert_eq!(sorted_nodes, Some(expected));
}
