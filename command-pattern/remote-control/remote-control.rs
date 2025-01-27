trait Command {
    fn execute(&self);
}

// Receiver
struct Light;

impl Light {
    fn on(&self) {
        println!("Light is on");
    }

    fn off(&self) {
        println!("Light is off");
    }
}

// Concrete Commands
struct LightOnCommand<'a> {
    light: &'a Light,
}

impl<'a> LightOnCommand<'a> {
    fn new(light: &'a Light) -> Self {
        Self { light }
    }
}

impl<'a> Command for LightOnCommand<'a> {
    fn execute(&self) {
        self.light.on();
    }
}

struct LightOffCommand<'a> {
    light: &'a Light,
}

impl<'a> LightOffCommand<'a> {
    fn new(light: &'a Light) -> Self {
        Self { light }
    }
}

impl<'a> Command for LightOffCommand<'a> {
    fn execute(&self) {
        self.light.off();
    }
}

// Invoker
struct RemoteControl<'a> {
    command: Option<Box<dyn Command + 'a>>,
}

impl<'a> RemoteControl<'a> {
    fn new() -> Self {
        Self { command: None }
    }

    fn set_command(&mut self, command: Box<dyn Command + 'a>) {
        self.command = Some(command);
    }

    fn press_button(&self) {
        if let Some(command) = &self.command {
            command.execute();
        }
    }
}

fn main() {
    let light = Light;

    let light_on_command = Box::new(LightOnCommand::new(&light));
    let light_off_command = Box::new(LightOffCommand::new(&light));

    let mut remote_control = RemoteControl::new();

    // Turn on the light
    remote_control.set_command(light_on_command);
    remote_control.press_button();

    // Turn off the light
    remote_control.set_command(light_off_command);
    remote_control.press_button();
}
