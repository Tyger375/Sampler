#include "lcd.h"
#include "esp_log.h"
#include "driver/i2c_types.h"

/*
void lcd::navigate_screen(Screen* screen)
{
    if (current_screen != nullptr)
        current_screen->get_element(row_offset).on_event(EVENT_UNSELECT);

    current_screen = screen;
    focus = -1;
    row_offset = 0;

    if (!current_screen->is_empty())
    {
        current_screen->get_element(row_offset).on_event(EVENT_SELECT);
        current_screen->broadcast_event(EVENT_ON_NAVIGATE);
    }
}

Screen* lcd::find_screen(const std::string& screen_id) const
{
    for (const auto& screen : screens)
    {
        if (screen->id == screen_id)
            return screen.get();
    }

    return nullptr;
}

lcd::lcd(i2c_master_bus_handle_t bus_handle, const uint8_t address)
    : display(bus_handle, address)
{
    user_events = xQueueCreate(10, sizeof(user_event_t));

    display.init();
    display.clear_screen();
}

void lcd::load_screen(std::unique_ptr<Screen> screen)
{
    screens.push_back(std::move(screen));
    *
    row_offset = 0;
    focus = -1;
    screen = new_screen;

    if (!screen.empty())
        screen[row_offset]->on_event(EVENT_SELECT);
    *
}

void lcd::render()
{
    if (current_screen->is_empty()) return;

    uint16_t offset = row_offset - (row_offset % 2);

    if (offset >= current_screen->size())
    {
        row_offset = 0;
        offset = 0;
        current_screen->on_event(EVENT_SELECT, offset);
    }

    //ESP_LOGI("SCREEN", "----------------------");
    display.clear_screen();

    auto& first = current_screen->get_element(offset);
    std::string first_render = first.render();
    display.set_cursor(0, 0);
    display.write(first_render.substr(0, std::min((int)first_render.size(), 16)));

    if (offset + 1 < current_screen->size())
    {
        auto& second = current_screen->get_element(offset + 1);
        std::string second_render = second.render();
        display.set_cursor(0, 1);
        display.write(second_render.substr(0, std::min((int)second_render.size(), 16)));
    }

    //ESP_LOGI("SCREEN", "----------------------");
}

void lcd::navigate(const std::string& screen_id)
{
    auto* screen = find_screen(screen_id);
    if (screen == nullptr)
    {
        ESP_LOGE("LCD", "Couldn't find screen: %s", screen_id.c_str());
        return;
    }

    if (current_screen != nullptr)
        backstack.push_back(current_screen->id);

    navigate_screen(screen);
}

void lcd::back()
{
    if (backstack.empty()) return;

    const auto id = backstack.back();
    backstack.pop_back();

    auto* screen = find_screen(id);
    if (screen == nullptr) return;

    navigate_screen(screen);
}

bool lcd::custom_event(const uint32_t event) const
{
    if (current_screen->is_empty()) return false;

    return current_screen->on_custom_event(event);
}

void lcd::action_left()
{
    if (current_screen->is_empty()) return;

    if (focus < 0)
    {
        current_screen->on_event(EVENT_UNSELECT, row_offset);
        if (row_offset != 0)
            row_offset--;
        current_screen->on_event(EVENT_SELECT, row_offset);
    } else
    {
        current_screen->on_event(EVENT_SCROLL_LEFT, row_offset);
    }
}

void lcd::action_right()
{
    if (current_screen->is_empty()) return;

    if (focus < 0)
    {
        const uint16_t old_offset = row_offset;
        if (!current_screen->on_event(EVENT_UNSELECT, row_offset)) return;

        row_offset++;
        if (row_offset >= current_screen->size())
            row_offset = current_screen->size() - 1;

        if (!current_screen->on_event(EVENT_SELECT, row_offset))
            row_offset = old_offset;
    } else
    {
        current_screen->on_event(EVENT_SCROLL_RIGHT, row_offset);
    }
}

void lcd::action_click()
{
    if (current_screen->is_empty()) return;

    if (focus < 0)
    {
        if (current_screen->on_event(EVENT_FOCUS, row_offset))
            focus = row_offset;
    } else
    {
        if (!current_screen->on_event(EVENT_OK, row_offset))
            focus = -1;
    }
}

void lcd::action_long_click()
{
    if (current_screen->is_empty()) return;

    if (focus < 0)
    {
        if (!current_screen->on_event(EVENT_BACK, row_offset))
            back();
    } else
    {
        if (!current_screen->on_event(EVENT_BACK, row_offset))
            focus = -1;
    }
}
*/