#ifndef SAMPLER_TYPES_H
#define SAMPLER_TYPES_H

#include <vector>
#include <memory>
#include "ui/element.h"

typedef std::unique_ptr<UIElement> graphics_element_t;
typedef std::vector<graphics_element_t> graphics_elements_t;

#endif //SAMPLER_TYPES_H