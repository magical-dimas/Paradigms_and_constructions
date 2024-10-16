class figure_color:
    def __init__(self):
        self._color = None
    @property
    def color(self):
        return self._color
    @color.setter
    def color(self, color):
        self._color = color