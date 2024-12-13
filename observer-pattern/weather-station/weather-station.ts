interface Observer {
  update(temperature: number): void;
}

// Concrete observer
class DisplayDevice implements Observer {
  update(temperature: number): void {
    console.log(
      `Display (observer): Temperature updated to ${temperature} degrees.`
    );
  }
}

// Subject
class WeatherStation {
  private observers: Observer[] = [];
  private temperature = 0;

  addObserver(obs: Observer) {
    this.observers.push(obs);
  }

  removeObserver(obs: Observer) {
    this.observers = this.observers.filter((o) => o != obs);
  }

  notifyObservers() {
    for (let obs of this.observers) {
      obs.update(this.temperature);
    }
  }

  setTemperature(temp: number) {
    console.info(`Setting temperature to ${temp} deg.`);

    this.temperature = temp;
    this.notifyObservers();
  }
}

// Dry run
const station = new WeatherStation();
const displayDevice = new DisplayDevice();

station.addObserver(displayDevice); // Registering the display device as an observer.

for (let i = 0; i < 3; ++i) {
  station.setTemperature(31 * i + 5);
}
