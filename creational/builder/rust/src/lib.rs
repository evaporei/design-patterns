trait Item {
    fn name(&self) -> String;
    fn packing(&self) -> Box<Packing>;
    fn price(&self) -> f64;
}

trait Packing {
    fn pack(&self) -> String;
}

struct Wrapper {}

impl Packing for Wrapper {
    fn pack(&self) -> String {
        String::from("Wrapper")
    }
}

struct Bottle {}

impl Packing for Bottle {
    fn pack(&self) -> String {
        String::from("Bottle")
    }
}

struct VegBurger {}

impl Item for VegBurger {
    fn price(&self) -> f64 {
        25 as f64
    }
    fn packing(&self) -> Box<Packing> {
        Box::new(Wrapper {})
    }
    fn name(&self) -> String {
        String::from("Veg Burger")
    }
}

struct ChickenBurger {}

impl Item for ChickenBurger {
    fn price(&self) -> f64 {
        50.5 as f64
    }
    fn packing(&self) -> Box<Packing> {
        Box::new(Wrapper {})
    }
    fn name(&self) -> String {
        String::from("Chicken Burger")
    }
}

struct Coke {}

impl Item for Coke {
    fn price(&self) -> f64 {
        30 as f64
    }
    fn packing(&self) -> Box<Packing> {
        Box::new(Bottle {})
    }
    fn name(&self) -> String {
        String::from("Coke")
    }
}

struct Pepsi {}

impl Item for Pepsi {
    fn price(&self) -> f64 {
        35 as f64
    }
    fn packing(&self) -> Box<Packing> {
        Box::new(Bottle {})
    }
    fn name(&self) -> String {
        String::from("Pepsi")
    }
}

pub struct Meal {
    items: Vec<Box<Item>>,
}

impl Meal {
    fn new() -> Meal {
        Meal {
            items: vec![],
        }
    }
    fn add_item(&mut self, item: Box<Item>) {
        self.items.push(item);
    }
    pub fn get_cost(&self) -> f64 {
        let mut total_cost: f64 = 0.0;

        for item in &self.items {
            total_cost += item.price();
        }

        total_cost
    }
    pub fn show_items(&self) {
        for item in &self.items {
            println!("Item: {}", item.name());
            println!("Packing: {}", item.packing().pack());
            println!("Price: {}", item.price());
        }
    }
}

pub struct MealBuilder {}

impl MealBuilder {
    pub fn prepare_veg_meal(&self) -> Meal {
        let mut meal = Meal::new();
        meal.add_item(Box::new(VegBurger {}));
        meal.add_item(Box::new(Coke {}));
        meal
    }
    pub fn prepare_non_veg_meal(&self) -> Meal {
        let mut meal = Meal::new();
        meal.add_item(Box::new(ChickenBurger {}));
        meal.add_item(Box::new(Pepsi {}));
        meal
    }
}
