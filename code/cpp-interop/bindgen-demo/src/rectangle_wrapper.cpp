#include "rectangle_wrapper.hpp"

extern "C"
{
    Rectangle* rectangle_new(double width, double height)
    {
        return new Rectangle(width, height);
    }

    double rectangle_area(const Rectangle* rect)
    {
        return rect->area();
    }

    void rectangle_free(Rectangle* rect)
    {
        delete rect;
    }
}
