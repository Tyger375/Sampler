#ifndef SAMPLER_ELEMENT_H
#define SAMPLER_ELEMENT_H

#include "event.h"
#include <string>

extern uint64_t ELEMENT_IDS;

struct UIElement
{
    const uint64_t id;
    bool selected = false;

    UIElement() : id(ELEMENT_IDS++)
    {
    }

    virtual ~UIElement() = default;
    virtual std::string render() = 0;
    virtual bool on_event(const graphics_event_t event)
    {
        if (event == EVENT_SELECT)
        {
            selected = true;
            return true;
        }
        if (event == EVENT_UNSELECT)
        {
            selected = false;
            return true;
        }
        return false;
    }
};

#endif //SAMPLER_ELEMENT_H