#[path = "./sort/mod.rs"]
mod sort;
fn main() {
    // test quick sort
    // let mut arr = vec![5, 4, 3, 2, 1];
    // sort::quick_sort::quick_sort(&mut arr, 0, 4);
    // println!("{:?}", arr)

    // test merge sort
    let mut arr = vec![5, 4, 3, 2, 1];
    sort::merge_sort::merge_sort(&mut arr);
    println!("{:?}", arr)
}
