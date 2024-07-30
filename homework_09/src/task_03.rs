// 3 Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.

pub fn get_element_from_slice<T>(slice: &[T], n: usize) -> &T {
    &slice[slice.len() - n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element_from_slice() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[3..];
        let result = get_element_from_slice(slice, 1);

        let expected = &mut 4;

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_slice_out_of_index() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[3..];
        let result = get_element_from_slice(slice, 10);

        let expected = &mut 4;

        assert_eq!(result, expected);
    }
}
