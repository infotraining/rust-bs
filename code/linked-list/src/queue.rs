use std::{mem, ptr};

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = *mut Node<T>;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // UNSAFE pointer
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let mut new_tail = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                unsafe {
                    (*self.tail).next = new_tail; // dereferencing pointer is unsafe
                }
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
           self.head.as_ref().map(|node| &(*node).elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut (*node).elem)
        }
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod queue_tests {
    use rstest::rstest;
    use super::List;
    #[test]
    fn queue_basic_operations() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    fn pop_on_empty_list_returns_None() {
        let mut lst = List::<i32>::new();
        assert_eq!(lst.pop(), None);
    }

    #[rstest]
    fn push_pop_in_fifo_order() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);
        assert_eq!(lst.pop(), Some(1));
        assert_eq!(lst.pop(), Some(2));
        assert_eq!(lst.pop(), Some(3));
        assert_eq!(lst.pop(), None);
    }

    #[rstest]
    fn peek_returns_reference_to_head() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);
        assert_eq!(lst.peek(), Some(&1));
    }

    #[rstest]
    fn peek_mut_returns_mutable_reference_to_head() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);
        let mut_head = lst.peek_mut().unwrap();
        *mut_head = 4;
        assert_eq!(lst.peek(), Some(&4));
    }

    #[rstest]
    fn iter_creates_iterator() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut iter = lst.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[rstest]
    fn iter_mut_creates_iterator_with_mutable_references() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut iter = lst.iter_mut();

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[rstest]
    fn into_iter_creates_iterator_that_takes_ownership() {
        let mut lst: List<String> = List::new();
        lst.push(1.to_string());
        lst.push(2.to_string());
        lst.push(3.to_string());

        let mut iter = lst.into_iter();

        assert_eq!(iter.next(), Some("1".to_string()));
        assert_eq!(iter.next(), Some("2".to_string()));
        assert_eq!(iter.next(), Some("3".to_string()));
        assert_eq!(iter.next(), None);
    }
}
