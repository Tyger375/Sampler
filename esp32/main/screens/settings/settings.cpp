#include "settings.h"

#include <graphics/ui/button/button.h>
#include <graphics/ui/intinput/intinput.h>

#include "../screens.h"
#include "settings/settings_manager.h"
#include "graphics/ui/text/text.h"
#include "graphics/ui/intinput/intinput.h"
#include "esp_log.h"

void create_settings_screen(lcd& lcd)
{
    auto screen = std::make_unique<SettingsScreen>(lcd);
    lcd.load_screen(std::move(screen));
}

SettingsScreen::SettingsScreen(lcd& lcd) : Screen("settings")
{
    const auto& settings = SettingsManager::instance();

    auto title = std::make_unique<UIText>("Settings");
    add_element(std::move(title));

    auto bpmSetting = std::make_unique<UIIntInput>([](const int value) {
        ESP_LOGI("SAMPLER", "BPM SAVING %i", value);
        SettingsManager::instance().save_bpm(value);
    });
    bpmSetting->label = "BPM";
    bpmSetting->minValue = 60;
    bpmSetting->maxValue = 200;
    bpmSetting->value = settings.bpm;
    add_element(std::move(bpmSetting));

    auto btnPadSettings = std::make_unique<UIButton>("Pad Settings", [&lcd]()
    {
        lcd.navigate("pad_settings");
    });
    add_element(std::move(btnPadSettings));
}

bool SettingsScreen::on_custom_event(uint32_t event)
{
    return false;
}
