type BoxedCallback<TResult = ()> = Box<dyn FnMut() -> TResult + 'static>;
type BoxedCallbackWithArg<T, TResult = ()> = Box<dyn FnMut(T) -> TResult + 'static>;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Signal<Arg> {
    slots: Vec<BoxedCallbackWithArg<Arg>>,
    weak_self: Weak<RefCell<Self>>,
}

pub struct Connection<Arg> {
    signal: Weak<RefCell<Signal<Arg>>>,
    index: usize,
}

impl<Arg> Connection<Arg> {
    pub fn disconnect(&self) {
        if let Some(signal) = self.signal.upgrade() {
            signal.borrow_mut().slots.remove(self.index);
        }
    }
}

impl<Arg: Copy + Clone> Signal<Arg> {
    pub fn new() -> Rc<RefCell<Self>> {
        let signal = Rc::new(RefCell::new(Self {
            slots: Vec::new(),
            weak_self: Weak::new(),
        }));
        signal.borrow_mut().weak_self = Rc::downgrade(&signal);
        signal
    }

    pub fn connect(&mut self, slot: BoxedCallbackWithArg<Arg>) -> Connection<Arg> {
        self.slots.push(slot);

        return Connection {
            signal: self.weak_self.clone(),
            index: self.slots.len() - 1,
        };
    }

    pub fn emit(&mut self, arg: Arg) {
        self.slots.iter_mut().for_each(|slot| slot(arg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        let message = Rc::new(RefCell::new("".to_string()));

        let mut signal = Signal::<i32>::new();

        let message_clone = message.clone();
        let slot = Box::new(move |arg| {
            *message_clone.borrow_mut() += &format!("Received: {};", arg);
        });
        let connection = signal.borrow_mut().connect(slot);
        signal.borrow_mut().emit(42);
        signal.borrow_mut().emit(43);

        connection.disconnect();

        signal.borrow_mut().emit(44);

        assert_eq!(*message.borrow(), "Received: 42;Received: 43;");
    }
}

////////////////////////////////////////////////////////////////////////////////////////
/// Signals and slots - demo
////////////////////////////////////////////////////////.//////////////////////////////

pub struct TempSensor {
    temperature: f64,
    on_temperature_changed: Rc<RefCell<Signal<f64>>>,
}

impl TempSensor {
    fn new() -> Self {
        Self {
            temperature: 0.0,
            on_temperature_changed: Signal::new(),
        }
    }

    fn set_temperature(&mut self, temperature: f64) {
        if self.temperature == temperature {
            return;
        }

        self.temperature = temperature;
        self.on_temperature_changed
            .borrow_mut()
            .emit(self.temperature);
    }
}

pub fn signals_slots_demo() {
    let mut sensor = TempSensor::new();

    let connection_1 = sensor
        .on_temperature_changed
        .borrow_mut()
        .connect(Box::new(|temp| {
            println!("Temperature changed: {}", temp);
        }));

    let connection_2 = sensor
        .on_temperature_changed
        .borrow_mut()
        .connect(Box::new(|temp| {
            println!("Logging temperature: {}", temp);
        }));

    sensor.set_temperature(22.0);
    sensor.set_temperature(23.0);

    connection_1.disconnect();
    println!("\nDisconnected connection_1\n");

    sensor.set_temperature(24.0);
    sensor.set_temperature(25.0);
}