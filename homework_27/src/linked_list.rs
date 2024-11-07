use std::cell::RefCell;
use std::default::Default;
use std::fmt::Debug;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList { head: None }
    }
}

impl<T: Debug> LinkedList<T> {
    pub fn push_head(&mut self, value: T) {
        let head_holder = self.head.take();
        let new_node = Node {
            value,
            next: head_holder,
        };
        let link_to_new_node: Link<T> = Some(Rc::new(RefCell::new(new_node)));
        self.head = link_to_new_node;
    }
}
