# Stores the state of the text editor
class Memento:
  def __init__(self, state):
    self.state = state

# Originator: The object whose state needs to be saved and restored
class TextEditor:
  def __init__(self):
    self._content = """"""

  def write(self, text):
    self._content = text

  def save(self):
    return Memento(self._content)

  def restore(self, memento):
    self._content = memento.state

  def read(self):
    return self._content


# Caretaker: manages Mementos
if __name__ == '__main__':
  editor = TextEditor()
  print('Initial content: ', editor.read())

  # Let's write something in the editor and save it
  editor.write('Rust for systems, Go for apps')
  saved_state = editor.save() # memento is returned
  print(f'Content after writing: {editor.read()}')

  # Let's change the content now
  editor.write('Ok, but Rust takes way too long to write, but gives a peace of mind in the long run 😅')
  print(f'Content after modifying: {editor.read()}')

  # Restoring prev content (undo)
  editor.restore(saved_state)
  print(f'Content after restoring: {editor.read()}')
  