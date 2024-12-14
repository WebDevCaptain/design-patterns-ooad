from abc import ABC, abstractmethod


# Strategy Interface definition
class TextFormatter(ABC):
    @abstractmethod
    def format(self, text: str) -> str:
        pass


# --- Concrete Strategies ---
class PlainTextFormatter(TextFormatter):
    def format(self, text: str) -> str:
        return text


class BoldTextFormatter(TextFormatter):
    def format(self, text: str) -> str:
        return f"**{text}**"


class ItalicTextFormatter(TextFormatter):
    def format(self, text: str) -> str:
        return f"*{text}*"


# Context Class
class TextEditor:
    def __init__(self, formatter: TextFormatter):
        self.formatter = formatter

    def set_formatter(self, formatter: TextFormatter):
        self.formatter = formatter

    def display(self, text: str):
        print(self.formatter.format(text))


# Usage
inputText = "Hello, World!"

editor = TextEditor(PlainTextFormatter())
editor.display(inputText)

editor.set_formatter(BoldTextFormatter())
editor.display(inputText)

editor.set_formatter(ItalicTextFormatter())
editor.display(inputText)
