#ifndef SAMPLER_ROW_H
#define SAMPLER_ROW_H

#include <graphics/graphics_types.h>
#include <graphics/ui/element.h>
#include <graphics/ui/event.h>

class UIRow : public UIElement
{
    graphics_elements_t elements;

    bool focus = false;
    size_t offset = 0;
public:
    void add_element(graphics_element_t element);

    std::string render(bool) override;
    bool on_event(graphics_event_t) override;
};

#endif //SAMPLER_ROW_H