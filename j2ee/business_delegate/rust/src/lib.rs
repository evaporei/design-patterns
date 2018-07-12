pub trait BusinessService {
    fn do_processing(&self);
}

pub struct EJBService;

impl BusinessService for EJBService {
    fn do_processing(&self) {
        println!("Processing task by invoking EJB Service");
    }
}

pub struct JMSService;

impl BusinessService for JMSService {
    fn do_processing(&self) {
        println!("Processing task by invoking JMS Service");
    }
}

pub struct BusinessLookUp;

impl BusinessLookUp {
    pub fn get_business_service(&self, service_type: &str) -> Box<BusinessService> {
        if service_type == "EJB" {
            return Box::new(EJBService)
        }
        Box::new(JMSService)
    }
}

pub struct BusinessDelegate {
    look_up_service: BusinessLookUp,
    business_service: Option<Box<BusinessService>>,
    service_type: String,
}

impl BusinessDelegate {
    pub fn new() -> Self {
        BusinessDelegate {
            look_up_service: BusinessLookUp,
            business_service: None,
            service_type: String::new(),
        }
    }
    pub fn set_service_type(&mut self, service_type: &str) {
        self.service_type = String::from(service_type);
    }
    pub fn do_task(&mut self) {
        self.business_service = Some(self.look_up_service.get_business_service(&self.service_type));
        if let Some(ref business_service) = self.business_service {
            business_service.do_processing();
        }
    }
}

pub struct Client<'a> {
    pub business_service: &'a mut BusinessDelegate,
}

impl<'a> Client<'a> {
    pub fn new(business_service: &'a mut BusinessDelegate) -> Self {
        Client { business_service }
    }
    pub fn do_task(&mut self) {
        self.business_service.do_task();
    }
}
