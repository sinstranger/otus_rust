// Принимает слайс и число N. Возвращает два слайса с элементами:
// с нулевого по N-1;
// с N-го по последний;

pub fn get_two_elements_from_slice<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
    (&slice[..slice.len() - n], &slice[slice.len() - n..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_two_elements_from_slice() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[..];
        let result = get_two_elements_from_slice(slice, 2);
        assert_eq!(result, (&[1, 2, 3][..], &[4, 5][..]));
    }

    #[test]
    #[should_panic]
    fn test_get_two_elements_from_slice_out_of_index() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[..];
        let result = get_two_elements_from_slice(slice, 10);
        assert_eq!(result, (&[1, 2, 3][..], &[4, 5][..]));
    }
}
