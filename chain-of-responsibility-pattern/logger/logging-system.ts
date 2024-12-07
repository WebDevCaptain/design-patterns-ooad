abstract class Logger {
  private logLevel: string;
  private nextHandler: Logger | null;

  constructor(logLevel: string) {
    this.logLevel = logLevel;
    this.nextHandler = null;
  }

  /**
   * Sets the next handler in the chain of responsibility.
   *
   * @param handler - The next logger handler in the chain
   * @returns The next logger handler, allowing method chaining.
   */
  setNext(handler: Logger): Logger {
    this.nextHandler = handler;
    return handler;
  }

  /**
   * Handles the log message if level matches.
   *
   * If the level doesn't match, it passes the message to the next handler in the chain.
   *
   * @param level - The log level to handle (eg: INFO, DEBUG, etc)
   * @param message - The message to log.
   */
  handle(level: string, message: string): void {
    if (this.logLevel === level) {
      this.logMessage(message);
    } else if (this.nextHandler) {
      console.log("Match failed, passing it along", this.logLevel, level);
      this.nextHandler.handle(level, message);
    }
  }

  // This logic needs to be implemented by the child classes of this base class
  protected abstract logMessage(msg: string): void;
}

class InfoLogger extends Logger {
  protected logMessage(msg: string): void {
    console.info(`[INFO]: ${msg}`);
  }
}

class DebugLogger extends Logger {
  protected logMessage(msg: string): void {
    console.debug(`[DEBUG]: ${msg}`);
  }
}

class ErrorLogger extends Logger {
  protected logMessage(msg: string): void {
    console.error(`[ERROR]: ${msg}`);
  }
}

// Usage
enum LogTypes {
  INFO = "i",
  DEBUG = "d",
  ERROR = "e",
}

// Setup the chain of responsibility
const infoLogger = new InfoLogger(LogTypes.INFO);
const debugLogger = new DebugLogger(LogTypes.DEBUG);
const errorLogger = new ErrorLogger(LogTypes.ERROR);

// Linking handlers
infoLogger.setNext(debugLogger).setNext(errorLogger);

// Using the chain
infoLogger.handle(LogTypes.INFO, "This is an informational message");
infoLogger.handle(LogTypes.DEBUG, "This is a debug-level message");
infoLogger.handle(LogTypes.ERROR, "This is an error message.");
