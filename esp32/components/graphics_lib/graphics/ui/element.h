#ifndef GRAPHICS_ELEMENT_H
#define GRAPHICS_ELEMENT_H

#include "event.h"
#include <string>

extern uint64_t ELEMENT_IDS;

struct UIElement
{
    const uint64_t id;
    //bool selected = false;

    UIElement() : id(ELEMENT_IDS++)
    {
    }

    virtual ~UIElement() = default;
    virtual std::string render(bool) = 0;
    /**
     * @return Requesting focus
     */
    virtual bool on_event(graphics_event_t) { return false; }
};

#endif //GRAPHICS_ELEMENT_H