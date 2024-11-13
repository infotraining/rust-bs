use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn push(&mut self, value: T) {
        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(Box::new(Node { value, next: None }));

        self.len += 1;
    }

    fn last(&self) -> Option<&T> {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.value);
            }

            current = &node.next;
        }

        None
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: self.head.map(|node| *node),
        }
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current_node = &self.head;

        for _i in 0..index {
            current_node = &current_node.as_ref().expect("Index out of bounds").next;
        }

        &(*current_node.as_ref().expect("Index out of bounds")).value
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut current_node = &mut self.head;

        for _i in 0..index {
            current_node = &mut current_node.as_mut().expect("Index out of bounds").next;
        }

        &mut (*current_node.as_mut().expect("Index out of bounds")).value
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}

struct IntoIter<T> {
    next: Option<Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.map(|node| *node);
            node.value
        })
    }
}

macro_rules! lst {
    ($($e:expr),*) => {{
        let mut lst = LinkedList::new();
        $(lst.push($e);)*
        lst
    }};
}

#[cfg(test)]
mod tests_linked_list {
    use super::*;

    #[test]
    fn new_creates_empty_list() {
        let lst = LinkedList::<i32>::new();

        assert_eq!(lst.len(), 0);
    }

    #[test]
    fn push_adds_element_at_end() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);
        lst.push(43);

        assert_eq!(lst.len(), 2);
    }

    #[test]
    fn last_returns_reference_to_last_element() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);
        lst.push(43);

        assert_eq!(lst.last().unwrap(), &43);
    }

    #[test]
    fn index_operator_returns_reference_to_element() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);
        lst.push(43);

        assert_eq!(lst[0], 42);
        assert_eq!(lst[1], 43);
    }

    #[test]
    #[should_panic]
    fn index_operator_panics_when_out_of_bounds() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);

        let _ = lst[1];
    }

    #[test]
    fn index_mut_allows_to_change_element() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);
        lst.push(43);

        lst[0] = 100;
        lst[1] = 101;

        assert_eq!(lst[0], 100);
        assert_eq!(lst[1], 101);
    }

    #[test]
    fn iter_returns_iterator() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);
        lst.push(43);
        lst.push(44);

        let mut iter = lst.iter();
        assert_eq!(iter.next(), Some(&42));
        assert_eq!(iter.next(), Some(&43));
        assert_eq!(iter.next(), Some(&44));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_with_for_loop() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);
        lst.push(43);
        lst.push(44);

        let mut sum = 0;
        for i in lst.iter() {
            sum += i;
        }

        assert_eq!(sum, 129);
    }

    #[test]
    fn into_iter_with_for_loop() {
        let mut lst = LinkedList::<String>::new();
        lst.push("Hello".to_string());
        lst.push("World".to_string());
        
        let mut result = String::new();
        for i in lst.into_iter() {
            result += &i;
        }
    }

    #[test]
    fn iter_mut_with_for_loop() {
        let mut lst = LinkedList::<i32>::new();
        lst.push(42);
        lst.push(43);
        lst.push(44);

        for item in lst.iter_mut() {
            *item += 100;
        }

        assert_eq!(lst[0], 142);
        assert_eq!(lst[1], 143);
        assert_eq!(lst[2], 144);
    }

    #[test]
    fn macro_lst_creates_list_with_given_elements() {
        let lst = lst![1, 2, 3, 4, 5];

        assert_eq!(lst.len(), 5);
        assert_eq!(lst[0], 1);
        assert_eq!(lst[1], 2);
        assert_eq!(lst[2], 3);
        assert_eq!(lst[3], 4);
        assert_eq!(lst[4], 5);
    }
}

fn main() {
    let my_list = lst![1, 2, 3, 4, 5];

    for i in my_list.iter() {
        println!("{}", i);
    }
}
