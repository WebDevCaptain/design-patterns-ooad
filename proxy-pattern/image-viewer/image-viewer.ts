interface Image {
  display(): void;
}

class RealImage implements Image {
  private filename: string;

  constructor(filename: string) {
    this.filename = filename;
    this.loadImageFromDisk();
  }

  display(): void {
    console.log(`Displaying image: ${this.filename}`);
  }

  loadImageFromDisk() {
    console.log(`Loading image from disk: ${this.filename}`);
  }
}

class ProxyImage implements Image {
  private filename: string;
  private realImage: RealImage | null;

  constructor(filename: string) {
    this.filename = filename;
    this.realImage = null;
  }

  display(): void {
    if (!this.realImage) {
      this.realImage = new RealImage(this.filename);
    }

    this.realImage?.display();
  }
}

// Dry run
const image = new ProxyImage("shreyash.png");
console.log("[DEBUG]: Image created, not loading yet.");
image.display();

console.log("\n[DEBUG]: Displaying again:");
image.display();
