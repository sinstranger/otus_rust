pub struct TaskTwo<'a, T> {
    data: &'a mut [T],
}

impl<'a, T> TaskTwo<'a, T> {
    // 2 Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
    pub fn get_slice_element_from_start(&mut self, n: usize) -> &mut T {
        &mut self.data[n]
    }
}

#[cfg(test)]
mod test_get_slice_element_from_start {
    use super::*;

    #[test]
    fn test_get_element_from_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..5];
        let mut instance = TaskTwo { data: slice };
        let result = instance.get_slice_element_from_start(1);
        let expected = &mut 5;
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_slice_out_of_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..5];
        let mut instance = TaskTwo { data: slice };
        let _result = instance.get_slice_element_from_start(10);
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_slice_empty_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..3];
        let mut instance = TaskTwo { data: slice };
        let _result = instance.get_slice_element_from_start(10);
    }
}
