pub struct ChatRoom;

impl ChatRoom {
    fn show_message(user: &User, message: &str) {
        println!("*timestamp* [{}] : {}", user.name, message);
    }
}

pub struct User<'a> {
    pub name: &'a str,
}

impl<'a> User<'a> {
    pub fn new(name: &'a str) -> Self {
        User { name }
    }
    pub fn send_message(&self, message: &str) {
        ChatRoom::show_message(self, message);
    }
}
