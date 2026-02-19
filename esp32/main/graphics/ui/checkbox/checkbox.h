#ifndef SAMPLER_CHECKBOX_H
#define SAMPLER_CHECKBOX_H

#include <graphics/ui/element.h>
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


#endif //SAMPLER_CHECKBOX_H