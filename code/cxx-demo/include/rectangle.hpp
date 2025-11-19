#ifndef RECTANGLE_HPP
#define RECTANGLE_HPP

#include <memory>
#include <string>

namespace Shapes
{
    class Rectangle
    {
        inline static const std::string id_str = "Shapes::Rectangle";
    public:
        static const std::string& id() { return id_str; }

        Rectangle(double width, double height);
        double width() const { return width_; }
        double height() const { return height_; }
        double area() const;
        void inflate(double factor);

    private:
        double width_;
        double height_;
    };

    std::unique_ptr<Rectangle> new_rectangle(double width, double height);
} // namespace Shapes

#endif // RECTANGLE_HPP