// Memento: Stores the state of the text editor
struct Memento {
    state: String,
}

impl Memento {
    fn new(state: &str) -> Self {
        Self {
            state: state.to_string(),
        }
    }

    fn get_state(&self) -> &str {
        &self.state
    }
}

// Originator: Manages the state of the text editor
struct TextEditor {
    content: String,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn write(&mut self, text: &str) {
        self.content = text.to_string();
    }

    fn save(&self) -> Memento {
        // Saves the current state in a memento
        Memento::new(&self.content)
    }

    fn restore(&mut self, memento: &Memento) {
        self.content = memento.get_state().to_string();
    }

    fn read(&self) -> &str {
        &self.content
    }
}

// Caretaker: Manages the mementos
fn main() {
    let mut editor = TextEditor::new();
    println!("Initial content: {}", editor.read());

    // Write and save text
    editor.write("Rust is awesome");
    let saved_state = editor.save();
    println!("Content after writing: {}", editor.read());

    // Modify content
    editor.write("Rust fixes C++ for beginners");
    println!("Content after modifying: {}", editor.read());

    // Restore prev content
    editor.restore(&saved_state);
    println!("Content after restoring: {}", editor.read());
}
