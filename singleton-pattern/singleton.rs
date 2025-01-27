use std::sync::{Arc, Mutex, Once};

struct Singleton {}

impl Singleton {
    // Private constructor (assuming)
    fn new() -> Self {
        Self {}
    }
}

// Static instance
static mut INSTANCE: Option<Arc<Mutex<Singleton>>> = None;
static INIT: Once = Once::new();

fn get_instance() -> Arc<Mutex<Singleton>> {
    unsafe {
        INIT.call_once(|| {
            // Create the instance one time, and store it in the static variable
            INSTANCE = Some(Arc::new(Mutex::new(Singleton::new())));
        });

        INSTANCE.clone().unwrap()
    }
}

fn main() {
    let s1 = get_instance();
    let s2 = get_instance();

    // Check if both instances are the same
    let same = Arc::ptr_eq(&s1, &s2);

    println!("Both instances are same: {}", same);
}
