use std::{
    cell::RefCell,
    rc::Rc
};

struct Node<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
    pub value: Option<T>
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            next: None,
            value: Some(value)
        }
    }
}

struct SLL<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    len: usize
}

impl<T> SLL<T> {
    pub fn new() -> SLL<T> {
        SLL {
            head: None,
            len: 0
        }
    }

    pub fn add(&mut self, value: T) {

    }

    pub fn append(&mut self, value: T) {

    }

    pub fn insert(&mut self, value: T, position: usize) {

    }

    pub fn remove(&mut self, position: usize) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
