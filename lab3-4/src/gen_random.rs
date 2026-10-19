pub fn gen_random(
    count: usize,
    min: i32,
    max: i32,
) -> Result<impl Iterator<Item = i32>, &'static str> {
    if min > max {
        return Err("Нижняя граница диапазона больше верхней");
    }

    Ok((0..count).map(move |_| rand::random_range(min..=max)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_and_bounds_are_respected() {
        let values: Vec<_> = gen_random(1000, -3, 7).unwrap().collect();
        assert_eq!(values.len(), 1000);
        assert!(values.iter().all(|value| (-3..=7).contains(value)));
    }

    #[test]
    fn zero_count_is_empty() {
        assert_eq!(gen_random(0, 1, 3).unwrap().next(), None);
    }

    #[test]
    fn equal_bounds_produce_constant_values() {
        let values: Vec<_> = gen_random(4, 7, 7).unwrap().collect();
        assert_eq!(values, vec![7; 4]);
    }

    #[test]
    fn reversed_bounds_are_rejected() {
        assert!(gen_random(10, 3, 1).is_err());
        assert!(gen_random(0, 3, 1).is_err());
    }
}
