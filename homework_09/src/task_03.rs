// 3 Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.

pub fn get_element_from_slice<T>(slice: &[T], n: usize) -> Option<&T> {
    if n < slice.len() {
        Some(&slice[slice.len() - n - 1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element_from_slice() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[3..5];
        let result = get_element_from_slice(slice, 1).unwrap();

        let expected = &mut 4;

        assert_eq!(result, expected);
    }
}
