#include "pad_settings.h"

#include <esp_log.h>
#include <graphics/ui/button/button.h>
#include <graphics/ui/intinput/intinput.h>
#include <graphics/ui/text/text.h>
#include <pads/pads.h>
#include <settings/manager.h>
#include <settings/pads/pads_component.h>
#include <utils/utils.h>
#include "../screens.h"

screen_t create_pad_settings_screen(GraphicsManager& /*graphics_manager*/)
{
    return std::make_unique<PadSettingsScreen>();
}

PadSettingsScreen::PadSettingsScreen() : Screen("pad_settings")
{}

void PadSettingsScreen::on_start()
{
    Screen::on_start();

    PadsManager::instance().is_enabled = false;

    select_pad();
}

void PadSettingsScreen::on_end()
{
    Screen::on_end();

    PadsManager::instance().is_enabled = true;
}

bool PadSettingsScreen::on_back()
{
    if (pageFocus >= 0)
    {
        select_pad();
        return true;
    }

    return Screen::on_back();
}

bool PadSettingsScreen::on_custom_event(const uint32_t event)
{
    const uint8_t channel = event & 0b111;
    const bool long_press = (event & 0b1000) > 0;

    if (!long_press && pageFocus < 0)
    {
        pageFocus = channel;
        pad_selected();
        return true;
    }

    return false;
}

void PadSettingsScreen::select_pad()
{
    pageFocus = -1;
    row_offset = 0;
    elements.clear();

    add_element(std::make_unique<UIText>("Press button"));
}

void PadSettingsScreen::pad_selected()
{
    if (pageFocus < 0 || pageFocus > 8) return;

    row_offset = 0;

    auto padSettings = SettingsManager::instance().get_component<PadsComponent>("pads");
    if (padSettings == nullptr) return;

    const auto config = padSettings->get_pad_config(pageFocus);

    elements.clear();

    add_element(std::make_unique<UIText>("PAD: " + std::to_string(pageFocus)));

    ui_intinput_config_t noteInput{
        .text = "Note",
        .customFormat = [](const int value)
        {
            return Utils::int_to_note(value);
        },
        .onChange = [](int value)
        {
            if (value < 0) value = 0;
            if (value > 127) value = 127;
            return value;
        },
        .onDone = [this, padSettings](const int value)
        {
            if (pageFocus < 0 || pageFocus > 8) return;

            padSettings->set_pad_note(pageFocus, value);
        }
    };
    add_element(std::make_unique<UIIntInput>(noteInput, config.note));

    ui_button_config_t saveBtn{
        .text = "Save",
        .callback = [this, padSettings]
        {
            padSettings->commit();
            on_back();
        }
    };
    add_element(std::make_unique<UIButton>(saveBtn));
    /*
    auto note = std::make_unique<UIIntInput>([this](const int value)
    {
        if (pageFocus < 0) return;

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
    */
}

/*
bool PadSettingsScreen::on_event(graphics_event_t event, uint16_t element)
{
    if (focus >= 0)
    {
//        if (event == EVENT_FOCUS)
//        {
//            ESP_LOGI("PadSettingsScreen", "Pad configured");
//            select_pad();
//            return false;
//        }
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
*/