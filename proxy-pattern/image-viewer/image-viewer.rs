// Trait representing the common interface
trait Image {
    fn display(&mut self);
}

// The real Subject
struct RealImage {
    filename: String,
}

impl RealImage {
    fn new(filename: &str) -> Self {
        let img = RealImage {
            filename: filename.to_string(),
        };
        img.load_image_from_disk();

        img
    }

    fn load_image_from_disk(&self) {
        println!("Loading image from disk: {}", self.filename);
    }
}

impl Image for RealImage {
    fn display(&mut self) {
        println!("Displaying image: {}", self.filename);
    }
}

// Proxy - controls access to the real Subject (RealImage)
struct ProxyImage {
    filename: String,
    real_image: Option<RealImage>, // Lazy initialization
}

impl ProxyImage {
    fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            real_image: None,
        }
    }
}

impl Image for ProxyImage {
    fn display(&mut self) {
        // Use mutable borrow to check and potentially initialize real_image
        if self.real_image.is_none() {
            // Lazily initialize RealImage
            let new_image = RealImage::new(&self.filename);
            self.real_image = Some(new_image);
        }

        // Now safely unwrap and call `display`
        if let Some(real_image) = self.real_image.as_mut() {
            real_image.display();
        }
    }
}

fn main() {
    let mut image = ProxyImage::new("shreyash.jpg");
    println!("[DEBUG]: Image created. No loading yet. \n");

    image.display(); // Loads and displays the image

    println!("\n[DEBUG]: Displaying again:");
    image.display(); // Doesn't load again, just displays the image
}
