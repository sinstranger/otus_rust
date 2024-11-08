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
        let head_holder: Link<T> = self.head.take();
        let new_node: Node<T> = Node {
            value,
            next: head_holder,
        };
        let link_to_new_node: Link<T> = Some(Rc::new(RefCell::new(new_node)));
        self.head = link_to_new_node;
    }

    pub fn len(&self) -> usize {
        let mut counter: usize = 0;
        let mut current = self.head.clone();
        while let Some(node) = current {
            counter += 1;
            current = node.borrow().next.clone();
        }
        counter
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter{
            current: self.head.clone(),
        }
    }

}

pub struct ListIter<T> {
    current: Link<T>,
}

impl<T> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // let current = self.current.take();
        let current = self.current.clone();

        if let Some(value) = current {
            // self.current = current.borrow().
            None
        } else {
            None
        }

    }

    
    // fn next(&mut self) -> Option<Self::Item> {
    //     self.current.take().map(|node| {
    //         let node = node.borrow();
    //         self.current = node.next.clone();
    //         Rc::try_unwrap(Rc::clone(&node))
    //             .ok()
    //             .unwrap()
    //             .into_inner()
    //             .value
    //     })
    // }
}