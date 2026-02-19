#include "text.h"

#include <utility>

UIText::UIText(std::string text) : text(std::move(text))
{

}

std::string UIText::render(const bool selected)
{
    return (selected ? ">" : "") + text;
}

void UIText::setText(std::string newText)
{
    text = std::move(newText);
}
