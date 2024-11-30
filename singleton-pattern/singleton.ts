class Singleton {
  private static instance: Singleton;

  // Making the constructor private so that it can't be instantiated directly
  private constructor() {}

  public static getInstance() {
    if (!Singleton.instance) {
      Singleton.instance = new Singleton();
    }

    return Singleton.instance;
  }
}

// Dry run
const s1 = Singleton.getInstance();
const s2 = Singleton.getInstance();

const bothInstancesSame = s1 === s2;

console.log(`\n Both singleton instances are same: ${bothInstancesSame} \n\n`);
