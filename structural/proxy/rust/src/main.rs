extern crate proxy;

use proxy::ProxyImage;
use proxy::Image;

fn main() {
    let image = ProxyImage::new("tha_meme.png".to_string());

    image.display();

    image.display();
}
