from lab_python_oop.geometric_figure import geometric_figure
from lab_python_oop.figure_color import figure_color

class Rectangle(geometric_figure):
    figure_type = "Прямоугольник"
    def __init__(self, width, height, color_param):
        self.width = width
        self.height = height
        self.color = figure_color()
        self.color.color = color_param
    def get_area(self):
        return self.width * self.height
    @classmethod
    def get_type(cls):
        return cls.figure_type
    def __repr__(self):
       return 'Тип: {}, Ширина: {}, Высота: {}, Цвет: {}, Площадь: {}'.format(Rectangle.get_type(), self.width, self.height, self.color.color, self.get_area())