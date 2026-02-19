#ifndef GRAPHICS_TEXT_H
#define GRAPHICS_TEXT_H

#include "../element.h"

struct UIText : UIElement
{
private:
    std::string text;
public:
    explicit UIText(std::string);

    std::string render(bool) override;
    void setText(std::string);
};

#endif //GRAPHICS_TEXT_H