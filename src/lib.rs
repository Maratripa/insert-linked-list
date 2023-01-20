use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
    pub value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            next: None,
            prev: None,
            value: value,
        }
    }
}
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    pub len: usize,
}

pub struct Iter<'a, T: 'a> {
    curr: Option<Rc<RefCell<Node<T>>>>,
    marker: PhantomData<&'a RefCell<Node<T>>>,
}

struct IterNode<'a, T: 'a> {
    curr: Option<Rc<RefCell<Node<T>>>>,
    marker: PhantomData<&'a RefCell<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Create an empty simply linked list
    ///
    /// # Examples
    /// ```
    /// use insert_linked_list::LinkedList;
    ///
    /// let linkedlist: LinkedList<i16> = LinkedList::new();
    /// ```
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Add a node at the beginning of the list
    ///
    /// # Examples
    /// ```
    /// use insert_linked_list::LinkedList;
    ///
    /// let mut linkedlist: LinkedList<i16> = LinkedList::new();
    /// linkedlist.add(1);
    /// linkedlist.add(2);
    /// linkedlist.add(3);
    ///
    /// // Linked list looks like this:
    /// // 3 -> 2 -> 1
    ///
    /// assert_eq!(3, linkedlist.len);
    /// ```
    pub fn add(&mut self, value: T) {
        match &self.head {
            Some(node) => {
                let new_node = Node {
                    next: Some(Rc::clone(node)),
                    prev: None,
                    value: value,
                };
                let node_ref = Rc::new(RefCell::new(new_node));
                node.borrow_mut().prev = Some(Rc::downgrade(&node_ref));
                self.head = Some(Rc::clone(&node_ref));
                self.len = self.len + 1;
            }
            None => {
                let new_node = Node::new(value);
                let node_ref = Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(&node_ref));
                self.tail = Some(Rc::clone(&node_ref));
                self.len = 1;
            }
        }
    }

    /// Add a node at the end of the list
    ///
    /// # Examples
    /// ```
    /// use insert_linked_list::LinkedList;
    ///
    /// let mut linkedlist: LinkedList<i16> = LinkedList::new();
    /// linkedlist.append(1);
    /// linkedlist.append(2);
    /// linkedlist.append(3);
    ///
    /// // Linked list looks like this:
    /// // 1 -> 2 -> 3
    ///
    /// assert_eq!(3, linkedlist.len);
    /// ```
    pub fn append(&mut self, value: T) {
        match &self.tail {
            Some(node) => {
                let new_node = Node {
                    next: None,
                    prev: Some(Rc::downgrade(node)),
                    value: value,
                };
                let node_ref = Rc::new(RefCell::new(new_node));
                node.borrow_mut().next = Some(Rc::clone(&node_ref));
                self.tail = Some(Rc::clone(&node_ref));
                self.len = self.len + 1;
            }
            None => {
                let new_node = Node::new(value);
                let node_ref = Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(&node_ref));
                self.tail = Some(Rc::clone(&node_ref));
                self.len = 1;
            }
        }
    }

    pub fn insert(&mut self, value: T, position: usize) {
        if position > self.len || self.head.is_none() {
            panic!()
        }

        let mut counter = 0;

        for node in self.iter_nodes() {
            if counter == position { unsafe {
                let node_ref = Rc::clone(&node);
                let new_node = Node {
                    next: match &(*node_ref.as_ptr()).next {
                        Some(next) => Some(Rc::clone(next)),
                        None => None
                    },
                    prev: Some(Rc::downgrade(&node)),
                    value,
                };
                node.borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
                break;
            }
            }

            counter += 1;
        }
    }

    pub fn remove(&mut self, position: usize) {}

    pub fn iter(&self) -> Iter<'_, T> {
        match &self.head {
            Some(node) => Iter {
                curr: Some(Rc::clone(node)),
                marker: PhantomData,
            },
            None => Iter {
                curr: None,
                marker: PhantomData,
            },
        }
    }

    fn iter_nodes(&self) -> IterNode<'_, T> {
        match &self.head {
            Some(node) => IterNode {
                curr: Some(Rc::clone(node)),
                marker: PhantomData,
            },
            None => IterNode {
                curr: None,
                marker: PhantomData,
            },
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.curr.is_none() {
            None
        } else {
            unsafe {
                let node = Rc::clone(self.curr.as_ref().unwrap());
                self.curr = match &(*node.as_ptr()).next {
                    Some(next) => Some(Rc::clone(next)),
                    None => None,
                };

                Some(&(*node.as_ptr()).value)
            }
        }
    }
}

impl<'a, T> Iterator for IterNode<'a, T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        if self.curr.is_none() {
            None
        } else {
            unsafe {
                let mut node = Rc::clone(self.curr.as_ref().unwrap());
                self.curr = match &(*node.as_ptr()).next {
                    Some(next) => Some(Rc::clone(next)),
                    None => None,
                };

                Some(node)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {}
}
