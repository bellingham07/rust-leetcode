fn merge_sort(arr: &mut [i32])  {
    if arr.len() <= 1 {
        return;
    }
    let mid=arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let merged = merge(&arr[..mid], &arr[mid..]);
    arr.copy_from_slice(&merged);
}

fn merge(left: &[i32],right:&[i32]) -> Vec<i32> {
    let mut res=Vec::with_capacity(left.len()+right.len());
    let (mut i, mut j) = (0, 0);
    while i<left.len() && j<right.len() {
        if left[i] < right[j] {
            res.push(left[i]);
            i += 1;
        }else {
            res.push(right[j]);
            j+=1;
        }
    }
    res.extend_from_slice(&left[i..]);
    res.extend_from_slice(&right[j..]);

    res
}