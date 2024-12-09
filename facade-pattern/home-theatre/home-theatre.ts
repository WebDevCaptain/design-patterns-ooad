// Sub-systems
class Amplifier {
  on(): void {
    console.log("Amplifier is now ON.");
  }

  off(): void {
    console.log("Amplifier is now OFF.");
  }
}

class Projector {
  on(): void {
    console.log("Projector is now ON.");
  }

  off(): void {
    console.log("Projector is now OFF.");
  }
}

class Lights {
  dim(): void {
    console.log("Lights are dimmed for the movie.");
  }

  brighten(): void {
    console.log("Lights are back to full brightness.");
  }
}

class TVScreen {
  down(): void {
    console.log("Screen is lowered.");
  }

  up(): void {
    console.log("Screen is raised.");
  }
}

// Facade
class HomeTheatre {
  private amplifier: Amplifier;
  private projector: Projector;
  private lights: Lights;
  private screen: TVScreen;

  constructor() {
    this.amplifier = new Amplifier();
    this.projector = new Projector();
    this.lights = new Lights();
    this.screen = new TVScreen();
  }

  watchMovie(): void {
    console.log("Preparing to watch a movie now......");
    this.amplifier.on();
    this.projector.on();
    this.lights.dim();
    this.screen.down();
    console.log("Movie is ready to play!!!!!");
  }

  endMovie(): void {
    console.log("Shutting down movie now.");
    this.amplifier.off();
    this.projector.off();
    this.lights.brighten();
    this.screen.up();
    console.log("Movie shutdown completed.");
  }
}

const theatre = new HomeTheatre();
theatre.watchMovie();

console.log("\n \nTrying to end.....");
theatre.endMovie();
