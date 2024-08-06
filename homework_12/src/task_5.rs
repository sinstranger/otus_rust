pub struct TaskFive<'a, T> {
    data: &'a [T],
}

impl<'a, T> TaskFive<'a, T> {
    // 5 Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
    pub fn split_into_four_equal_parts(&self) -> [&[T]; 4] {
        let len = &self.data.len();
        if len < &4 {
            panic!()
        }
        let part_size = len / 4;
        let remainder = len % 4;

        // Calculate the sizes of the four parts
        let mut parts_sizes = [part_size; 4];
        for i in parts_sizes.iter_mut().take(remainder) {
            *i += 1;
        }

        // Create slices for each part
        let mut result = [self.data; 4];
        let mut start = 0;
        for i in 0..4 {
            if parts_sizes[i] > 0 {
                let end = start + parts_sizes[i];
                result[i] = &self.data[start..end];
                start = end;
            }
        }

        result
    }
}

#[cfg(test)]
mod test_split_into_four_equal_parts {
    use super::*;

    #[test]
    fn test_split_into_four_equal_parts() {
        let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110];
        let instance = TaskFive { data: &arr };
        let parts = instance.split_into_four_equal_parts();
        let expected = [&arr[..3], &arr[3..6], &arr[6..9], &arr[9..]];
        assert_eq!(parts, expected);
    }

    #[test]
    #[should_panic]
    fn test_split_into_four_equal_parts_small_array() {
        let arr = [10, 20];
        let instance = TaskFive { data: &arr };
        let _parts = instance.split_into_four_equal_parts();
    }
}
