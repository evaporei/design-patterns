pub trait Game {
    fn initialize(&self);
    fn start_play(&self);
    fn end_play(&self);
    fn play(&self) {
        self.initialize();
        self.start_play();
        self.end_play();
    }
}

pub struct Cricket;

impl Game for Cricket {
    fn initialize(&self) {
        println!("Cricket Game Initialized! Start playing.");
    }
    fn start_play(&self) {
        println!("Cricket Game Started! Enjoy the game!");
    }
    fn end_play(&self) {
        println!("Cricket Game Finished!");
    }
}

pub struct Football;

impl Game for Football {
    fn initialize(&self) {
        println!("Football Game Initialized! Start playing.");
    }
    fn start_play(&self) {
        println!("Football Game Started! Enjoy the game!");
    }
    fn end_play(&self) {
        println!("Football Game Finished!");
    }
}
