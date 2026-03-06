#ifndef SAMPLER_CHECKBOX_H
#define SAMPLER_CHECKBOX_H
#include <functional>
#include <graphics/ui/element.h>

struct ui_checkbox_config_t
{
    std::string label;
    std::function<bool(bool)> onChange;
};

class UICheckBox : public UIElement
{
    ui_checkbox_config_t config;
public:
    bool checked = false;

    UICheckBox(ui_checkbox_config_t, bool);

    std::string render(bool) override;
    bool on_event(graphics_event_t) override;
};
/*
#include <graphics_test/ui/element.h>
#include <functional>

class UICheckBox : public UIElement
{
    std::function<void(bool)> onChange;

public:
    bool checked;
    std::string label;

    explicit UICheckBox(std::function<void(bool)> onChange)
    {
        this->onChange = std::move(onChange);
    }

    std::string render() override;
    bool on_event(graphics_event_t) override;
};
*/

#endif //SAMPLER_CHECKBOX_H