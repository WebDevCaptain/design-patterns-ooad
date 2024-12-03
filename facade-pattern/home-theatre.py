# ==== Subsystems ==
class Amplifier:
    def on(self):
        print("Amplifier is now ON.")

    def off(self):
        print("Amplifier is now OFF.")


class Projector:
    def on(self):
        print("Projector is now ON.")

    def off(self):
        print("Projector is now OFF.")


class Lights:
    def dim(self):
        print("Lights are dimmed for the movie.")

    def brighten(self):
        print("Lights are back to full brightness.")


class Screen:
    def down(self):
        print("Screen is lowered.")

    def up(self):
        print("Screen is raised.")


# Facade
class HomeTheater:
    def __init__(self):
        self.amplifier = Amplifier()
        self.projector = Projector()
        self.lights = Lights()
        self.screen = Screen()

    def watch_movie(self):
        print("Preparing to watch a movie...")
        self.amplifier.on()
        self.projector.on()
        self.lights.dim()
        self.screen.down()
        print("Movie is ready to play!")

    def end_movie(self):
        print("Shutting down the movie...")
        self.amplifier.off()
        self.projector.off()
        self.lights.brighten()
        self.screen.up()
        print("Movie shutdown complete.")


# Dry run
if __name__ == "__main__":
    theater = HomeTheater()
    theater.watch_movie()
    print("\n\n--- Movie Finished ---\n")
    theater.end_movie()
