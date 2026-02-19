#include "button.h"

/*
UIButton::UIButton(const std::string& text, const std::function<void()>& onClick)
{
    this->text = text;
    this->onClick = onClick;
}

bool UIButton::on_event(const graphics_event_t event)
{
    if (event == EVENT_FOCUS)
    {
        onClick();
        return false;
    }

    return UIElement::on_event(event);
}

*/
UIButton::UIButton(const ui_button_config_t& config) : config(config) {}

std::string UIButton::render(bool selected)
{
    return (selected ? ">" : "") + config.text;
}

bool UIButton::on_event(const graphics_event_t event)
{
    if (event == EVENT_CLICK)
    {
        if (config.callback)
        {
            config.callback();
        }
    }

    return UIElement::on_event(event);
}
