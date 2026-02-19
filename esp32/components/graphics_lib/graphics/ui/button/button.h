#ifndef GRAPHICS_BUTTON_H
#define GRAPHICS_BUTTON_H


#include <functional>
#include "../element.h"

struct ui_button_config_t
{
    std::string text;
    std::function<void()> callback;
};

struct UIButton : UIElement
{
private:
    ui_button_config_t config;
public:
    explicit UIButton(const ui_button_config_t&);

    std::string render(bool) override;
    bool on_event(graphics_event_t) override;
};


#endif //GRAPHICS_BUTTON_H