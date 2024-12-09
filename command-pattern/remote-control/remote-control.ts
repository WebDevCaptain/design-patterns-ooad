interface Command {
  execute(): void;
}

class Light {
  on() {
    console.log("Light turned ON.");
  }

  off() {
    console.log("Light turned OFF.");
  }
}

class LightOnCommand implements Command {
  private light: Light;

  constructor(light: Light) {
    this.light = light;
  }

  execute(): void {
    this.light.on();
  }
}

class LightOffCommand implements Command {
  private light: Light;

  constructor(light: Light) {
    this.light = light;
  }

  execute(): void {
    this.light.off();
  }
}

class RemoteControl {
  private command: Command | null;

  constructor() {
    this.command = null;
  }

  setCommand(command: Command) {
    this.command = command;
  }

  pressButton() {
    if (this.command) {
      this.command.execute();
    }
  }
}

// Usage
const light = new Light();

const lightOnCommand = new LightOnCommand(light);
const lightOffCommand = new LightOffCommand(light);

const remote = new RemoteControl();

// Turning on lights
remote.setCommand(lightOnCommand);
remote.pressButton();

// Enough, now turn it off after 2 sec
setTimeout(() => {
  remote.setCommand(lightOffCommand);
  remote.pressButton();
}, 2000);
