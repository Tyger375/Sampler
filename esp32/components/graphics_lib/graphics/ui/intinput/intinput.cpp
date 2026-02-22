#include "intinput.h"

ui_intinput_config_t UIIntInput::defaultConfig()
{
    return {
        .text = "",
        .customFormat = [](const int value) { return std::to_string(value); },
        .onChange = [](const int value) { return value; },
        .onDone = [](const int _) {}
    };
}

UIIntInput::UIIntInput(ui_intinput_config_t config, const int defaultValue)
    : value(defaultValue), config(std::move(config))
{

}


std::string UIIntInput::render(const bool selected)
{
    return (selected ? ">" : "") +  config.text + ": " + config.customFormat(value);
}

bool UIIntInput::on_event(const graphics_event_t event)
{
    switch (event)
    {
    case EVENT_CLICK:
        focus = !focus;
        old_value = value;
        if (!focus)
            config.onDone(value);
        return focus;
    case EVENT_BACK:
        value = old_value;
        return focus;
    case EVENT_SCROLL_RIGHT:
        value = config.onChange(value + 1);
        return false;
    case EVENT_SCROLL_LEFT:
        value = config.onChange(value - 1);
        return false;
    default:
        return UIElement::on_event(event);
    }
    /*
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
    */
}
