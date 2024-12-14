from abc import ABC, abstractmethod


# The interface that defines the common operations
class Image(ABC):
    @abstractmethod
    def display(self):
        pass


# The Real Subject which does the heavy lifting (e.g., loading a high-resolution image)
class RealImage(Image):
    def __init__(self, filename):
        self.filename = filename
        self._load_image_from_disk()

    def _load_image_from_disk(self):
        print(f"Loading image from disk: {self.filename}")

    def display(self):
        print(f"Displaying image: {self.filename}")


# The Proxy which acts as a placeholder
class ProxyImage(Image):
    def __init__(self, filename):
        self.filename = filename

        # RealImage will be created only when needed (to preserve resources)
        self.real_image = None

    def display(self):
        if self.real_image is None:
            # Lazy initialization
            self.real_image = RealImage(self.filename)
        self.real_image.display()


# Dryrun
if __name__ == "__main__":
    image = ProxyImage("shreyash.jpg")
    print("[DEBUG]: Image created. No loading yet. \n")

    image.display()  # Loads and displays the image

    print("\n[DEBUG]: Displaying again:")
    image.display()  # Doesn't load again, just displays the img
