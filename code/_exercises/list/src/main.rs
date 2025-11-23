use list::lifo_list::LifoList;

fn main() {
    println!("LIFO List ver. 1.0");

    let mut lst = LifoList::new();
    println!("lst: {lst}");

    lst.push(1);
    lst.push(2);
    lst.push(3);
    lst.push(665);

    println!("lst: {lst}");

    lst.pop();

    println!("lst: {lst}");

    println!("Iterating over list:");
    for item in lst.iter() {
        print!("{} ", item);
    }
    println!("");

    println!("Changing items on the list:");
    for item in lst.iter_mut() {
        *item *= *item;
    }
    println!("{lst}");
}
