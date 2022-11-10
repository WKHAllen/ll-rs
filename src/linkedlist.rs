#![allow(dead_code)]

use std::iter::{FromIterator, IntoIterator};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::vec::IntoIter;

/// Linked list errors.
pub enum LinkedListError {
    IndexOutOfBounds { index: usize, size: usize },
    InvalidArraySize { size: usize, array_size: usize },
}

impl std::fmt::Debug for LinkedListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::IndexOutOfBounds { index, size } => write!(
                f,
                "index out of bounds; linked list size: {}, provided index: {}",
                size, index
            ),
            Self::InvalidArraySize { size, array_size } => write!(
                f,
                "invalid array size; linked list size: {}, array size: {}",
                size, array_size
            ),
        }
    }
}

/// Linked list result type.
pub type Result<T> = core::result::Result<T, LinkedListError>;

/// An iterator over the elements of a linked list.
pub struct Iter<'a, T> {
    /// A reference to the current node.
    current_node: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> Iter<'a, T> {
    /// Create an iterator from a linked list.
    pub fn new(ll: &'a LinkedList<T>) -> Self {
        Self {
            current_node: ll.head.as_ref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node {
            Some(node) => {
                self.current_node = node.get_next();
                Some(&**node)
            }
            None => None,
        }
    }
}

/// A node in a linked list.
#[derive(Clone, Debug)]
struct LinkedListNode<T> {
    /// The node's value.
    value: T,
    /// The next node in the linked list.
    next: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T> {
    /// Take the node's value and next node, consuming the node itself.
    pub fn take(self) -> (T, Option<LinkedListNode<T>>) {
        (self.value, self.next.map(|next| *next))
    }

    /// Get a reference to the node's value.
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Get a mutable reference to the node's value.
    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Set a node's value.
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    /// Take the node's value, consuming the node itself.
    pub fn take_value(self) -> T {
        self.value
    }

    /// Check if the node points to another node after it.
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Get an optional reference to the next node.
    pub fn get_next(&self) -> Option<&LinkedListNode<T>> {
        self.next.as_ref().map(|next| &**next)
    }

    /// Get an optional mutable reference to the next node.
    pub fn get_next_mut(&mut self) -> Option<&mut LinkedListNode<T>> {
        self.next.as_mut().map(|next| &mut **next)
    }

    /// Take ownership of the next node, removing it from the list.
    pub fn take_next(&mut self) -> Option<LinkedListNode<T>> {
        self.next.take().map(|next| *next)
    }

    /// Set the node's next node.
    pub fn set_next(&mut self, next: Self) {
        self.next = Some(Box::new(next));
    }

    /// Set an empty next node with the specified value.
    pub fn set_next_by_value(&mut self, value: T) {
        self.next = Some(Box::new(Self { value, next: None }));
    }
}

impl<T> Deref for LinkedListNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for LinkedListNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

/// A linked list.
#[derive(Clone, Debug)]
pub struct LinkedList<T> {
    /// The first node in the linked list.
    head: Option<LinkedListNode<T>>,
    /// The total number of nodes in the linked list.
    size: usize,
}

impl<T> LinkedList<T> {
    /// Create an empty linked list.
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    /// Get the number of nodes in the linked list.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Get a reference to the value at a node by index.
    pub fn get_value(&self, index: usize) -> Result<&T> {
        if index < self.size {
            let mut current = self.head.as_ref().unwrap();

            for _ in 0..index {
                current = current.get_next().unwrap();
            }

            Ok(current.get_value())
        } else {
            Err(LinkedListError::IndexOutOfBounds {
                index,
                size: self.size,
            })
        }
    }

    /// Get a mutable reference to the value at a node by index.
    pub fn get_value_mut(&mut self, index: usize) -> Result<&mut T> {
        if index < self.size {
            let mut current = self.head.as_mut().unwrap();

            for _ in 0..index {
                current = current.get_next_mut().unwrap();
            }

            Ok(current.get_value_mut())
        } else {
            Err(LinkedListError::IndexOutOfBounds {
                index,
                size: self.size,
            })
        }
    }

