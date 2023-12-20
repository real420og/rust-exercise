pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[pivot] >= arr[j] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

#[test]
fn test_quick_sort() {
    let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
    quick_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
