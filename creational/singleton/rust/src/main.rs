extern crate singleton;

use singleton::SingletonObject;

fn main() {
    let object_option = SingletonObject::get_instance();

    if let &Some(ref object) = object_option {
        object.show_message();
    }
}
