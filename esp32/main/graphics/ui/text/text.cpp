#include "text.h"

#include <utility>

UIText::UIText(std::string text) : text(std::move(text))
{

}

std::string UIText::render()
{
    return (selected ? ">" : "") + text;
}

void UIText::setText(std::string newText)
{
    text = std::move(newText);
}

bool UIText::on_event(const graphics_event_t event)
{
    return UIElement::on_event(event);
}
