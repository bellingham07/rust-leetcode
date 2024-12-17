// mod sort;
mod find;
fn main() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let mut arr = vec![1, 2, 23, 3, 4, 5, 6, 7, 1, 3, 4, 8, 9, 10];
    // let high = arr.len().saturating_sub(1);
    // sort::quick_sort::quick_sort(&mut arr, 0, high);
    // // merge_sort(&mut arr);
    // for j in 0..arr.len() {
    //     println!("{}", arr[j]);
    // }
    let target = 5;
    println!(
        "res:{}",
        find::mid_search::mid_search(&arr, &target).unwrap()
    )
}
