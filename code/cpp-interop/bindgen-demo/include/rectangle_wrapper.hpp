#ifndef RECTANGLE_WRAPPER_HPP
#define RECTANGLE_WRAPPER_HPP

#ifdef __cplusplus
extern "C"
{
#endif

#include "rectangle.hpp"

Rectangle* rectangle_new(double width, double height);
double rectangle_area(const Rectangle* rect);
void rectangle_free(Rectangle* rect);

#ifdef __cplusplus
}
#endif

#endif // RECTANGLE_WRAPPER_HPP