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

impl<T: Debug + Copy> LinkedList<T> {
    // changes LinkedList head to new node
    pub fn push_head(&mut self, value: T) {
        let head_holder: Link<T> = self.head.take();
        let new_node: Node<T> = Node {
            value,
            next: head_holder,
        };
        let link_to_new_node: Link<T> = Some(Rc::new(RefCell::new(new_node)));
        self.head = link_to_new_node;
    }

// Добавить элемент в конец списка
pub fn push_tail(&mut self, value: T) {
    let new_node = Rc::new(RefCell::new(Node { value, next: None }));

    match &self.head {
        None => self.head = Some(Rc::clone(&new_node)),

        Some(node) => {
            let mut current = Rc::clone(node);
            loop {
                let next_node_link = current.borrow().next.clone();
                match next_node_link {
                    None => {
                        current.borrow_mut().next = Some(Rc::clone(&new_node));
                        break;
                    }
                    Some(next_node) => {
                        current = next_node;
                    }
                }
            }
        }
    }
}

    // returns length of LinkedList iteratively
    pub fn len(&self) -> usize {
        let mut counter: usize = 0;
        let mut current = self.head.clone();
        while let Some(node) = current {
            counter += 1;
            current = node.borrow().next.clone();
        }
        counter
    }

    // returns iterator for LinkedList
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.clone(),
        }
    }
}

pub struct ListIter<T: Copy> {
    current: Link<T>,
}

impl<T: Copy> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();

        if let Some(node) = current {
            let next = node.borrow().next.clone();
            self.current = next;

            let x = node.borrow().value.clone();
            Some(x)
        } else {
            None
        }
    }
}
