#[path = "./sort/mod.rs"]
mod sort;
fn main() {
    // test quick sort
    // let mut arr = vec![5, 4, 3, 2, 1];
    // sort::quick_sort::quick_sort(&mut arr, 0, 4);
    // println!("{:?}", arr)

    // test merge sort
    // let mut arr = vec![5, 4, 3, 2, 1];
    // sort::merge_sort::merge_sort(&mut arr);
    // println!("{:?}", arr)

    // test bubble sort
    // let mut arr = vec![5, 4, 3, 2, 1];
    // sort::bubble_sort::bubble_sort(&mut arr);
    // println!("{:?}", arr)

    // test select sort
    // let mut arr = vec![5, 4, 3, 2, 1];
    // sort::select_sort::select_sort(&mut arr);
    // println!("{:?}", arr)

    // test insert sort
    let mut arr = vec![5, 4, 3, 2, 1];
    sort::insert_sort::insert_sort(&mut arr);
    println!("{:?}", arr)

    // let mut arr = vec![5, 4, 3, 2, 1];
    // println!("{:?}", arr)

    // let mut arr = vec![5, 4, 3, 2, 1];
    // println!("{:?}", arr)

    // let mut arr = vec![5, 4, 3, 2, 1];
    // println!("{:?}", arr)
}
