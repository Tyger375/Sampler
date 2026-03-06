#include "screen.h"

void Screen::add_element(std::unique_ptr<UIElement> element)
{
    elements.push_back(std::move(element));
}

void Screen::on_scroll(const bool direction)
{
    if (focus)
    {
        elements[row_offset]->on_event(
            direction ? EVENT_SCROLL_RIGHT : EVENT_SCROLL_LEFT
        );
    } else
    {
        if (direction)
        {
            row_offset = std::min(row_offset + 1, elements.size() - 1);
        }
        else
        {
            if (row_offset == 0) return;
            row_offset--;
        }
    }
}

void Screen::on_click()
{
    focus = elements[row_offset]->on_event(EVENT_CLICK);
}

bool Screen::on_back()
{
    if (focus)
    {
        focus = false;
        return elements[row_offset]->on_event(EVENT_BACK);
    }
    return false;
}

bool Screen::on_custom_event(uint32_t)
{
    return false;
}

/*
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
*/

std::vector<std::string> Screen::render(const uint8_t rows_size)
{
    std::vector<std::string> rows_render{};

    const size_t offset = row_offset - (row_offset % 2);

    for (size_t i = offset; i < std::min(offset + rows_size, elements.size()); ++i)
    {
        rows_render.push_back(elements[i]->render(i == row_offset));
    }
    return rows_render;
}
