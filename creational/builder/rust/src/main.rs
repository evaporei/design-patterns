extern crate builder;

use builder::MealBuilder;

fn main() {
    let meal_builder = MealBuilder {};

    let veg_meal = meal_builder.prepare_veg_meal();
    println!("Veg Meal!");
    veg_meal.show_items();
    println!("Total Cost: {}", veg_meal.get_cost());

    println!("");

    let non_veg_meal = meal_builder.prepare_non_veg_meal();
    println!("Non Veg Meal!");
    non_veg_meal.show_items();
    println!("Total Cost: {}", non_veg_meal.get_cost());
}
