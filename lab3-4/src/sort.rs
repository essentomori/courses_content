use std::cmp::Reverse;

pub fn sort_with_closure(data: &[i32]) -> Vec<i32> {
    let mut result = data.to_vec();
    result.sort_by_key(|value| Reverse(value.unsigned_abs()));
    result
}

fn abs_descending(value: &i32) -> Reverse<u32> {
    Reverse(value.unsigned_abs())
}

pub fn sort_with_function(data: &[i32]) -> Vec<i32> {
    let mut result = data.to_vec();
    result.sort_by_key(abs_descending);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_versions_preserve_order_for_equal_magnitudes() {
        let data = vec![4, -30, 30, 100, -100, 123, 1, 0, -1, -4];
        let original = data.clone();
        let expected = vec![123, 100, -100, -30, 30, 4, -4, 1, -1, 0];
        assert_eq!(sort_with_closure(&data), expected);
        assert_eq!(sort_with_function(&data), expected);
        assert_eq!(data, original);
    }

    #[test]
    fn minimum_integer_does_not_overflow() {
        let data = [0, i32::MAX, i32::MIN, -1];
        let expected = vec![i32::MIN, i32::MAX, -1, 0];
        assert_eq!(sort_with_closure(&data), expected);
        assert_eq!(sort_with_function(&data), expected);
    }
}
