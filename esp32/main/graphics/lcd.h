#ifndef SAMPLER_LCD_H
#define SAMPLER_LCD_H

#include <memory>
#include "FreeRTOS.h"
#include "queue.h"
#include <vector>
#include "screen/screen.h"
#include "driver/lcd1602.h"

class lcd
{
    lcd1602 display;

    uint16_t row_offset{};
    std::vector<std::unique_ptr<Screen>> screens{};
    std::vector<std::string> backstack{};
    Screen* current_screen = nullptr;

    int focus = -1;

    void navigate_screen(Screen*);

    [[nodiscard]] Screen* find_screen(const std::string&) const;
public:
    QueueHandle_t user_events = nullptr;

    lcd(i2c_master_bus_handle_t, uint8_t);

    void load_screen(std::unique_ptr<Screen>);
    void render();

    void navigate(const std::string&);
    void back();

    bool custom_event(uint32_t) const;

    void action_left();
    void action_right();
    void action_click();
    void action_long_click();
};


#endif //SAMPLER_LCD_H