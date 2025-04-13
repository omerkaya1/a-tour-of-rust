pub fn median(mut input: Vec<f32>) -> Option<f32> {
    if input.is_empty() {
        return None;
    }

    input.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let size = input.len();
    let middle = size / 2;

    let median = if size % 2 == 0 {
        (input[middle - 1] + input[middle]) / 2.0
    } else {
        input[middle]
    };
    Some(median)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = median(vec![1.0, 2.0, 3.0]);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn empty_input() {
        let result = median(vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn even_num_of_elements() {
        let result = median(vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(result, Some(2.5))
    }

    #[test]
    fn unsorted_even_num_of_elements() {
        let result = median(vec![3.0, 1.0, 2.0, 4.0]);
        assert_eq!(result, Some(2.5))
    }
}
