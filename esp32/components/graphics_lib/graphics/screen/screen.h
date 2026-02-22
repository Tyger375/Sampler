#ifndef GRAPHICS_SCREEN_H
#define GRAPHICS_SCREEN_H
#include <utility>
#include "graphics/graphics_types.h"
#include "graphics/ui/element.h"

class Screen
{
protected:
    graphics_elements_t elements;
    size_t row_offset{};
    bool focus = false;
public:
    const std::string id;

    explicit Screen(std::string id) : id(std::move(id)) {}
    virtual ~Screen() = default;

    std::vector<std::string> render(uint8_t);

    void add_element(std::unique_ptr<UIElement> element);

    virtual void on_start()
    {
        for (const auto& ui_element : elements)
        {
            ui_element->on_event(EVENT_SCREEN_START);
        }
    }

    virtual void on_end()
    {
        for (const auto& ui_element : elements)
        {
            ui_element->on_event(EVENT_SCREEN_END);
        }
    }

    virtual void on_scroll(bool);
    virtual void on_click();
    virtual bool on_back();
    /*

    const graphics_elements_t& get();

    [[nodiscard]] bool is_empty() const;
    [[nodiscard]] UIElement& get_element(uint64_t) const;
    [[nodiscard]] size_t size() const;

    virtual bool on_event(graphics_event_t, uint16_t);
    virtual bool on_custom_event(uint32_t) { return false; }

    void broadcast_event(const graphics_event_t event) const
    {
        for (const auto& e : elements)
        {
            e->on_event(event);
        }
    }*/
};

typedef std::unique_ptr<Screen> screen_t;

#endif //GRAPHICS_SCREEN_H