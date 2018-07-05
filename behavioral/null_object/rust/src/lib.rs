pub trait AbstractCustomer {
    fn is_nil(&self) -> bool;
    fn get_name(&self) -> &str;
}

pub struct RealCustomer<'a> {
    name: &'a str,
}

impl<'a> RealCustomer<'a> {
    pub fn new(name: &'a str) -> Self {
        RealCustomer { name }
    }
}

impl<'a> AbstractCustomer for RealCustomer<'a> {
    fn is_nil(&self) -> bool {
        false
    }
    fn get_name(&self) -> &str {
        self.name
    }
}

pub struct NullCustomer;

impl AbstractCustomer for NullCustomer {
    fn is_nil(&self) -> bool {
        true
    }
    fn get_name(&self) -> &str {
        "Not Available in Customer Database"
    }
}

pub struct CustomerFactory<'a> {
    customer_names: Vec<&'a str>,
}

impl<'a> CustomerFactory<'a> {
    pub fn new(customer_names: Vec<&'a str>) -> Self {
        CustomerFactory { customer_names }
    }
    pub fn get_customer(&self, name: &'static str) -> Box<AbstractCustomer> {
        for customer in &self.customer_names {
            if *customer == name {
                return Box::new(RealCustomer::new(name))
            }
        }

        Box::new(NullCustomer)
    }
}
