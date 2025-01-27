// Subsystems
struct Amplifier;

impl Amplifier {
    fn on(&self) {
        println!("Amplifier is on");
    }

    fn off(&self) {
        println!("Amplifier is off");
    }
}

struct Projector;

impl Projector {
    fn on(&self) {
        println!("Projector is on");
    }

    fn off(&self) {
        println!("Projector is off");
    }
}

struct Lights;

impl Lights {
    fn dim(&self) {
        println!("Lights are dimmed for the movie.");
    }

    fn brighten(&self) {
        println!("Lights are back to full brightness.");
    }
}

struct Screen;

impl Screen {
    fn down(&self) {
        println!("Screen is lowered.");
    }

    fn up(&self) {
        println!("Screen is raised.");
    }
}

// Facade
struct HomeTheater {
    amplifier: Amplifier,
    projector: Projector,
    lights: Lights,
    screen: Screen,
}

impl HomeTheater {
    fn new() -> Self {
        HomeTheater {
            amplifier: Amplifier,
            projector: Projector,
            lights: Lights,
            screen: Screen,
        }
    }

    fn watch_movie(&self) {
        println!("Preparing to watch a movie now......");
        self.amplifier.on();
        self.projector.on();
        self.lights.dim();
        self.screen.down();
        println!("Movie is ready to play!!!!!");
    }

    fn end_movie(&self) {
        println!("Ending the movie now......");
        self.amplifier.off();
        self.projector.off();
        self.lights.brighten();
        self.screen.up();
        println!("Movie has ended!");
    }
}

fn main() {
    let theatre = HomeTheater::new();
    theatre.watch_movie();
    println!("\n\n--- Movie Finished ---\n");
    theatre.end_movie();
}
