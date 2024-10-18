from lab_python_oop.rectangle import Rectangle

class Square(Rectangle):
    figure_type = "Квадрат"
    def __init__(self, width, color):
        super().__init__(width, width, color)
    @classmethod
    def get_type(cls):
        return cls.figure_type
    def __repr__(self):
       return 'Тип: {}, Сторона: {}, Цвет: {}, Площадь: {}'.format(Square.get_type(), self.width, self.color.color, super().get_area())