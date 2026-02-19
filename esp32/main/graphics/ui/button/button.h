#ifndef SAMPLER_BUTTON_H
#define SAMPLER_BUTTON_H

#include <functional>
#include "../element.h"

struct UIButton : UIElement
{
private:
    std::string text;
    std::function<void()> onClick;
public:
    UIButton(const std::string&, const std::function<void()>&);

    std::string render() override;
    bool on_event(graphics_event_t) override;
};


#endif //SAMPLER_BUTTON_H