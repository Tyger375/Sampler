#ifndef SAMPLER_SCREEN_H
#define SAMPLER_SCREEN_H
#include <utility>
#include "../types.h"
#include "../ui/element.h"

class Screen
{
protected:
    graphics_elements_t elements;
public:
    virtual ~Screen() = default;
    const std::string id;

    void add_element(std::unique_ptr<UIElement> element);
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
    }

    explicit Screen(std::string id) : id(std::move(id))
    {}
};


#endif //SAMPLER_SCREEN_H