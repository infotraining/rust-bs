use std::sync::{Arc, Weak};

pub trait Observer {
    type Subject;

    fn update(&self, subject: &Self::Subject);
}

pub trait Observable {
    type ObserverPtr;

    fn add_observer(&mut self, observer: Self::ObserverPtr);
    fn remove_observer(&mut self, observer: Self::ObserverPtr);
    fn notify(&self);
}

pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
}

impl Observable for Subject {
    type ObserverPtr = Arc<dyn Observer<Subject = Self>>;

    fn add_observer(&mut self, observer: Self::ObserverPtr) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn remove_observer(&mut self, observer: Self::ObserverPtr) {
        self.observers
            .retain(|o| !o.ptr_eq(&Arc::downgrade(&observer)));
    }

    fn notify(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.update(&self));
    }
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: Vec::new(),
            state: state.into(),
        }
    }

    pub fn set_state(&mut self, state: &str) {
        if self.state == state {
            return;
        }

        self.state = state.into();
        self.notify();
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Observer demo
//////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ConcreteObserver {
    name: String,
}

impl ConcreteObserver {
    pub fn new(name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.into() })
    }
}

impl Observer for ConcreteObserver {
    type Subject = Subject;

    fn update(&self, subject: &Self::Subject) {
        println!(
            "Observing subject with state={:?} in {}",
            subject.state(),
            self.name
        );
    }
}

pub fn observer_demo() {
    let mut subject = Subject::new("initial state");

    let observer1 = ConcreteObserver::new("observer1");
    let observer2 = ConcreteObserver::new("observer2");

    subject.add_observer(observer1.clone());
    subject.add_observer(observer2.clone());

    subject.set_state("new state - A");
    subject.set_state("new state - B");

    subject.remove_observer(observer1);
    println!("\nRemoved observer1\n");

    subject.set_state("new state - C");
    subject.set_state("new state - D");
}
