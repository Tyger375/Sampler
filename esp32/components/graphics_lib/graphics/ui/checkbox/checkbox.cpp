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
UICheckBox::UICheckBox(ui_checkbox_config_t config, const bool defaultValue) : config(std::move(config)), checked(defaultValue)
{
}

std::string UICheckBox::render(const bool selected)
{
    return (selected ? ">" : "") + config.label + ": " + (checked ? "ON" : "OFF");
}

bool UICheckBox::on_event(const graphics_event_t event)
{
    if (event == EVENT_CLICK && config.onChange)
    {
        checked = config.onChange(!checked);
    }
    return UIElement::on_event(event);
}
