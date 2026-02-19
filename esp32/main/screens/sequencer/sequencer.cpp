#include "sequencer.h"

#include <esp_log.h>
#include <graphics_test/ui/button/button.h>

#include "../screens.h"
#include <graphics_test/ui/checkbox/checkbox.h>
#include <graphics_test/ui/text/text.h>
#include <pads/pads.h>
#include <quantizer/quantizer.h>
#include <sequencer/sequencer.h>

void create_sequencer_screen(lcd& lcd)
{
    lcd.load_screen(std::make_unique<SequencerScreen>());
}

SequencerScreen::SequencerScreen() : Screen("sequencer")
{
    on_start();
}

void SequencerScreen::on_start()
{
    focus = -1;
    elements.clear();

    elements.push_back(std::make_unique<UIText>("Sequencer"));
    auto enableSequencer = std::make_unique<UICheckBox>([](const bool checked) {
        Sequencer::instance().enable = checked;
    });
    enableSequencer->checked = Sequencer::instance().enable;
    enableSequencer->label = "Enable";
    elements.push_back(std::move(enableSequencer));

    elements.push_back(std::make_unique<UIButton>("Start", []{
        auto& quantizer = Quantizer::instance();
        quantizer.steps = 15;
        quantizer.ticks = 5;
        ESP_LOGI("Sequencer", "Restarting");
    }));

    elements.push_back(std::make_unique<UIButton>("Tracks", [this]
    {
        show_tracks();
    }));
}

void SequencerScreen::show_tracks()
{
    PadsManager::instance().enable = true;
    focus = 0;
    elements.clear();

    elements.push_back(std::make_unique<UIText>("Tracks"));
    elements.push_back(std::make_unique<UIButton>("Add", [this]
    {
        Sequencer::instance().tracks.push_back({
            .loops = 1,
            .resolution = SEQ_RES_HALF_BEAT,
            .note = 60,
            .trigger = false,
            .triggers = {}
        });
        show_tracks();
    }));

    //ESP_LOGI("TRACKS", "%i", Sequencer::instance().tracks.size());
    for (size_t i = 0; i < Sequencer::instance().tracks.size(); ++i)
    {
        elements.push_back(std::make_unique<UIButton>(std::to_string(i), [this, i]
        {
            edit_track(i);
        }));
    }
}

void SequencerScreen::edit_track(uint16_t index)
{
    PadsManager::instance().enable = false;
    focus = 1;
    editingTrack = index;
    elements.clear();

    elements.push_back(std::make_unique<UIText>("Track " + std::to_string(editingTrack)));
    elements.push_back(std::make_unique<UIButton>("Delete", [this]
    {
        Sequencer::instance().tracks.erase(Sequencer::instance().tracks.begin() + editingTrack);
        show_tracks();
    }));
}

bool SequencerScreen::on_event(const graphics_event_t event, const uint16_t element)
{
    if (event == EVENT_BACK && focus >= 0)
    {
        switch (focus)
        {
        case 0:
            on_start();
            break;
        case 1:
            show_tracks();
            break;
        default: break;
        }
        return true;
    }
    return Screen::on_event(event, element);
}

bool SequencerScreen::on_custom_event(uint32_t event)
{
    if (focus == 1)
    {
        const uint8_t channel = event & 0b111;
        const bool long_press = (event & 0b1000) > 0;

        if (!long_press)
        {
            ESP_LOGI("SEQUENCER", "PRESSED %u", channel);
            auto& sequencer = Sequencer::instance();
            auto& triggers = sequencer.tracks[editingTrack].triggers;
            auto item = std::ranges::find(triggers, channel);
            if (item != triggers.end())
            {
                ESP_LOGI("SEQUENCER", "REMOVING TRIGGER");
                triggers.erase(item);
            } else
            {
                ESP_LOGI("SEQUENCER", "ADDING TRIGGER");
                triggers.push_back(channel);
            }
        }
        return true;
    }
    return false;
}
