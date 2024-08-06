pub struct TaskOne<'a, T> {
    data: &'a mut (T, T),
}

impl<'a, T> TaskOne<'a, T> {
    // 1 Принимает мутабельную ссылку на кортеж и bool значение.
    // Если false, возвращает мутабельную ссылку на первый элемент кортежа.
    // Если true, возвращает мутабельную ссылку на второй элемент кортежа.
    pub fn get_element_from_tuple(&mut self, get_last: bool) -> &mut T {
        if get_last {
            &mut self.data.1
        } else {
            &mut self.data.0
        }
    }
}

#[cfg(test)]
mod test_get_element_from_tuple {
    use super::*;

    #[test]
    fn test_get_element_from_tuple_false() {
        let mut tuple = (1, 2);
        let mut container = TaskOne { data: &mut tuple };
        let result = container.get_element_from_tuple(false);
        let expected = &mut 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_element_from_tuple_true() {
        let mut tuple = (1, 2);
        let mut container = TaskOne { data: &mut tuple };
        let result = container.get_element_from_tuple(true);
        let expected = &mut 2;
        assert_eq!(result, expected);
    }
}
