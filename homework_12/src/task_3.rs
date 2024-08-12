pub struct TaskThree<'a, T> {
    data: &'a [T],
}

impl<'a, T> TaskThree<'a, T> {
    // 3 Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
    pub fn get_slice_element_from_end(&self, n: usize) -> &T {
        &self.data[&self.data.len() - n - 1]
    }
}

#[cfg(test)]
mod test_get_slice_element_from_end {
    use super::*;

    #[test]
    fn test_get_slice_element_from_end() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[3..];
        let instance = TaskThree { data: slice };
        let result = instance.get_slice_element_from_end(1);
        let expected = &mut 4;
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_get_slice_element_from_end_out_of_index() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[3..];
        let instance = TaskThree { data: slice };
        let _result = instance.get_slice_element_from_end(10);
    }
}
