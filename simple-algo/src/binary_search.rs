pub fn binary_search(arr: &[i32], value: i32) -> Option<usize> {
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

#[test]
fn test_binary_search() {
    let arr = vec![2, 9, 10, 11, 12, 27, 38, 47, 54];

    let key = binary_search(&arr, 1);
    assert_eq!(None, key);

    for i in 0..arr.len() {
        let key = binary_search(&arr, arr[i]);
        assert_eq!(i, key.unwrap());
    }
}

