#include "home.h"
#include "../screens.h"
#include "graphics/ui/text/text.h"
#include "graphics/ui/button/button.h"
#include <esp_log.h>
#include <graphics/ui/row/row.h>

void create_home_screen(lcd& lcd)
{
    auto screen = std::make_unique<HomeScreen>(lcd);
    lcd.load_screen(std::move(screen));
}

HomeScreen::HomeScreen(lcd& lcd) : Screen("home")
{
    auto title = std::make_unique<UIText>("TITLE");
    add_element(std::move(title));

    auto row = std::make_unique<UIRow>();
    row->add_element(std::make_unique<UIButton>("Sequencer", [&lcd]
    {
        lcd.navigate("sequencer");
    }));
    row->add_element(std::make_unique<UIText>("Test"));

    add_element(std::move(row));

    auto settings_btn = std::make_unique<UIButton>("Settings", [&lcd]
    {
        lcd.navigate("settings");
    });
    add_element(std::move(settings_btn));
}

bool HomeScreen::on_custom_event(uint32_t)
{
    return false;
}
