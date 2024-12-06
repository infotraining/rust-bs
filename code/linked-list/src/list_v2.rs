struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

macro_rules! lst {
    ($($e:expr),*) => {{
        let mut lst = List::new();
        $(lst.push($e);)*
        lst
    }};
}

mod tests_list {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn pop_on_empty_list_returns_None() {
        let mut lst = List::<i32>::new();
        assert_eq!(lst.pop(), None);
    }

    #[rstest]
    fn push_pop_in_lifo_order() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);
        assert_eq!(lst.pop(), Some(3));
        assert_eq!(lst.pop(), Some(2));
        assert_eq!(lst.pop(), Some(1));
        assert_eq!(lst.pop(), None);
    }

    #[rstest]
    fn peek_returns_reference_to_head() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);
        assert_eq!(lst.peek(), Some(&3));
        assert_eq!(lst.peek(), Some(&3));
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

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[rstest]
    fn iter_mut_creates_iterator_with_mutable_references() {
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut iter = lst.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

    #[rstest]
    fn into_iter_creates_iterator_that_takes_ownership() {
        let mut lst: List<String> = List::new();
        lst.push(1.to_string());
        lst.push(2.to_string());
        lst.push(3.to_string());

        let mut iter = lst.into_iter();

        assert_eq!(iter.next(), Some("3".to_string()));
        assert_eq!(iter.next(), Some("2".to_string()));
        assert_eq!(iter.next(), Some("1".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[rstest]
    fn macro_lst() {
        let l = lst![1, 2, 3];

        let mut iter = l.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
