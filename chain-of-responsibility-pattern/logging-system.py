class Logger:
    """Base handler in the chain."""

    def __init__(self, log_level: str):
        self.log_level = log_level
        self.next_handler = None

    def set_next(self, handler):
        """Sets the next handler in the chain."""
        self.next_handler = handler
        return handler  # For chaining calls (when we link the handlers together, we can do a.set_next(b).set_next(c) and so on..)

    def handle(self, level, message):
        """Handles the request if the level matches; otherwise passes it along."""
        if self.log_level == level:
            self._log_message(message)
        elif self.next_handler:
            self.next_handler.handle(level, message)

    def _log_message(self, message):
        """Actual logging logic to be implemented by subclasses."""
        raise NotImplementedError("Subclasses must implement this method")


class InfoLogger(Logger):
    def _log_message(self, message):
        print(f"INFO: {message}")


class DebugLogger(Logger):
    def _log_message(self, message):
        print(f"DEBUG: {message}")


class ErrorLogger(Logger):
    def _log_message(self, message):
        print(f"ERROR: {message}")


# Setting up the "chain of responsibility"
if __name__ == "__main__":
    CONST_INFO = "Info"
    CONST_DEBUG = "Debug"
    CONST_ERROR = "Error"

    # Create handlers
    info_logger = InfoLogger(CONST_INFO)
    debug_logger = DebugLogger(CONST_DEBUG)
    error_logger = ErrorLogger(CONST_ERROR)

    # Link handlers
    info_logger.set_next(debug_logger).set_next(error_logger)

    # Use the chain
    info_logger.handle(CONST_INFO, "This is an informational message.")
    info_logger.handle(CONST_DEBUG, "This is a debug-level message.")
    info_logger.handle(CONST_ERROR, "This is an error message.")
