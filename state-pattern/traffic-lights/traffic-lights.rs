trait TrafficLightState {
    fn switch(self: Box<Self>, traffic_light: &mut TrafficLight);
}

// Concrete states
struct RedLight;
struct GreenLight;
struct YellowLight;

impl TrafficLightState for RedLight {
    fn switch(self: Box<Self>, traffic_light: &mut TrafficLight) {
        println!("Switching from Red to Green");
        // traffic_light.state = Box::new(GreenLight);
        traffic_light.set_state(Box::new(GreenLight));
    }
}

impl TrafficLightState for GreenLight {
    fn switch(self: Box<Self>, traffic_light: &mut TrafficLight) {
        println!("Switching from Green to Yellow");
        // traffic_light.state = Box::new(YellowLight);
        traffic_light.set_state(Box::new(YellowLight));
    }
}

impl TrafficLightState for YellowLight {
    fn switch(self: Box<Self>, traffic_light: &mut TrafficLight) {
        println!("Switching from Yellow to Red");
        // traffic_light.state = Box::new(RedLight);
        traffic_light.set_state(Box::new(RedLight));
    }
}

// Context
struct TrafficLight {
    state: Box<dyn TrafficLightState>,
}

impl TrafficLight {
    fn new() -> Self {
        Self {
            state: Box::new(RedLight), // Initially red light is on
        }
    }

    fn set_state(&mut self, state: Box<dyn TrafficLightState>) {
        self.state = state;
    }

    fn switch(&mut self) {
        let current_state = std::mem::replace(&mut self.state, Box::new(RedLight)); // Returns the old state and puts a placeholder in place, to satisfy the borrow checker
        current_state.switch(self);
    }
}

fn main() {
    let mut traffic_light = TrafficLight::new();

    for _ in 0..5 {
        // Let's simulate 6 state transitions
        traffic_light.switch();
    }
}
