pub fn mid_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == *target {
            return Some(mid);
        } else if arr[mid] < *target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}
