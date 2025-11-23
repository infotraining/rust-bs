#include "rectangle.hpp"

namespace Shapes
{
    Rectangle::Rectangle(double width, double height)
        : width_(width)
        , height_(height)
    {
    }

    double Rectangle::area() const
    {
        return width_ * height_;
    }

    void Rectangle::inflate(double factor)
    {
        width_ *= factor;
        height_ *= factor;
    }

    std::unique_ptr<Rectangle> new_rectangle(double width, double height)
    {
        return std::make_unique<Rectangle>(width, height);
    }
} // namespace Shapes