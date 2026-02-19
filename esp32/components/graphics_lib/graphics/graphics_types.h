#ifndef GRAPHICS_TYPES_H
#define GRAPHICS_TYPES_H

#include <vector>
#include <memory>
#include "ui/element.h"

typedef std::unique_ptr<UIElement> graphics_element_t;
typedef std::vector<graphics_element_t> graphics_elements_t;

#endif //GRAPHICS_TYPES_H