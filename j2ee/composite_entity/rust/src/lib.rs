struct DependentObject1 {
    data: String,
}

impl DependentObject1 {
    fn new() -> Self {
        DependentObject1 { data: String::new() }
    }
    fn set_data(&mut self, data: &str) {
        self.data = String::from(data);
    }
    fn get_data(&self) -> &String {
        &self.data
    }
}

struct DependentObject2 {
    data: String,
}

impl DependentObject2 {
    fn new() -> Self {
        DependentObject2 { data: String::new() }
    }
    fn set_data(&mut self, data: &str) {
        self.data = String::from(data);
    }
    fn get_data(&self) -> &String {
        &self.data
    }
}

struct CoarseGrainedObject {
    dependent_object1: DependentObject1,
    dependent_object2: DependentObject2,
}

impl CoarseGrainedObject {
    fn new() -> Self {
        CoarseGrainedObject {
            dependent_object1: DependentObject1::new(),
            dependent_object2: DependentObject2::new(),
        }
    }
    fn set_data(&mut self, data1: &str, data2: &str) {
        self.dependent_object1.set_data(data1);
        self.dependent_object2.set_data(data2);
    }
    fn get_data(&self) -> Vec<&String> {
        vec![
            self.dependent_object1.get_data(),
            self.dependent_object2.get_data(),
        ]
    }
}

struct CompositeEntity {
    coarse_grained_object: CoarseGrainedObject,
}

impl CompositeEntity {
    fn new() -> Self {
        CompositeEntity { coarse_grained_object: CoarseGrainedObject::new() }
    }
    fn set_data(&mut self, data1: &str, data2: &str) {
        self.coarse_grained_object.set_data(data1, data2);
    }
    fn get_data(&self) -> Vec<&String> {
        self.coarse_grained_object.get_data()
    }
}

pub struct Client {
    composite_entity: CompositeEntity,
}

impl Client {
    pub fn new() -> Self {
        Client { composite_entity: CompositeEntity::new() }
    }
    pub fn set_data(&mut self, data1: &str, data2: &str) {
        self.composite_entity.set_data(data1, data2);
    }
    pub fn print_data(&self) {
        self.composite_entity.get_data()
            .iter()
            .for_each(|data| println!("Data: {}", data));
    }
}
