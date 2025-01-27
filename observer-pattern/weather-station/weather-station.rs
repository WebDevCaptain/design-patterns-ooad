use std::cell::RefCell;
use std::rc::Rc;

// Abstract Observer
trait Observer {
    fn update(&self, temperature: u32);
}

// Concrete Observer
struct DisplayDevice;

impl Observer for DisplayDevice {
    fn update(&self, temperature: u32) {
        println!(
            "Display Device: Temperature updated to {} degrees.",
            temperature
        );
    }
}

// Subject
struct WeatherStation {
    temperature: u32,
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl WeatherStation {
    fn new() -> Self {
        Self {
            temperature: 0,
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|obs| !Rc::ptr_eq(obs, &observer));
    }

    fn notify_observers(&self) {
        for obs in &self.observers {
            obs.borrow().update(self.temperature);
        }
    }

    fn set_temperature(&mut self, temperature: u32) {
        println!(
            "Weather Station: Before updating temperature to {} degrees.",
            temperature
        );
        self.temperature = temperature;
        self.notify_observers();
    }
}

fn main() {
    let mut station = WeatherStation::new();
    let display_device = Rc::new(RefCell::new(DisplayDevice));

    station.add_observer(display_device.clone());

    station.set_temperature(25);

    station.set_temperature(32);

    station.remove_observer(display_device.clone());

    println!("Removed display device as an observer, updating temp again.");
    station.set_temperature(28);

    println!("Adding display device as an observer again.");
    station.add_observer(display_device.clone());

    station.set_temperature(30);
}
