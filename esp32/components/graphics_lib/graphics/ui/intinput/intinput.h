#ifndef SAMPLER_INTINPUT_H
#define SAMPLER_INTINPUT_H

/*
#include <functional>
#include "../element.h"

struct UIIntInput : UIElement
{
private:
    int old_value{};
    std::function<void(int)> onDone;
public:
    std::function<std::string(int)> customFormat;

    std::string label;
    int value{};
    int maxValue{};
    int minValue{};

    explicit UIIntInput(std::function<void(int)> onDone)
    {
        this->onDone = std::move(onDone);
        this->customFormat = [](const int val) {
            return std::to_string(val);
        };
    }

    std::string render() override;
    bool on_event(graphics_event_t) override;
};
*/

#endif //SAMPLER_INTINPUT_H