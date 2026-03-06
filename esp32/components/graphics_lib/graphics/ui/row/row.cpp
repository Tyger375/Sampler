#include "row.h"

#include <esp_log.h>

void UIRow::add_element(graphics_element_t element)
{
    elements.push_back(std::move(element));
}

std::string UIRow::render(const bool selected)
{
    std::string text = (selected ? ">" : "");

    if (offset < elements.size())
    {
        text += elements[offset]->render(focus);
        text += " ";
    }
    if (offset + 1 < elements.size())
    {
        text += elements[offset + 1]->render(false);
    }

    return text.substr(0, 16);
}

bool UIRow::on_event(const graphics_event_t event)
{
    switch (event)
    {
    case EVENT_CLICK:
        if (!focus)
        {
            focus = true;
        } else
        {
            elements[offset]->on_event(event);
        }
        return true;
    case EVENT_BACK:
        if (focus)
        {
            focus = false;
            return true;
        }
        return false;
    case EVENT_SCROLL_RIGHT:
        offset = std::min(offset + 1, elements.size() - 1);
        return false;
    case EVENT_SCROLL_LEFT:
        offset = std::max(offset - 1, 0U);
        return false;
    default:
        return UIElement::on_event(event);
    }
    /*
     * OLD IMPLEMENTATION
    if (event == EVENT_ON_NAVIGATE)
    {
        elements[offset]->on_event(EVENT_UNSELECT);
        focus = -1;
        offset = 0;
        return false;
    }
    if (focus >= 0)
    {
        if (event == EVENT_UNSELECT)
        {
            selected = false;
            return true;
        }
        if (event == EVENT_BACK)
        {
            focus = -1;
            elements[offset]->on_event(EVENT_UNSELECT);
            return false;
        }
        if (event == EVENT_SCROLL_LEFT)
        {
            elements[offset]->on_event(EVENT_UNSELECT);
            if (offset != 0)
                offset--;
            elements[offset]->on_event(EVENT_SELECT);
            return false;
        }
        if (event == EVENT_SCROLL_RIGHT)
        {
            const uint16_t old_offset = offset;
            if (!elements[offset]->on_event(EVENT_UNSELECT)) return false;

            offset++;
            if (offset >= elements.size())
                offset = elements.size() - 1;

            if (!elements[offset]->on_event(EVENT_SELECT))
                offset = old_offset;
            return false;
        }

        if (event == EVENT_OK)
            event = EVENT_FOCUS;
        return elements[focus]->on_event(event);
    }
    if (event == EVENT_FOCUS)
    {
        if (elements.empty()) return false;

        focus = 0;
        elements[focus]->on_event(EVENT_SELECT);
        return true;
    }
    */
    return UIElement::on_event(event);
}
