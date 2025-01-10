pub fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pi = partition(arr, low, high);
        if pi > 0 {
            quick_sort(arr, low, pi - 1);
        }
        quick_sort(arr, pi + 1, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let mut i = low;
    for j in low..high {
        if arr[j] < arr[high] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}
// todo 这里的测试执行不到
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_quick_sort() {
//         let mut arr = [10, 7, 8, 9, 1, 5];
//         super::quick_sort(&mut arr, 0, arr.len() - 1);
//         assert_eq!(arr, [1, 5, 7, 8, 9, 10]);
//     }
// }
