pub trait Image {
    fn display(&self);
}

struct RealImage {
    file_name: String,
}

impl RealImage {
    fn new(file_name: String) -> Self {
        let real_image = RealImage { file_name };
        real_image.load_from_disk();
        real_image
    }
    fn load_from_disk(&self) {
        println!("Loading {}", self.file_name);
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("Displaying {}", self.file_name);
    }
}

pub struct ProxyImage {
    file_name: String,
    real_image: RealImage,
}

impl ProxyImage {
    pub fn new(file_name: String) -> Self {
        ProxyImage { file_name: file_name.clone(), real_image: RealImage::new(file_name) }
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        self.real_image.display();
    }
}
