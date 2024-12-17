from lab_python_oop.rectangle import Rectangle
from lab_python_oop.circle import Circle
from lab_python_oop.square import Square
from sympy import Symbol, posify, exp

def main():
    rect = Rectangle(3, 3, "синий")
    circle = Circle(3, "зелёный")
    square = Square(3, "красный")
    print(rect)
    print(circle)
    print(square)
    x = Symbol('x')
    expr = exp(x**2)
    print(posify(expr))

if __name__ == "__main__":
    main()