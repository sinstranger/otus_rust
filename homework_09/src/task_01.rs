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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element_from_tuple_false() {
        let mut container = (1, 2);
        let result = get_element_from_tuple(&mut container, false);

        let expected = &mut 1;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_element_from_tuple_true() {
        let mut container = (1, 2);
        let result = get_element_from_tuple(&mut container, true);

        let expected = &mut 2;

        assert_eq!(result, expected);
    }
}
