#ifndef RECTANGLE_HPP
#define RECTANGLE_HPP

class Rectangle
{
public:
    Rectangle(double width, double height) : width(width), height(height) {}
    double area() const { return width * height; }

private:
    double width;
    double height;
};

#endif // RECTANGLE_HPP