interface TextFormatter {
  format(text: string): string;
}

// <----------- Concrete strategies ------------>
class PlainTextFormatter implements TextFormatter {
  format(text: string): string {
    return text;
  }
}

class BoldTextFormatter implements TextFormatter {
  format(text: string): string {
    return `**${text}**`;
  }
}

class ItalicTextFormatter implements TextFormatter {
  format(text: string): string {
    return `*${text}*`;
  }
}

// Context class
class SimpleTextEditor {
  private formatter: TextFormatter;

  constructor(formatter: TextFormatter) {
    this.formatter = formatter;
  }

  setFormatter(formatter: TextFormatter) {
    this.formatter = formatter;
  }

  display(text: string) {
    console.log(this.formatter.format(text));
  }
}

// Usage
const inputText = "Hello from Typescript!!";

const simpleEditor = new SimpleTextEditor(new PlainTextFormatter());
simpleEditor.display(inputText);

simpleEditor.setFormatter(new BoldTextFormatter());
simpleEditor.display(inputText);

simpleEditor.setFormatter(new ItalicTextFormatter());
simpleEditor.display(inputText);
