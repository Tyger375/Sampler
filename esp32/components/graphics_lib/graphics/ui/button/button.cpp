#include "button.h"

#include <utility>

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
UIButton::UIButton(ui_button_config_t config) : config(std::move(config)) {}

std::string UIButton::render(const bool selected)
{
    return (selected ? ">" : "") + config.text;
}

bool UIButton::on_event(const graphics_event_t event)
{
    if (event == EVENT_CLICK && config.callback)
    {
        config.callback();
    }

    return UIElement::on_event(event);
}
