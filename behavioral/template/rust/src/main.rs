extern crate template;

use template::*;

fn main() {
    let mut game: Box<Game> = Box::new(Cricket);
    game.play();

    println!("");

    game = Box::new(Football);
    game.play();
}
