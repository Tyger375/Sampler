#include "intinput.h"

/*
std::string UIIntInput::render()
{
    return (selected ? ">" : "") +  label + ": " + customFormat(value);
}

bool UIIntInput::on_event(const graphics_event_t event)
{
    if (event == EVENT_FOCUS)
    {
        old_value = value;
        return true;
    }
    if (event == EVENT_SCROLL_RIGHT)
    {
        value++;
        if (value > maxValue)
            value = 200;
        return true;
    }
    if (event == EVENT_SCROLL_LEFT)
    {
        value--;
        if (value < minValue)
            value = 60;
        return true;
    }
    if (event == EVENT_OK)
    {
        onDone(value);
        return false;
    }
    if (event == EVENT_BACK)
    {
        value = old_value;
        return false;
    }
    return UIElement::on_event(event);
}

*/