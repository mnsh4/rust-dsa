fn linear_search(item: &T, arr: &[T]) -> i32 {
    let mut idx_pos = -1; 

    for (idx, &data) in arr.iter().enumerate() {
        if item == data {
            idx_pos = idx as i32;
            return idx_pos;
        }
    }
    idx_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_found() {
        let arr = vec![4, 2, 7, 1, 9, 5];
        let target = &7;

        assert_eq!(linear_search(&arr, target), Some(2));
    }

    #[test]
    fn test_linear_search_not_found() {
        let arr = vec!["Argentina", "Japon", "Paraguay"];
        let target = &"Japon"; 

        assert_eq!(linear_search(&arr, target), None);
    }
}
