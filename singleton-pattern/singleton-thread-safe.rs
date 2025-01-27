use std::sync::{Arc, LazyLock, Mutex};

struct Singleton {
    // Add some fields if needed
}

impl Singleton {
    // Private constructor
    fn new() -> Self {
        Self {}
    }
}

// Static LazyLock instance to ensure thread-safe, lazy initialization
static INSTANCE: LazyLock<Arc<Mutex<Singleton>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Singleton::new())));

// Helper -_> retrieve the singleton
fn get_instance() -> Arc<Mutex<Singleton>> {
    INSTANCE.clone()
}

fn main() {
    let s1 = get_instance();
    let s2 = get_instance();

    // Check if both instances are same
    let same = Arc::ptr_eq(&s1, &s2);

    println!("Both instances are the same: {}", same);
}
