#![allow(dead_code)]

use std::cell::RefCell;
use std::iter::{FromIterator, IntoIterator};
use std::rc::Rc;

/// Linked list errors.
pub enum LinkedListError {
    IndexOutOfBounds,
    InvalidArraySize,
}

impl std::fmt::Debug for LinkedListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IndexOutOfBounds => write!(f, "linked list index out of bounds"),
            Self::InvalidArraySize => write!(f, "invalid array size"),
        }
    }
}

/// Linked list result type.
pub type Result<T> = core::result::Result<T, LinkedListError>;

/// A node in a linked list.
#[derive(Clone, Debug)]
struct Node<T> {
    /// The node's value.
    value: T,
    /// The next node in the linked list.
    next: Option<Rc<RefCell<Node<T>>>>,
    /// The previous node in the linked list.
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    // /// Get a reference to the value at a node by index.
    // pub fn get_value(&self, index: usize) -> Result<Ref<T>> {
    //     match &self.next {
    //         Some(next) => {
    //             let next_ref = next.borrow();

    //             if index == 1 {
    //                 Ok(Ref::map(next_ref, |node| &node.value))
    //             } else {
    //                 next_ref.get_value(index - 1)
    //             }
    //         }
    //         None => Err(LinkedListError::IndexOutOfBounds),
    //     }
    // }

    // /// Get a mutable reference to the value at a node by index.
    // pub fn get_value_mut(&mut self, index: usize) -> Result<RefMut<T>> {
    //     match &self.next {
    //         Some(next) => {
    //             let mut next_ref = next.borrow_mut();

    //             if index == 1 {
    //                 Ok(RefMut::map(next_ref, |node| &mut node.value))
    //             } else {
    //                 next_ref.get_value_mut(index - 1)
    //             }
    //         }
    //         None => Err(LinkedListError::IndexOutOfBounds),
    //     }
    // }

    /// Set the value at a node by index.
    pub fn set_value(&mut self, index: usize, value: T) -> Result<()> {
        if index == 0 {
            self.value = value;
            Ok(())
        } else {
            match &self.next {
                Some(next) => next.borrow_mut().set_value(index - 1, value),
                None => Err(LinkedListError::IndexOutOfBounds),
            }
        }
    }

    /// Insert a value in a new node at a given index.
    pub fn push(&mut self, index: usize, value: T) -> Result<()> {
        if index == 1 {
            if self.next.is_some() {
                let next = self.next.take().unwrap();
                let mut orig_next = next.borrow_mut();
                let new_node = Rc::new(RefCell::new(Node {
                    value,
                    next: Some(Rc::clone(&next)),
                    prev: Some(Rc::clone(&orig_next.prev.take().unwrap())),
                }));
                orig_next.prev = Some(Rc::clone(&new_node));
                self.next = Some(new_node);

                Ok(())
            } else {
                Err(LinkedListError::IndexOutOfBounds)
            }
        } else {
            match &self.next {
                Some(next) => next.borrow_mut().push(index - 1, value),
                None => Err(LinkedListError::IndexOutOfBounds),
            }
        }
    }

    /// Remove the node at a given index, returning the node's owned value.
    pub fn pop(&mut self, index: usize) -> Result<T> {
        if index == 1 {
            if self.next.is_some() {
                let pop_node_ref = self.next.take().unwrap();
                {
                    let mut pop_node_borrow = pop_node_ref.borrow_mut();
                    let new_next_ref = pop_node_borrow.next.take().unwrap();
                    {
                        let mut new_next = new_next_ref.borrow_mut();
                        new_next.prev = pop_node_borrow.prev.take();
                    }
                    self.next = Some(new_next_ref);
                }
                let pop_node = Rc::try_unwrap(pop_node_ref)
                    .map_err(|_| "attempted to unwrap Rc with multiple references".to_owned())
                    .unwrap();
                let value = pop_node.into_inner().value;

                Ok(value)
            } else {
                Err(LinkedListError::IndexOutOfBounds)
            }
        } else {
            match &self.next {
                Some(next) => next.borrow_mut().pop(index - 1),
                None => Err(LinkedListError::IndexOutOfBounds),
            }
        }
    }
}

