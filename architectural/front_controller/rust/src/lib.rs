struct HomeView;

impl HomeView {
    fn show(&self) {
        println!("Displaying Home Page");
    }
}

struct StudentView;

impl StudentView {
    fn show(&self) {
        println!("Displaying Student Page");
    }
}

struct Dispatcher {
    home_view: HomeView,
    student_view: StudentView,
}

impl Dispatcher {
    fn new() -> Self {
        Dispatcher { home_view: HomeView, student_view: StudentView }
    }
    fn dispatch(&self, request: &str) {
        if request == "STUDENT" {
            self.student_view.show();
        } else {
            self.home_view.show();
        }
    }
}

pub struct FrontController {
    dispatcher: Dispatcher,
}

impl FrontController {
    pub fn new() -> Self {
        FrontController { dispatcher: Dispatcher::new() }
    }
    fn is_authentic_user(&self) -> bool {
        println!("User is authenticated successfully.");
        true
    }
    fn track_request(&self, request: &str) {
        println!("Page requested: {}", request);
    }
    pub fn dispatch_request(&self, request: &str) {
        self.track_request(request);

        if self.is_authentic_user() {
            self.dispatcher.dispatch(request);
        }
    }
}
