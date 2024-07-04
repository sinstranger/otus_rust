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

pub fn get_element_from_slice<T>(slice: &mut [T], n: usize) -> Option<&mut T> {
    if n < slice.len() {
        Some(&mut slice[n])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element_from_tuple() {
        let mut container = (1, 2);
        let result = get_element_from_tuple(&mut container, false);

        let expected = &mut 1;

        assert_eq!(result, expected);
    }


    #[test]
    fn test_get_element_from_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..5];
        let result = get_element_from_slice(slice, 1).unwrap();

        let expected = &mut 5;

        assert_eq!(result, expected);
    }
}
