// 2 Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.

pub fn get_element_from_slice<T>(slice: &mut [T], n: usize) -> &mut T {
    &mut slice[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element_from_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..5];
        let result = get_element_from_slice(slice, 1);

        let expected = &mut 5;

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_slice_out_of_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..5];
        let result = get_element_from_slice(slice, 10);

        let expected = &mut 5;

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_slice_empty_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..3];
        let result = get_element_from_slice(slice, 10);

        let expected = &mut 5;

        assert_eq!(result, expected);
    }
}
