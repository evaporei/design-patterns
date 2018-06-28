pub trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<&Self::Item>;
}

pub struct NameIterator {
    index: usize,
    names: Vec<String>,
}

impl NameIterator {
    pub fn new() -> Self {
        NameIterator {
            index: 0,
            names: vec![
                "Robert".to_string(),
                "John".to_string(),
                "Julie".to_string(),
                "Lora".to_string(),
            ],
        }
    }
}

impl MyIterator for NameIterator {
    type Item = String;
    fn next(&mut self) -> Option<&String> {
        let item = self.names.get(self.index);
        self.index += 1;
        item
    }
}

pub struct NameRepository {
    pub iterator: Box<MyIterator<Item=String>>,
}

impl NameRepository {
    pub fn new() -> Self {
        NameRepository { iterator: Box::new(NameIterator::new()) }
    }
}
