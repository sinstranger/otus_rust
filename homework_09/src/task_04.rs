// Принимает слайс и число N. Возвращает два слайса с элементами:
// с нулевого по N-1;
// с N-го по последний;

pub fn get_two_elements_from_slice<T>(slice: &[T], n: usize) -> Option<(&[T], &[T])> {
    if n < slice.len() {
        Some((&slice[..slice.len() - n - 1], &slice[slice.len() - n - 1..]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_two_elements_from_slice() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[..];
        let result = get_two_elements_from_slice(slice, 2).unwrap();

        let expected_1 = &slice[..2];
        let expected_2 = &slice[2..];

        assert_eq!(result, (expected_1, expected_2));
    }
}
