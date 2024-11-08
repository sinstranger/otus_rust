#[cfg(test)]
mod tests_push_head {
    use crate::linked_list::LinkedList;

    #[test]
    fn test_common_case() {
        let mut empty_list: LinkedList<u32> = LinkedList::default();
        empty_list.push_head(1);
        let string = format!("{:?}", empty_list);
        let expected =
            "LinkedList { head: Some(RefCell { value: Node { value: 1, next: None } }) }"
                .to_string();
        assert_eq!(string, expected);
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
