use crate::FunctionsBox;

#[cfg(test)]
mod test_get_element_from_tuple {
    use super::*;

    #[test]
    fn test_get_element_from_tuple_false() {
        let mut container = (1, 2);
        let result = FunctionsBox::get_element_from_tuple(&mut container, false);
        let expected = &mut 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_element_from_tuple_true() {
        let mut container = (1, 2);
        let result = FunctionsBox::get_element_from_tuple(&mut container, true);
        let expected = &mut 2;
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_get_slice_element_from_start {
    use super::*;

    #[test]
    fn test_get_element_from_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..5];
        let result = FunctionsBox::get_slice_element_from_start(slice, 1);
        let expected = &mut 5;
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_slice_out_of_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..5];
        let _result = FunctionsBox::get_slice_element_from_start(slice, 10);
    }

    #[test]
    #[should_panic]
    fn test_get_element_from_slice_empty_slice() {
        let mut container = [1, 2, 3, 4, 5];
        let slice = &mut container[3..3];
        let _result = FunctionsBox::get_slice_element_from_start(slice, 10);
    }
}

#[cfg(test)]
mod test_get_slice_element_from_end {
    use super::*;

    #[test]
    fn test_get_slice_element_from_end() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[3..];
        let result = FunctionsBox::get_slice_element_from_end(slice, 1);
        let expected = &mut 4;
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_get_slice_element_from_end_out_of_index() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[3..];
        let _result = FunctionsBox::get_slice_element_from_end(slice, 10);
    }
}

#[cfg(test)]
mod test_get_two_elements_from_slice {
    use super::*;

    #[test]
    fn test_get_two_elements_from_slice() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[..];
        let result = FunctionsBox::get_two_elements_from_slice(slice, 2);
        assert_eq!(result, (&[1, 2, 3][..], &[4, 5][..]));
    }

    #[test]
    #[should_panic]
    fn test_get_two_elements_from_slice_out_of_index() {
        let container = [1, 2, 3, 4, 5];
        let slice = &container[..];
        let _result = FunctionsBox::get_two_elements_from_slice(slice, 10);
    }
}

#[cfg(test)]
mod test_split_into_four_equal_parts {
    use super::*;

    #[test]
    fn test_split_into_four_equal_parts() {
        let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110];
        let parts = FunctionsBox::split_into_four_equal_parts(&arr);
        let expected = [&arr[..3], &arr[3..6], &arr[6..9], &arr[9..]];
        assert_eq!(parts, expected);
    }

    #[test]
    #[should_panic]
    fn test_split_into_four_equal_parts_small_array() {
        let arr = [10, 20];
        let _parts = FunctionsBox::split_into_four_equal_parts(&arr);
    }
}
