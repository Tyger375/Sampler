#ifndef SAMPLER_INTINPUT_H
#define SAMPLER_INTINPUT_H

#include <functional>
#include "../element.h"

struct ui_intinput_config_t
{
    std::string text;
    std::function<std::string(int)> formatValue;
    std::function<int(int)> onChange;
    std::function<void(int)> onDone;
};

struct UIIntInput : UIElement
{
private:
    bool focus = false;

    int old_value{};
    int value{};
    ui_intinput_config_t config;
public:
    static ui_intinput_config_t defaultConfig();

    UIIntInput(ui_intinput_config_t, int);

    std::string render(bool) override;
    bool on_event(graphics_event_t) override;
};

#endif //SAMPLER_INTINPUT_H