impl<T: Copy> Node<T> {
    /// Gets a copy of the value at a given index.
    pub fn get(&self, index: usize) -> Result<T> {
        if index == 0 {
            Ok(self.value)
        } else {
            match &self.next {
                Some(next) => next.borrow().get(index - 1),
                None => Err(LinkedListError::IndexOutOfBounds),
            }
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.value != other.value {
            false
        } else if self.next.is_some() && other.next.is_some() {
            *self.next.as_ref().unwrap().borrow() == *other.next.as_ref().unwrap().borrow()
        } else if self.next.is_none() && other.next.is_none() {
            true
        } else {
            false
        }
    }
}

/// A linked list.
#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    /// The first node in the linked list.
    head: Option<Rc<RefCell<Node<T>>>>,
    /// The last node in the linked list.
    tail: Option<Rc<RefCell<Node<T>>>>,
    /// The total number of nodes in the linked list.
    size: usize,
}

impl<T> LinkedList<T> {
    /// Create an empty linked list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// Get the number of nodes in the linked list.
    pub fn len(&self) -> usize {
        self.size
    }

    // /// Get a reference to the value at a node by index.
    // pub fn get_value(&self, index: usize) -> Result<Ref<T>> {
    //     match &self.head {
    //         Some(head) => {
    //             let head_ref = head.borrow();

    //             if index == 1 {
    //                 Ok(Ref::map(head_ref, |node| &node.value))
    //             } else {
    //                 head_ref.get_value(index)
    //             }
    //         }
    //         None => Err(LinkedListError::IndexOutOfBounds),
    //     }
    // }

    // /// Get a mutable reference to the value at a node by index.
    // pub fn get_value_mut(&mut self, index: usize) -> Result<RefMut<T>> {
    //     match &self.head {
    //         Some(head) => {
    //             let mut head_ref = head.borrow_mut();

    //             if index == 1 {
    //                 Ok(RefMut::map(head_ref, |node| &mut node.value))
    //             } else {
    //                 head_ref.get_value_mut(index)
    //             }
    //         }
    //         None => Err(LinkedListError::IndexOutOfBounds),
    //     }
    // }

    /// Set the value at a node by index.
    pub fn set_value(&mut self, index: usize, value: T) -> Result<()> {
        match &self.head {
            Some(head) => head.borrow_mut().set_value(index, value),
            None => Err(LinkedListError::IndexOutOfBounds),
        }
    }

