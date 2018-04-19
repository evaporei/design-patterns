use std::sync::{Once, ONCE_INIT};

static mut OBJECT: Option<SingletonObject> = None;
static INIT: Once = ONCE_INIT;

pub struct SingletonObject {}

impl SingletonObject {
    pub fn get_instance<'a>() -> &'a Option<SingletonObject> {
        unsafe {
            INIT.call_once(|| {
                OBJECT = Some(SingletonObject {});
            });
            &OBJECT
        }
    }
    pub fn show_message(&self) {
        println!("Message!!!");
    }
}
