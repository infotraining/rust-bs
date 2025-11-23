use std::fmt::{Debug, Display};

#[derive(Debug)]
struct ListNode<T> {
    item: T,
    next: Option<Box<ListNode<T>>>,
}

#[derive(Debug)]
pub struct FwdList<T> {
    head: Option<Box<ListNode<T>>>,
    size: usize,
}

impl<T> FwdList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, item: T) {
        let new_node = Box::new(ListNode {
            item,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.item)
            }
            None => None,
        }
    }
}

fn print_node<T: Display>(fmt: &mut std::fmt::Formatter, node: &ListNode<T>) -> std::fmt::Result {
    write!(fmt, "[{}, ", node.item)?;
    if let Some(next) = &node.next {
        write!(fmt, "-> ")?;
        print_node(fmt, &next)?;
        write!(fmt, "]")?;
    }
    else {
        write!(fmt, "None")?;
    }
    Ok(())
}

impl<T: Display> Display for FwdList<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "FwdList: ")?;

        if let Some(head) = &self.head {
            print_node(fmt, &**head)?;
        }
        else {
            write!(fmt, "[]")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests_fwd_list_display {
    use rstest::rstest;
    use crate::fwd_list::FwdList;

    #[rstest]
    fn empty_list() {
        let lst: FwdList<i32> = FwdList::new();

        let fmt_output = format!("{}", lst);
        assert_eq!(fmt_output, "FwdList: []");
    }

    #[rstest]
    fn list_with_items() {
        let mut lst: FwdList<i32> = FwdList::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let fmt_output = format!("{}", lst);
        assert_eq!(fmt_output, "FwdList: [3, -> [2, -> [1, None]]");
    }
}

pub struct IntoIter<T>(Option<Box<ListNode<T>>>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(node) => {
                self.0 = node.next;
                let item = node.item;
                Some(item)
            }
            None => None,
        }
    }
}

impl<T> IntoIterator for FwdList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        IntoIter(self.head.take())
    }
}

pub struct Iter<'a, T> {
    node: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.map(|node| {
            self.node = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

pub struct IterMut<'a, T> {
    node: Option<&'a mut ListNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_mut().map(|node| &mut **node);
            &mut node.item
        })
    }
}

impl<T> FwdList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            node: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            node: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

#[cfg(test)]
mod tests_fwd_list {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn lst_with_items() -> FwdList<i32> {
        let mut lst = FwdList::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);
        lst
    }

    #[test]
    fn new_list_is_empty() {
        let lst = FwdList::<i32>::new();

        assert!(lst.empty());
    }

    #[test]
    fn new_list_size_is_zero() {
        let lst = FwdList::<i32>::new();

        assert_eq!(lst.size(), 0);
    }

    #[test]
    fn new_list_when_item_pushed_then_list_is_no_longer_empty() {
        let mut lst = FwdList::<i32>::new();
        assert!(lst.empty());

        lst.push(1);
        assert!(!lst.empty());
    }

    #[test]
    fn new_list_when_item_pushed_then_size_is_increased() {
        let mut lst = FwdList::<i32>::new();
        assert_eq!(lst.size(), 0);
        lst.push(1);
        assert_eq!(lst.size(), 1);
    }

    #[rstest]
    #[allow(non_snake_case)]
    fn pop_returns_items_in_LIFO_order(lst_with_items: FwdList<i32>) {
        let mut lst = lst_with_items;

        assert_eq!(lst.pop(), Some(3));
        assert_eq!(lst.pop(), Some(2));
        assert_eq!(lst.pop(), Some(1));
        assert_eq!(lst.pop(), None);
    }

    #[rstest]
    fn when_item_is_popped_size_is_decreased(lst_with_items: FwdList<i32>) {
        let mut lst = lst_with_items;
        assert_eq!(lst.size(), 3);

        lst.pop();
        assert_eq!(lst.size(), 2);
        lst.pop();
        assert_eq!(lst.size(), 1);
        lst.pop();
        assert_eq!(lst.size(), 0);
        lst.pop();
        assert_eq!(lst.size(), 0);
    }

    #[rstest]
    fn when_all_items_are_popped_then_list_is_empty(lst_with_items: FwdList<i32>) {
        let mut lst = lst_with_items;
        lst.pop();
        lst.pop();
        lst.pop();
        assert!(lst.empty());
    }

    #[rstest]
    fn list_into_iter(lst_with_items: FwdList<i32>) {
        let mut lst = lst_with_items;

        let mut it = lst.into_iter();
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
    }

    #[rstest]
    fn list_for_into_iter(lst_with_items: FwdList<i32>) {
        let mut items = vec![];
        for item in lst_with_items.into_iter() {
            items.push(item);
        }

        assert_eq!(items, vec![3, 2, 1]);
    }

    #[rstest]
    fn list_iter(lst_with_items: FwdList<i32>) {
        let mut lst = lst_with_items;

        let mut it = lst.iter();
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
    }

    #[rstest]
    fn list_iter_mut(mut lst_with_items: FwdList<i32>) {
        let mut it = lst_with_items.iter_mut();
        assert_eq!(it.next(), Some(&mut 3));
        assert_eq!(it.next(), Some(&mut 2));
        assert_eq!(it.next(), Some(&mut 1));
        assert_eq!(it.next(), None);
    }
}
