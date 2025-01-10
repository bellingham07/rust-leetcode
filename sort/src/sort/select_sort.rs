// Selection sort. O(n^2)
// Stable sort.
pub fn select_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min = i;
        // Find the minimum element in the unsorted part of the array.
        // Put it at the beginning of the unsorted part.
        for j in i + 1..len {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}