    /// Insert a value in a new node at a given index in the linked list.
    pub fn push(&mut self, index: usize, value: T) -> Result<()> {
        if self.size == 0 {
            if index == 0 {
                let new_node = Rc::new(RefCell::new(Node {
                    value,
                    next: None,
                    prev: None,
                }));
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
                self.size += 1;

                Ok(())
            } else {
                Err(LinkedListError::IndexOutOfBounds)
            }
        } else if index == 0 {
            self.push_front(value);

            Ok(())
        } else if index == self.size {
            self.push_back(value);

            Ok(())
        } else {
            match self.head.as_ref().unwrap().borrow_mut().push(index, value) {
                Ok(x) => {
                    self.size += 1;
                    Ok(x)
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Insert a value at the start of the linked list.
    pub fn push_front(&mut self, value: T) {
        if self.size == 0 {
            let new_node = Rc::new(RefCell::new(Node {
                value,
                next: None,
                prev: None,
            }));
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        } else {
            let orig_head = self.head.take().unwrap();
            let new_node = Rc::new(RefCell::new(Node {
                value,
                next: Some(Rc::clone(&orig_head)),
                prev: None,
            }));
            orig_head.borrow_mut().prev = Some(Rc::clone(&new_node));
            self.head = Some(new_node);
        }

        self.size += 1;
    }

    /// Insert a value at the end of the linked list.
    pub fn push_back(&mut self, value: T) {
        if self.size == 0 {
            let new_node = Rc::new(RefCell::new(Node {
                value,
                next: None,
                prev: None,
            }));
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        } else {
            let orig_tail = self.tail.take().unwrap();
            let new_node = Rc::new(RefCell::new(Node {
                value,
                next: None,
                prev: Some(Rc::clone(&orig_tail)),
            }));
            orig_tail.borrow_mut().next = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }

        self.size += 1;
    }

    /// Remove the node at a given index, returning the node's owned value.
    pub fn pop(&mut self, index: usize) -> Result<T> {
        if index == 0 {
            self.pop_front()
        } else if index == self.size - 1 {
            self.pop_back()
        } else {
            match &self.head {
                Some(head) => match head.borrow_mut().pop(index) {
                    Ok(x) => {
                        self.size -= 1;
                        Ok(x)
                    }
                    Err(e) => Err(e),
                },
                None => Err(LinkedListError::IndexOutOfBounds),
            }
        }
    }

    /// Remove the first node, returning the node's owned value.
    pub fn pop_front(&mut self) -> Result<T> {
        if self.size == 1 {
            self.tail = None;
            let pop_node_ref = self.head.take().unwrap();
            let pop_node = Rc::try_unwrap(pop_node_ref)
                .map_err(|_| "attempted to unwrap Rc with multiple references".to_owned())
                .unwrap();
            let value = pop_node.into_inner().value;
            self.size -= 1;

            Ok(value)
        } else if self.size > 1 {
            let pop_node_ref = self.head.take().unwrap();
            let new_head = pop_node_ref.borrow_mut().next.take().unwrap();
            new_head.borrow_mut().prev = None;
            self.head = Some(new_head);
            let pop_node = Rc::try_unwrap(pop_node_ref)
                .map_err(|_| "attempted to unwrap Rc with multiple references".to_owned())
                .unwrap();
            let value = pop_node.into_inner().value;
            self.size -= 1;

            Ok(value)
        } else {
            Err(LinkedListError::IndexOutOfBounds)
        }
    }

    /// Remove the last node, returning the node's owned value.
    pub fn pop_back(&mut self) -> Result<T> {
        if self.size == 1 {
            self.pop_front()
        } else if self.size > 1 {
            let pop_node_ref = self.tail.take().unwrap();
            let new_tail = pop_node_ref.borrow_mut().prev.take().unwrap();
            new_tail.borrow_mut().next = None;
            self.tail = Some(new_tail);
            let pop_node = Rc::try_unwrap(pop_node_ref)
                .map_err(|_| "attempted to unwrap Rc with multiple references".to_owned())
                .unwrap();
            let value = pop_node.into_inner().value;
            self.size -= 1;

            Ok(value)
        } else {
            Err(LinkedListError::IndexOutOfBounds)
        }
    }

    /// Clear the linked list.
    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    /// Reverse the elements in the linked list in place.
    pub fn reverse(&mut self) {
        let mut orig = Self {
            head: self.head.take(),
            tail: self.tail.take(),
            size: self.size,
        };

        self.size = 0;

        while orig.len() > 0 {
            let value = orig.pop_front().unwrap();
            self.push_front(value);
        }
    }
}

impl<T: Copy> LinkedList<T> {
    /// Gets a copy of the value at a given index.
    pub fn get(&self, index: usize) -> Result<T> {
        match &self.head {
            Some(head) => head.borrow().get(index),
            None => Err(LinkedListError::IndexOutOfBounds),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> From<&[T]> for LinkedList<T> {
    fn from(arr: &[T]) -> Self {
        arr.to_vec().into()
    }
}

impl<T: Clone> From<&mut [T]> for LinkedList<T> {
    fn from(arr: &mut [T]) -> Self {
        arr.to_vec().into()
    }
}

impl<T: Clone, const N: usize> From<&[T; N]> for LinkedList<T> {
    fn from(arr: &[T; N]) -> Self {
        arr.to_vec().into()
    }
}

impl<T: Clone, const N: usize> From<&mut [T; N]> for LinkedList<T> {
    fn from(arr: &mut [T; N]) -> Self {
        arr.to_vec().into()
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(arr: [T; N]) -> Self {
        let vector: Vec<_> = arr.into();
        vector.into()
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(mut vector: Vec<T>) -> Self {
        let mut ll = Self::new();

        while let Some(value) = vector.pop() {
            ll.push(0, value).unwrap();
        }

        ll
    }
}

impl<T: std::fmt::Debug, const N: usize> TryInto<[T; N]> for LinkedList<T> {
    type Error = LinkedListError;

    fn try_into(self) -> core::result::Result<[T; N], Self::Error> {
        if self.size == N {
            let vector: Vec<_> = self.into();
            Ok(vector.try_into().unwrap())
        } else {
            Err(LinkedListError::InvalidArraySize)
        }
    }
}

impl<T> Into<Vec<T>> for LinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vector = Vec::new();

        while self.size > 0 {
            vector.push(self.pop_front().unwrap());
        }

        vector
    }
}

/// An iterator over a linked list.
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front().ok()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back().ok()
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            false
        } else if self.size == 0 {
            true
        } else {
            *self.head.as_ref().unwrap().borrow() == *other.head.as_ref().unwrap().borrow()
        }
    }
}
