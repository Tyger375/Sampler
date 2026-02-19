#ifndef SAMPLER_TEXT_H
#define SAMPLER_TEXT_H

#include "../element.h"

struct UIText : UIElement
{
private:
    std::string text;
public:
    explicit UIText(std::string);

    std::string render() override;
    void setText(std::string);

    bool on_event(graphics_event_t) override;
};

#endif //SAMPLER_TEXT_H