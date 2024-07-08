// Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.

pub fn split_into_four_equal_parts<T>(slice: &[T]) -> [&[T]; 4] {
    let len = slice.len();
    let part_size = len / 4;
    let remainder = len % 4;

    // Calculate the sizes of the four parts
    let mut parts_sizes = [part_size; 4];
    for i in parts_sizes.iter_mut().take(remainder) {
        *i += 1;
    }

    // Create slices for each part
    let mut result = [slice; 4];
    let mut start = 0;
    for i in 0..4 {
        if parts_sizes[i] > 0 {
            let end = start + parts_sizes[i];
            result[i] = &slice[start..end];
            start = end;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_four_equal_parts() {
        let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110];
        let parts = split_into_four_equal_parts(&arr);
        let expected = [&arr[..3], &arr[3..6], &arr[6..9], &arr[9..]];
        assert_eq!(parts, expected);
    }
}
