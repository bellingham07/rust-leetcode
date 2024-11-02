// fn main() {
//     let mut arr = vec![1, 2, 23, 3, 4, 5, 6, 7, 1, 3, 4, 8, 9, 10];
//     let high = arr.len().saturating_sub(1); // 使用 saturating_sub 防止溢出
//     quick_sort(&mut arr, 0, high);
//     for j in 0..arr.len() {
//         println!("{}", arr[j]);
//     }
// }
//
// fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
//     if low < high {
//         let pivot = partition(arr, low, high);
//         if pivot > 0 { // 确保 pivot-1 不会越界
//             quick_sort(arr, low, pivot - 1);
//         }
//         quick_sort(arr, pivot + 1, high);
//     }
// }
//
// fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
//     let pivot = arr[right];
//     let mut i = left;
//     for j in left..right {
//         if arr[j] < pivot {
//             arr.swap(i, j);
//             i += 1;
//         }
//     }
//     arr.swap(i, right);
//     i
// }