#[cfg(test)]
mod tests {

    use super::*;
    use crate::sort::{merge_sort, quick_sort};

    #[test]
    fn test_mid_search() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let target = &5;
        assert_eq!(mid_search(&arr, target), Some(4));
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![3, 2, 1, 5, 4];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 3, 8, 4, 2];
        quick_sort(&mut arr, 0, arr.len() - 1);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }
}
