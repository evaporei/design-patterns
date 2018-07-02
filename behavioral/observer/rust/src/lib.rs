pub trait Observer {
    fn update(&self, state: usize);
}

pub struct Subject {
    observers: Vec<Box<Observer>>,
    state: usize,
}

impl Subject {
    pub fn new() -> Self {
        Subject { observers: vec![], state: 0 }
    }
    pub fn get_state(& self) -> usize {
        self.state
    }
    pub fn set_state(&mut self, new_state: usize) {
        self.state = new_state;
        self.notify_all_observers();
    }
    pub fn attach(&mut self, observer: Box<Observer>) {
        self.observers.push(observer);
    }
    pub fn notify_all_observers(&self) {
        for observer in &self.observers {
            observer.update(self.get_state());
        }
    }
}

pub struct BinaryObserver;

impl Observer for BinaryObserver {
    fn update(&self, state: usize) {
        println!("Binary String: {:b}", state);
    }
}

pub struct OctalObserver;

impl Observer for OctalObserver {
    fn update(&self, state: usize) {
        println!("Octal String: {:o}", state);
    }
}

pub struct HexaObserver;

impl Observer for HexaObserver {
    fn update(&self, state: usize) {
        println!("Hexa String: {:x}", state);
    }
}
