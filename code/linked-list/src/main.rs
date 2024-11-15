#[macro_use]
mod list_v1;
mod list_v2;

use list_v1::LinkedList;

fn main() {
    let my_list = lst![1, 2, 3, 4, 5];

    print!("List: ");
    for i in my_list.iter() {
        print!("{} ", i);
    }
    println!();

    println!("Sum: {}", my_list.iter().sum::<i32>());
}
