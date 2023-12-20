pub fn heap_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], len: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if len > left && arr[left] > arr[largest] {
        largest = left;
    }
    if len > right && arr[right] > arr[largest] {
        largest = right;
    }
    if root != largest {
        arr.swap(root, largest);
        heapify(arr, len, largest);
    }
}

#[test]
fn test_heap_sort() {
    let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
    heap_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
