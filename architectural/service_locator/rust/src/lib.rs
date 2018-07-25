pub trait Service {
    fn get_name(&self) -> &str;
    fn execute(&self) {
        println!("Executing {}", self.get_name());
    }
}

struct Service1;

impl Service for Service1 {
    fn get_name(&self) -> &str {
        "Service1"
    }
}

struct Service2;

impl Service for Service2 {
    fn get_name(&self) -> &str {
        "Service2"
    }
}

struct InitialContext;

impl InitialContext {
    fn factory(&self, jndi_name: &str) -> Option<Box<Service>> {
        if jndi_name.to_uppercase() == "SERVICE1" {
            println!("Creating a new Service1 object");
            return Some(Box::new(Service1))
        }

        if jndi_name.to_uppercase() == "SERVICE2" {
            println!("Creating a new Service2 object");
            return Some(Box::new(Service2))
        }

        return None
    }
}

struct Cache {
    services: Vec<Box<Service>>,
}

impl Cache {
    fn new() -> Self {
        Cache { services: vec![] }
    }
    fn get_service(&self, service_name: &str) -> Option<&Box<Service>> {
        self.services.iter()
            .find(|service| {
                if service.get_name().to_uppercase() == service_name.to_uppercase() {
                    return true
                }

                return false
            })
    }
    fn add_service(&mut self, service_name: &str) {
        if let None = self.get_service(service_name) {
            if let Some(new_service) = InitialContext.factory(service_name) {
                self.services.push(new_service);
            }
        } else {
            println!("Has cached {} object", service_name);
        }
    }
}

pub struct ServiceLocator {
    cache: Cache,
}

impl ServiceLocator {
    pub fn new() -> Self {
        ServiceLocator { cache: Cache::new() }
    }
    pub fn get_service(&mut self, jndi_name: &str) -> Option<&Box<Service>> {
        self.cache.add_service(jndi_name);
        self.cache.get_service(jndi_name)
    }
}
