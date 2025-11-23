mod observer;
mod signals_slots;

use observer::observer_demo;
use signals_slots::signals_slots_demo;

fn main() {
    observer_demo();

    println!("\n-----------\n");

    signals_slots_demo();
}
