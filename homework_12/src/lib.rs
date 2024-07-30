mod tests;

pub struct FunctionsBox;

impl FunctionsBox {
    // 1 Принимает мутабельную ссылку на кортеж и bool значение.
    // Если false, возвращает мутабельную ссылку на первый элемент кортежа.
    // Если true, возвращает мутабельную ссылку на второй элемент кортежа.
    pub fn get_element_from_tuple<T>(container: &mut (T, T), get_last: bool) -> &mut T {
        if get_last {
            &mut container.1
        } else {
            &mut container.0
        }
    }

    // 2 Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
    pub fn get_slice_element_from_start<T>(slice: &mut [T], n: usize) -> &mut T {
        &mut slice[n]
    }

    // 3 Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
    pub fn get_slice_element_from_end<T>(slice: &[T], n: usize) -> &T {
        &slice[slice.len() - n - 1]
    }

    // 4. Принимает слайс и число N. Возвращает два слайса с элементами:
    // с нулевого по N-1;
    // с N-го по последний;
    pub fn get_two_elements_from_slice<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
        (&slice[..slice.len() - n], &slice[slice.len() - n..])
    }

    // 5 Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
    pub fn split_into_four_equal_parts<T>(slice: &[T]) -> [&[T]; 4] {
        let len = slice.len();
        if len < 4 {
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
}
