fn main() {
    let mut arr = vec![1, 2, 23, 3, 4, 5, 6, 7, 1, 3, 4, 8, 9, 10];
    let high = arr.len().saturating_sub(1);
    // quick_sort(&mut arr, 0, high);
    for j in 0..arr.len() {
        println!("{}", arr[j]);
    }
}

