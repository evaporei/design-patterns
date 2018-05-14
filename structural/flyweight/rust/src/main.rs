extern crate flyweight;

use flyweight::Shape;
use flyweight::CircleCache;
use flyweight::Color;

fn main() {
    let mut circle_cache = CircleCache::new();
    {
        let red_circle = circle_cache.get_circle(1, Color::Red);
        println!("Red Circle ID: {}", red_circle.id());
        red_circle.draw();
    }

    {
        let blue_circle = circle_cache.get_circle(2, Color::Blue);
        println!("Blue Circle ID: {}", blue_circle.id());
        blue_circle.draw();
    }

    {
        let same_red_circle = circle_cache.get_circle(3, Color::Red);
        println!("Same Red Circle ID: {}", same_red_circle.id());
        same_red_circle.draw();
    }
}
