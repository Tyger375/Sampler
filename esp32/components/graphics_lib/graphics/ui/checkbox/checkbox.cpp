#include "checkbox.h"

/*
std::string UICheckBox::render()
{
    return (selected ? ">" : "") + label + ": " + (checked ? "ON" : "OFF");
}

bool UICheckBox::on_event(const graphics_event_t event)
{
    if (event == EVENT_FOCUS)
    {
        checked = !checked;
        onChange(checked);
        return false;
    }

    return UIElement::on_event(event);
}
*/