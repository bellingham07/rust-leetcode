mod quick_sort;

fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);
        if pivot > 0 {
            quick_sort(arr, low, pivot - 1);
        }
        quick_sort(arr, pivot + 1, high);
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