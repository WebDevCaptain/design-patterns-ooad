#[derive(Clone, Debug)]
struct Circle {
    radius: f64,
}

#[derive(Clone, Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    // Original circle
    let c1 = Circle { radius: 10.0 };
    println!("Original circle: {:?}", c1);

    // Cloned circle
    let mut c2 = c1.clone();
    c2.radius = 20.0;
    println!("Cloned circle: {:?}, Original: {:?}", c2, c1);

    // Original rectangle
    let r1 = Rectangle {
        width: 20.0,
        height: 30.0,
    };
    println!("Original rectangle: {:?}", r1);

    // Cloned rectangle
    let mut r2 = r1.clone();
    r2.width = 40.0;
    r2.height = 50.0;
    println!("Cloned rectangle: {:?}, Original rectangle: {:?}", r2, r1);
}
