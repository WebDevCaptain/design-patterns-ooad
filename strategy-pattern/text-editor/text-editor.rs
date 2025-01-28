// Strategy
trait TextFormatter {
    fn format(&self, text: &str) -> String;
}

// Concrete strategy
struct PlainTextFormatter;
struct BoldTextFormatter;
struct ItalicTextFormatter;

impl TextFormatter for PlainTextFormatter {
    fn format(&self, text: &str) -> String {
        text.to_string()
    }
}

impl TextFormatter for BoldTextFormatter {
    fn format(&self, text: &str) -> String {
        format!("**{}**", text)
    }
}

impl TextFormatter for ItalicTextFormatter {
    fn format(&self, text: &str) -> String {
        format!("*{}*", text)
    }
}

// Context struct
struct TextEditor<'a> {
    formatter: Box<dyn TextFormatter + 'a>,
}

impl<'a> TextEditor<'a> {
    fn new(formatter: Box<dyn TextFormatter + 'a>) -> Self {
        Self { formatter }
    }

    fn set_formatter(&mut self, formatter: Box<dyn TextFormatter + 'a>) {
        self.formatter = formatter;
    }

    fn display(&self, text: &str) {
        println!("{}", self.formatter.format(text));
    }
}

// Usage
fn main() {
    let mut editor = TextEditor::new(Box::new(PlainTextFormatter));
    editor.display("Hello, World!");

    editor.set_formatter(Box::new(BoldTextFormatter));
    editor.display("Rust makes safety fast and elegant");

    editor.set_formatter(Box::new(ItalicTextFormatter));
    editor.display("Memory safety, zero-cost abstraction, pure power");
}
