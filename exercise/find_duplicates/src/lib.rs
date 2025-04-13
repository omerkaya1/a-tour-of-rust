pub fn unique<T: Ord>(mut input: Vec<T>) -> Vec<T> {
    input.sort();
    input.dedup();
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_unique() {
        let result = unique(vec![1, 2, 3]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn all_unique_unsorted() {
        let result = unique(vec![5, 1, 2, 3]);
        assert_eq!(result, vec![1, 2, 3, 5]);
    }

    #[test]
    fn duplicates_unsorted() {
        let result = unique(vec![5, 5, 1, 2, 2, 3]);
        assert_eq!(result, vec![1, 2, 3, 5]);
    }

    #[test]
    fn duplicates_unsorted_str() {
        let result = unique(vec!["1", "5", "1", "2", "2", "3"]);
        assert_eq!(result, vec!["1", "2", "3", "5"]);
    }
}
