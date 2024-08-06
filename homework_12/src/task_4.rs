pub struct TaskFour<'a, T> {
    data: &'a [T],
}

impl<'a, T> TaskFour<'a, T> {
    // 4. Принимает слайс и число N. Возвращает два слайса с элементами:
    // с нулевого по N-1;
    // с N-го по последний;
    pub fn get_two_elements_from_slice(&self, n: usize) -> (&[T], &[T]) {
        (
            &self.data[..&self.data.len() - n],
            &self.data[&self.data.len() - n..],
        )
    }
}

#[cfg(test)]
mod test_get_two_elements_from_slice {
    use super::*;

    #[test]
    fn test_get_two_elements_from_slice() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[..];
        let instance = TaskFour { data: slice };
        let result = instance.get_two_elements_from_slice(2);
        assert_eq!(result, (&[1, 2, 3][..], &[4, 5][..]));
    }

    #[test]
    #[should_panic]
    fn test_get_two_elements_from_slice_out_of_index() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[..];
        let instance = TaskFour { data: slice };
        let _result = instance.get_two_elements_from_slice(10);
    }
}
