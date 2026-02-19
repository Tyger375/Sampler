#include "pad_settings.h"

#include <esp_log.h>
#include <graphics/ui/button/button.h>
#include <graphics/ui/intinput/intinput.h>
#include <graphics/ui/text/text.h>
#include <pads/pads.h>
#include <utils/utils.h>

#include "../screens.h"

void create_pad_settings_screen(lcd& lcd)
{
    auto screen = std::make_unique<PadSettingsScreen>();
    lcd.load_screen(std::move(screen));
}

PadSettingsScreen::PadSettingsScreen() : Screen("pad_settings")
{
    select_pad();
}

void PadSettingsScreen::select_pad()
{
    focus = -1;
    elements.clear();

    add_element(std::make_unique<UIText>("Press button"));
}

void PadSettingsScreen::pad_selected()
{
    if (focus < 0) return;
    if (elements.empty()) return;

    auto* title = (UIText*)elements[0].get();
    if (title == nullptr) return;
    title->setText("PAD: " + std::to_string(focus));

    auto note = std::make_unique<UIIntInput>([this](const int value)
    {
        if (focus < 0) return;

        PadsManager::instance().pads_settings[focus].note = value;
    });
    note->minValue = 0;
    note->maxValue = 127;
    note->value = 60;

    note->customFormat = [](const int value)
    {
        return Utils::int_to_note(value);
    };
    add_element(std::move(note));

    add_element(std::make_unique<UIButton>("Save", [this]()
    {
        if (focus < 0) return;

        auto& pm = PadsManager::instance();

        auto* noteInput = (UIIntInput*)elements[1].get();
        ESP_LOGI("PadSettingsScreen", "SAVING NOTE %i", noteInput->value);

        pm.pause_task();
        pm.pads_settings[focus].note = noteInput->value;
        pm.resume_task();

        select_pad();
    }));
}

bool PadSettingsScreen::on_event(graphics_event_t event, uint16_t element)
{
    if (focus >= 0)
    {
        /*if (event == EVENT_FOCUS)
        {
            ESP_LOGI("PadSettingsScreen", "Pad configured");
            select_pad();
            return false;
        }*/
        if (event == EVENT_BACK)
        {
            select_pad();
            return true;
        }
    }
    return Screen::on_event(event, element);
}

bool PadSettingsScreen::on_custom_event(uint32_t event)
{
    const uint8_t channel = event & 0b111;
    const bool long_press = (event & 0b1000) > 0;

    if (!long_press && focus < 0)
    {
        focus = channel;
        pad_selected();
        return true;
    }

    return false;
    //ESP_LOGI("PadSettingsScreen", "%u %i", channel, long_press);
}
