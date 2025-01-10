// Insert sort. Stable
// Time: O(n^2) or O(n)
// The optimal time complexity of insertion sort is O(n) ,which is very efficient when the seqence is already sorted or nearly sorted.
// The worst-case and average time complexity of insertion sort are both O(n^2).
pub fn insert_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
