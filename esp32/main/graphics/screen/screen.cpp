#include "screen.h"

void Screen::add_element(std::unique_ptr<UIElement> element)
{
    elements.push_back(std::move(element));
}

const std::vector<std::unique_ptr<UIElement>>& Screen::get()
{
    return elements;
}

bool Screen::is_empty() const
{
    return elements.empty();
}

UIElement& Screen::get_element(const uint64_t offset) const
{
    return *elements.at(offset);
}

size_t Screen::size() const
{
    return elements.size();
}

bool Screen::on_event(const graphics_event_t event, const uint16_t element)
{
    return elements[element]->on_event(event);
}
