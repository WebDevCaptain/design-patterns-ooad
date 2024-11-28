from abc import ABC, abstractmethod

# Observer (interface)
class Observer(ABC):
  @abstractmethod
  def update(self, temp: int):
    """React to temperature updates"""
    raise NotImplementedError

# Concrete observer: Say a display device
class DisplayDevice(Observer):
  def update(self, temperature: int):
    print(f'Display device (observer): Temperature updated to {temperature} degrees')
    # We can trigger some code here to update the display value according to the value of temperature received.
    # print('Display pixels updated')


# Subject
class WeatherStation:
  def __init__(self):
    self._observers = []
    self._temperature = 0

  def add_observer(self, observer: Observer):
    self._observers.append(observer)

  def remove_observer(self, observer: Observer):
    self._observers.remove(observer)

  def notify_observers(self):
    """Notify all observers about the temperature change"""
    for obs in self._observers:
      obs.update(self._temperature)

  def set_temperature(self, temp: int):
    """Set the temperature and notify observers"""
    print(f'Setting temperature to {temp} degrees')
    self._temperature = temp
    self.notify_observers()



# Dry run
if __name__ == '__main__':
  station = WeatherStation()
  display_device = DisplayDevice()

  station.add_observer(display_device) # Register display device as an observer
  
  import time
  import random

  for i in range(3):
    sleep_time = random.randint(1, 3) * i

    station.set_temperature(14 * i + 3)

    print(f'[DEBUG]: Sleeping for {sleep_time} seconds \n\n')
    time.sleep(sleep_time)