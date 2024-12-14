from abc import ABC, abstractmethod


# Abstract State
class TrafficLightState(ABC):
    @abstractmethod
    def switch(self, traffic_light):
        pass


# ------------ Concrete States -----
class RedLight(TrafficLightState):
    def switch(self, traffic_light):
        print("Switching from Red to Green")
        traffic_light.set_state(GreenLight())


class GreenLight(TrafficLightState):
    def switch(self, traffic_light):
        print("Switching from GREEN to YELLOW")
        traffic_light.set_state(YellowLight())


class YellowLight(TrafficLightState):
    def switch(self, traffic_light):
        print("Switching from Yellow to RED.")
        traffic_light.set_state(RedLight())


# Context
class TrafficLight:
    def __init__(self):
        self.state = RedLight()  # Initial state

    def set_state(self, state: TrafficLightState):
        self.state = state

    def switch(self):
        self.state.switch(self)


# Example usage
if __name__ == "__main__":
    traffic_light = TrafficLight()

    for _ in range(5):  # Let's simulate 6 state transitions
        traffic_light.switch()
