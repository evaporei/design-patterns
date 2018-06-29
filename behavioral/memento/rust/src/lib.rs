pub struct Memento {
    state: String,
}

impl Memento {
    fn get_state(&self) -> &String {
        &self.state
    }
}

pub struct Originator {
    state: String,
}

impl Originator {
    pub fn new(state: &str) -> Self {
        Originator { state: String::from(state) }
    }
    pub fn set_state(&mut self, state: &str) {
        self.state = String::from(state);
    }
    pub fn get_state(&self) -> &String {
        &self.state
    }
    pub fn save_state_to_memento(&self) -> Memento {
        Memento { state: self.get_state().clone() }
    }
    pub fn get_state_from_memento(&mut self, memento: &Memento) {
        self.state = memento.get_state().clone();
    }
}

pub struct CareTaker {
    memento_list: Vec<Memento>,
}

impl CareTaker {
    pub fn new() -> Self {
        CareTaker { memento_list: vec![] }
    }
    pub fn add(&mut self, memento: Memento) {
        self.memento_list.push(memento);
    }
    pub fn get(&self, index: usize) -> &Memento {
        &self.memento_list[index]
    }
}
