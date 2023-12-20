pub fn merge_sort(arr:&mut[i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let mid =len /2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let mut merged = Vec::with_capacity(len);
    let (mut left_idx, mut right_idx) = (0,0);

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            merged.push(left[left_idx]);
            left_idx+=1;
            continue;
        }

        merged.push(right[right_idx]);
        right_idx+=1;
    }

    merged.extend_from_slice(&left[left_idx..]);
    merged.extend_from_slice(&right[right_idx..]);

    arr.copy_from_slice(&merged);
}

#[test]
fn test_merge_sort() {
    let mut arr = vec![5, 2, 9, 1, 7, 6, 3, 8, 4];
    merge_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

