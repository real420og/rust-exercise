fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quicksort(&mut arr[0..pivot]);
    quicksort(&mut arr[pivot + 1..]);
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

fn heap_sort(arr: &mut [i32]) {
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

fn binary_search(arr: &[i32], value: i32) -> Option<usize> {
    let mut first = 0;
    let mut last = arr.len() - 1;
    while first <= last {
        let mid = (first + last) / 2;
        if arr[mid] == value {
            return Some(mid);
        }
        if value > arr[mid] {
            first = mid.checked_add(1)?;
        } else {
            last = mid.checked_sub(1)?;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search() {
        let arr = vec![2, 9, 10, 11, 12, 27, 38, 47, 54];

        let key = crate::binary_search(&arr, 1);
        assert_eq!(None, key);

        for i in 0..arr.len() {
            let key = crate::binary_search(&arr, arr[i]);
            assert_eq!(i, key.unwrap());
        }
    }

    #[test]
    fn quicksort() {
        let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
        crate::quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn heap_sort() {
        let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
        crate::heap_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
