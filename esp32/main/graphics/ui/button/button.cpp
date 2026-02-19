#include "button.h"

UIButton::UIButton(const std::string& text, const std::function<void()>& onClick)
{
    this->text = text;
    this->onClick = onClick;
}

std::string UIButton::render()
{
    return (selected ? ">" : "") + text;
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
