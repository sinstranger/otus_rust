#[cfg(test)]
mod tests_push_head {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_common_case() {
        let mut linked_list: LinkedList<u32> = LinkedList::default();
        linked_list.push_head(1);
        linked_list.push_head(10);
        linked_list.push_head(100);
        let result: Vec<u32> = linked_list.iter().collect();
        let expected: Vec<u32> = vec![100, 10, 1];
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_push_tail {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_len_two() {
        let mut linked_list: LinkedList<u32> = LinkedList::default();
        linked_list.push_tail(1);
        linked_list.push_tail(10);
        linked_list.push_tail(100);
        let result: Vec<u32> = linked_list.iter().collect();
        let expected: Vec<u32> = vec![1, 10, 100];
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_len {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_len_zero() {
        let empty_list: LinkedList<u32> = LinkedList::default();
        let len = empty_list.len();
        let expected: usize = 0;
        assert_eq!(len, expected);
    }

    #[test]
    fn test_len_two() {
        let mut linked_list: LinkedList<u32> = LinkedList::default();
        linked_list.push_head(1);
        linked_list.push_head(1);
        linked_list.push_head(1);
        let len = linked_list.len();
        let expected: usize = 3;
        assert_eq!(len, expected);
    }
}

#[cfg(test)]
mod tests_iter {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_len_zero() {
        let empty_list: LinkedList<u32> = LinkedList::default();
        let result: Vec<u32> = empty_list.iter().collect();
        let expected: Vec<u32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_len_two() {
        let mut linked_list: LinkedList<u32> = LinkedList::default();
        linked_list.push_head(1);
        linked_list.push_head(10);
        linked_list.push_head(100);
        let result: Vec<u32> = linked_list.iter().collect();
        let expected: Vec<u32> = vec![100, 10, 1];
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_split_by_index {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_len_zero() {
        let empty_list: LinkedList<u8> = LinkedList::default();

        let result = empty_list.split_by_index(4);
        let result_first: Vec<u8> = result.0.iter().collect();
        let result_second: Vec<u8> = result.1.iter().collect();

        let expected_first: Vec<u8> = vec![];
        let expected_second: Vec<u8> = vec![];

        assert_eq!(result_first, expected_first);
        assert_eq!(result_second, expected_second);
    }

    #[test]
    fn test_common_case() {
        let mut linked_list: LinkedList<u8> = LinkedList::default();
        let values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for i in values {
            linked_list.push_head(i);
        }
        let result = linked_list.split_by_index(4);
        let result_first: Vec<u8> = result.0.iter().collect();
        let result_second: Vec<u8> = result.1.iter().collect();

        let expected_first = vec![10, 9, 8, 7];
        let expected_second = vec![6, 5, 4, 3, 2, 1];

        assert_eq!(result_first, expected_first);
        assert_eq!(result_second, expected_second);
    }
}
