// Memento: Stores the state of the editor.
class Memento {
  private state: string;

  constructor(state: string) {
    this.state = state;
  }

  getState(): string {
    return this.state;
  }
}

// Originator: The object whose state needs to be saved and restored.
class TextEditor {
  private content: string = "";

  write(text: string): void {
    this.content = text;
  }

  save(): Memento {
    // Saves the current state in a memento
    return new Memento(this.content);
  }

  restore(memento: Memento): void {
    // Restores state from a memento
    this.content = memento.getState();
  }

  read(): string {
    return this.content;
  }
}

// Caretaker: manages mementos
const editor = new TextEditor();

console.log("Initial content: ", editor.read());

// Let's write some text
editor.write(
  "Node.js can now run Typescript files natively using an experimental flag."
);
const savedState = editor.save();
console.log("Content after writing: ", editor.read());

// Let's modify the content now
editor.write(
  "But Bun supports typescript without an experimental flag. and what about Deno ?"
);
console.log("Content after modifying: ", editor.read());

// Restoring prev content
editor.restore(savedState);
console.log("Content after restoring: ", editor.read());
