#[cfg(test)]
mod tests_push_head {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_common_case() {
        let mut empty_list: LinkedList<u32> = LinkedList::default();
        empty_list.push_head(1);
        empty_list.push_head(10);
        empty_list.push_head(100);
        let result: Vec<u32> = empty_list.iter().collect();
        let expected: Vec<u32> = vec![100, 10, 1];
        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod tests_push_tail {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_len_two() {
        let mut empty_list: LinkedList<u32> = LinkedList::default();
        empty_list.push_tail(1);
        empty_list.push_tail(10);
        empty_list.push_tail(100);
        let result: Vec<u32> = empty_list.iter().collect();
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
        let mut empty_list: LinkedList<u32> = LinkedList::default();
        empty_list.push_head(1);
        empty_list.push_head(1);
        empty_list.push_head(1);
        let len = empty_list.len();
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
        let mut empty_list: LinkedList<u32> = LinkedList::default();
        empty_list.push_head(1);
        empty_list.push_head(10);
        empty_list.push_head(100);
        let result: Vec<u32> = empty_list.iter().collect();
        let expected: Vec<u32> = vec![100, 10, 1];
        assert_eq!(result, expected);
    }
}
