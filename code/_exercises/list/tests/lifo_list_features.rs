use list::lifo_list::LifoList;
use rspec::describe;
use rstest::{fixture, rstest};
use std::cmp::Ordering;
use std::rc::Rc;

#[fixture]
fn lst() -> LifoList<i32> {
    let mut lst = LifoList::new();

    lst.push(1);
    lst.push(2);
    lst.push(3);
    lst.push(4);
    lst.push(5);
    lst.push(6);

    lst
}

#[rstest]
fn collecting_into_lifo_list() {
    let lst: LifoList<_> = (1..=5).collect();

    let mut it = lst.iter();
    assert_eq!(it.next(), Some(&5));
    assert_eq!(it.next(), Some(&4));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), None);
}

#[rstest]
fn mapping_with_function(lst: LifoList<i32>) {
    let mapped_lst = lst.iter().map(|x| x * 10);

    let mut it = mapped_lst;
    assert_eq!(it.next(), Some(60));
    assert_eq!(it.next(), Some(50));
    assert_eq!(it.next(), Some(40));
    assert_eq!(it.next(), Some(30));
    assert_eq!(it.next(), Some(20));
    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), None);
}

#[rstest]
fn filtering_with_function(lst: LifoList<i32>) {
    let filtered_lst = lst.iter().filter(|n| *n % 2 == 0);

    let mut it = filtered_lst;
    assert_eq!(it.next(), Some(&6));
    assert_eq!(it.next(), Some(&4));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), None);
}

#[rstest]
fn folding_with_function(lst: LifoList<i32>) {
    let zero = "0".to_string();
    let folded = lst.iter().fold(zero, |acc, n| format!("({acc} + {n})"));
    let expected = "((((((0 + 6) + 5) + 4) + 3) + 2) + 1)".to_string();
    assert_eq!(folded, expected);
}

//////////////////////////////////////////////////////////////////////////////////
/// RSpec-style tests
//////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, Debug)]
struct Environment {
    // ...
}

#[test]
fn iterator_features() {
    rspec::run(&describe(
        "lifo_list - iterator features",
        Environment::default(),
        |ctx| {
            let list = Rc::new(lst());

            let lst = list.clone();
            ctx.it("collects items into a container", move |_| {
                let v: Vec<i32> = lst.iter().cloned().collect();
                assert_eq!(v, vec![6, 5, 4, 3, 2, 1]);
            });

            let lst = list.clone();
            ctx.it("maps items with a function", move |_| {
                let mapped_lst = lst.iter().map(|x| x * x).collect::<Vec<i32>>();
                assert_eq!(mapped_lst, vec![36, 25, 16, 9, 4, 1]);
            });

            let lst = list.clone();
            ctx.it("filters items with a function", move |_| {
                let filtered_lst = lst.iter().filter(|n| *n % 2 == 0);
                assert_eq!(filtered_lst.cmp(vec![6, 4, 2].iter()), Ordering::Equal);
            });

            let lst = list.clone();
            ctx.it("folds items with a function", move |_| {
                let zero = "0".to_string();
                let folded = lst.iter().fold(zero, |acc, n| format!("({acc} + {n})"));
                let expected = "((((((0 + 6) + 5) + 4) + 3) + 2) + 1)".to_string();
                assert_eq!(folded, expected);
            });
        },
    ));
}