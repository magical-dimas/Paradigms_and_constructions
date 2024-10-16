from abc import ABC, abstractmethod

class geometric_figure(ABC):
    @abstractmethod
    def get_area(self):
        pass