    /// Set the value at a node by index.
    pub fn set_value(&mut self, index: usize, value: T) -> Result<()> {
        if index < self.size {
            let mut current = self.head.as_mut().unwrap();

            for _ in 0..index {
                current = current.get_next_mut().unwrap();
            }

            current.set_value(value);

            Ok(())
        } else {
            Err(LinkedListError::IndexOutOfBounds {
                index,
                size: self.size,
            })
        }
    }

    /// Insert a value in a new node at a given index in the linked list.
    pub fn push(&mut self, index: usize, value: T) -> Result<()> {
        if index == 0 {
            self.push_front(value);

            Ok(())
        } else if index <= self.size {
            let mut current = self.head.as_mut().unwrap();

            for _ in 0..index - 1 {
                current = current.get_next_mut().unwrap();
            }

            let current_next = current.take_next();
            current.set_next(LinkedListNode {
                value,
                next: current_next.map(|value| Box::new(value)),
            });
            self.size += 1;

            Ok(())
        } else {
            Err(LinkedListError::IndexOutOfBounds {
                index,
                size: self.size,
            })
        }
    }

    /// Insert a value at the start of the linked list.
    pub fn push_front(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(LinkedListNode {
            value,
            next: next.map(|value| Box::new(value)),
        });
        self.size += 1;
    }

    /// Insert a value at the end of the linked list.
    pub fn push_back(&mut self, value: T) {
        if self.size > 0 {
            let mut current = self.head.as_mut().unwrap();

            while current.has_next() {
                current = current.get_next_mut().unwrap();
            }

            current.set_next_by_value(value);
        } else {
            self.head = Some(LinkedListNode { value, next: None });
        }

        self.size += 1;
    }

    /// Remove the node at a given index, returning the node's owned value.
    pub fn pop(&mut self, index: usize) -> Result<T> {
        if self.size >= 1 && index == 0 {
            self.pop_front()
        } else if index < self.size {
            let mut current = self.head.as_mut().unwrap();

            for _ in 0..index - 1 {
                current = current.get_next_mut().unwrap();
            }

            let mut node = current.take_next().unwrap();

            if node.has_next() {
                current.set_next(node.take_next().unwrap());
            }

            self.size -= 1;

            Ok(node.take_value())
        } else {
            Err(LinkedListError::IndexOutOfBounds {
                index,
                size: self.size,
            })
        }
    }

    /// Remove the first node, returning the node's owned value.
    pub fn pop_front(&mut self) -> Result<T> {
        if self.size >= 1 {
            let mut node = self.head.take().unwrap();
            self.head = node.take_next();
            self.size -= 1;

            Ok(node.take_value())
        } else {
            Err(LinkedListError::IndexOutOfBounds { index: 0, size: 0 })
        }
    }

    /// Remove the last node, returning the node's owned value.
    pub fn pop_back(&mut self) -> Result<T> {
        if self.size == 1 {
            let node = self.head.take().unwrap();
            self.size = 0;

            Ok(node.take_value())
        } else if self.size > 1 {
            let mut current = self.head.as_mut().unwrap();

            while current.get_next().unwrap().has_next() {
                current = current.get_next_mut().unwrap();
            }

            let node = current.take_next().unwrap();
            self.size -= 1;

            Ok(node.take_value())
        } else {
            Err(LinkedListError::IndexOutOfBounds { index: 0, size: 0 })
        }
    }

    /// Clear the linked list.
    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    /// Reverse the elements in the linked list in place.
    pub fn reverse(&mut self) {
        let mut orig = Self {
            head: self.head.take(),
            size: self.size,
        };
        self.size = 0;

        while orig.len() > 0 {
            let value = orig.pop_front().unwrap();
            self.push(0, value).unwrap();
        }
    }

    /// Returns an iterator over the elements in the linked list.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(self)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_value(index).unwrap()
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_value_mut(index).unwrap()
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
            Err(LinkedListError::InvalidArraySize {
                size: self.size,
                array_size: N,
            })
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

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let vector: Vec<_> = self.into();
        vector.into_iter()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size == other.size {
            let mut self_current = self.head.as_ref();
            let mut other_current = other.head.as_ref();

            while self_current.is_some() {
                if self_current.unwrap().value != other_current.unwrap().value {
                    return false;
                }

                self_current = self_current.unwrap().get_next();
                other_current = other_current.unwrap().get_next();
            }

            true
        } else {
            false
        }
    }
}
