//////////////////////////////////////////////////////////////////
/// Persistent list implementation using Arc
/// 
/// list1 = A -> B -> C -> D
/// list2 = tail(list1) = B -> C -> D
/// list3 = push(list2, X) = X -> B -> C -> D
/// 
/// Memory looks like this:
/// 
/// list1 -> A ---+
///               |
///               v
/// list2 ------> B -> C -> D
///               ^
///               |
/// list3 -> X ---+
/// 
//////////////////////////////////////////////////////////////////

use std::rc::Rc;

struct Node<T> {
    item: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.item })
    }

    pub fn prepend(&self, item: T) -> List<T> {
        let node = Node{ item: item, next: self.head.clone() };
        List {
            head: Some(Rc::new(node))
        }
    }

    pub fn tail(&self) -> List<T>
    {
        List{ head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.item
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(node) = current_link {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                current_link = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test_rc_list {
    use super::*;

    #[test]
    fn head_on_empty_list_returns_none() {
        let lst = List::<i32>::new();
        assert_eq!(lst.head(), None);
    }

    #[test]
    fn head_returns_reference_to_recently_added_item() {
        let mut lst = List::new();
        
        let lst = lst.prepend(1).prepend(2).prepend(3);

        assert_eq!(lst.head(), Some(&3));
    }

    #[test]
    fn tail_returns_list_without_head() {
        let mut lst = List::new();
        let lst = lst.prepend(1).prepend(2).prepend(3);

        let lst = lst.tail();
        assert_eq!(lst.head(), Some(&2));

        let lst = lst.tail();
        assert_eq!(lst.head(), Some(&1));

        let lst = lst.tail();
        assert_eq!(lst.head(), None);
    }

    #[test]
    fn iter_returns_iterator_with_shared_references() {
        let mut lst = List::new();
        let lst = lst.prepend(1).prepend(2).prepend(3);

        let mut iter = lst.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}