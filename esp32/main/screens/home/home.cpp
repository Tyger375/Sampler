#include "home.h"
#include <memory>
#include "screens/screens.h"
#include <esp_log.h>
#include <graphics/graphics_types.h>
#include <graphics/manager/graphics_manager.h>
#include <graphics/screen/screen.h>
#include <graphics/ui/button/button.h>
#include <graphics/ui/row/row.h>
#include <graphics/ui/text/text.h>

screen_t create_home_screen(GraphicsManager& graphics_manager)
{
    /*
    auto screen = std::make_unique<HomeScreen>(gm);
    lcd.load_screen(std::move(screen));
    */
    return std::make_unique<HomeScreen>(graphics_manager);
}

HomeScreen::HomeScreen(GraphicsManager& graphics_manager) : Screen("home")
{
    add_element(std::make_unique<UIText>("TITLE"));

    auto row = std::make_unique<UIRow>();
    ui_button_config_t sequencerBtn{
        .text = "Sequencer",
        .callback = [&graphics_manager]
        {
            graphics_manager.navigate("sequencer");
        }
    };
    row->add_element(std::make_unique<UIButton>(sequencerBtn));

    row->add_element(std::make_unique<UIText>("Test"));

    add_element(std::move(row));

    ui_button_config_t settings_btn{
        .text = "Settings",
        .callback = [&graphics_manager]
        {
            graphics_manager.navigate("settings");
        }
    };
    add_element(std::make_unique<UIButton>(settings_btn));
    /*
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
    */
}

/*
bool HomeScreen::on_custom_event(uint32_t)
{
    return false;
}
*/
