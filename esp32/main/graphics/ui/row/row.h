#ifndef SAMPLER_ROW_H
#define SAMPLER_ROW_H

#include "../../types.h"

class UIRow : public UIElement
{
    graphics_elements_t elements;
    int focus = -1;
    uint8_t offset = 0;
public:
    void add_element(graphics_element_t element);

    std::string render() override;
    bool on_event(graphics_event_t) override;
};


#endif //SAMPLER_ROW_H