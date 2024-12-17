from lab_python_oop.geometric_figure import geometric_figure
from lab_python_oop.figure_color import figure_color
import math

class Circle(geometric_figure):
    figure_type = "Круг"
    def __init__(self, radius, color):
        self.radius = radius
        self.color = figure_color()
        self.color.color = color
    def get_area(self):
        return math.pi*self.radius*self.radius
    @classmethod
    def get_type(cls):
        return cls.figure_type
    def __repr__(self):
       return 'Тип: {}, Радиус: {}, Цвет: {}, Площадь: {}'.format(Circle.get_type(), self.radius, self.color.color, self.get_area())