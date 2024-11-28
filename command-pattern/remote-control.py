from abc import ABC, abstractmethod


class Command(ABC):
    @abstractmethod
    def execute(self):
        pass


# Receiver class
class Light:
    def on(self):
        print("Light is ON..")

    def off(self):
        print("Light is off")


# Concrete commands (Light ON and OFF)
class LightOnCommand(Command):
    def __init__(self, light: Light):
        self.light = light

    def execute(self):
        self.light.on()


class LightOffCommand(Command):
    def __init__(self, light: Light):
        self.light = light

    def execute(self):
        self.light.off()


# Invoker
class RemoteControl:
    def __init__(self):
        self.command = None

    def set_command(self, command: Command):
        self.command = command

    def press_button(self):
        if self.command:
            self.command.execute()


# Driver code
if __name__ == "__main__":
    light = Light()

    light_on = LightOnCommand(light)
    light_off = LightOffCommand(light)

    remote = RemoteControl()

    # Let's turn on the lights 💡
    remote.set_command(light_on)
    remote.press_button()

    # Turning off now
    remote.set_command(light_off)
    remote.press_button()
