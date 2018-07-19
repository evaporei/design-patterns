pub trait Filter {
    fn execute(&self, request: &str);
}

pub struct AuthenticationFilter;

impl Filter for AuthenticationFilter {
    fn execute(&self, request: &str) {
        println!("Authenticating request: {}", request);
    }
}

pub struct DebugFilter;

impl Filter for DebugFilter {
    fn execute(&self, request: &str) {
        println!("Request log: {}", request);
    }
}

struct Target;

impl Target {
    fn execute(&self, request: &str) {
        println!("Executing request: {}", request);
    }
}

struct FilterChain {
    filters: Vec<Box<Filter>>,
    target: Target,
}

impl FilterChain {
    fn new(target: Target) -> Self {
        FilterChain {
            filters: vec![],
            target,
        }
    }
    fn add_filter(&mut self, filter: Box<Filter>) {
        self.filters.push(filter);
    }
    fn execute(&self, request: &str) {
        for filter in &self.filters {
            filter.execute(request);
        }

        self.target.execute(request);
    }
}

pub struct FilterManager {
    filter_chain: FilterChain,
}

impl FilterManager {
    pub fn new() -> Self {
        FilterManager { filter_chain: FilterChain::new(Target) }
    }
    pub fn set_filter(&mut self, filter: Box<Filter>) {
        self.filter_chain.add_filter(filter);
    }
    fn filter_request(&self, request: &str) {
        self.filter_chain.execute(request);
    }
}

pub struct Client {
    filter_manager: FilterManager,
}

impl Client {
    pub fn new(filter_manager: FilterManager) -> Self {
        Client { filter_manager }
    }
    pub fn send_request(&self, request: &str) {
        self.filter_manager.filter_request(request);
    }
}
