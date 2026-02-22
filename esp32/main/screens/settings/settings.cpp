#include "settings.h"

#include <graphics/ui/button/button.h>
#include <graphics/ui/intinput/intinput.h>
#include <graphics/ui/text/text.h>

#include "../screens.h"
#include "settings/settings_manager.h"
#include "esp_log.h"

screen_t create_settings_screen(GraphicsManager& graphics_manager)
{
    return std::make_unique<SettingsScreen>(graphics_manager);
}

SettingsScreen::SettingsScreen(GraphicsManager& graphics_manager) : Screen("settings")
{
    const auto& settings = SettingsManager::instance();
    add_element(std::make_unique<UIText>("Settings"));

    ui_intinput_config_t bpmSettings{
        .text = "BPM",
        .customFormat = [](const int value)
        {
            return std::to_string(value);
        },
        .onChange = [](const int value)
        {
            if (value > 200)
                return value;
            if (value < 60)
                return value;
            return value;
        },
        .onDone = [](const int value)
        {
            ESP_LOGI("SAMPLER", "BPM SAVING %i", value);
            SettingsManager::instance().save_bpm(value);
        },
    };
    add_element(std::make_unique<UIIntInput>(bpmSettings, settings.bpm));

    ui_button_config_t btnPadSettings{
        .text = "Pad Settings",
        .callback = [&graphics_manager]
        {
            graphics_manager.navigate("pad_settings");
        }
    };
    add_element(std::make_unique<UIButton>(btnPadSettings));
}
