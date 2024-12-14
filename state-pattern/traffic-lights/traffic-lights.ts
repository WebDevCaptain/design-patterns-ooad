interface TrafficLightState {
  switchState(trafficLight: TrafficLight): void;
}

// <======= Concrete states =============>
class RedLight implements TrafficLightState {
  switchState(trafficLight: TrafficLight): void {
    console.log(`Switching from Red to Green light`);
    trafficLight.setState(new GreenLight());
  }
}

class GreenLight implements TrafficLightState {
  switchState(trafficLight: TrafficLight): void {
    console.log(`Switching from Green to Yellow light`);
    trafficLight.setState(new YellowLight());
  }
}

class YellowLight implements TrafficLightState {
  switchState(trafficLight: TrafficLight): void {
    console.log(`Switching from Yellow to Red light`);
    trafficLight.setState(new RedLight());
  }
}

// Context
class TrafficLight {
  private state: TrafficLightState;

  constructor() {
    this.state = new RedLight(); // Initial state
  }

  setState(state: TrafficLightState) {
    this.state = state;
  }

  switchState() {
    this.state.switchState(this);
  }
}

// Usage
const trafficLight = new TrafficLight();

for (let i = 0; i < 5; i++) {
  trafficLight.switchState();
}
