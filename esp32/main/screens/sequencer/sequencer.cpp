#include "sequencer.h"
#include <esp_log.h>
#include <graphics/ui/button/button.h>
#include <graphics/ui/checkbox/checkbox.h>
#include <graphics/ui/text/text.h>

#include "../screens.h"
#include <pads/pads.h>
#include <quantizer/quantizer.h>
#include <sequencer/sequencer.h>

screen_t create_sequencer_screen(GraphicsManager& graphics_manager)
{
    return std::make_unique<SequencerScreen>(graphics_manager);
}

SequencerScreen::SequencerScreen(GraphicsManager&) : Screen("sequencer")
{
    sequencer_page();
}

uint8_t pad_to_seq(const uint8_t pad_index)
{
    return (pad_index / 2) + (pad_index % 2) * 4;
}

void SequencerScreen::sequencer_page()
{
    row_offset = 0;
    pageFocus = -1;
    elements.clear();

    add_element(std::make_unique<UIText>("Sequencer"));

    ui_checkbox_config_t enableSequencer{
        .label = "Enable",
        .onChange = [](const bool checked)
        {
            Sequencer::instance().enable = checked;
            return checked;
        }
    };
    add_element(std::make_unique<UICheckBox>(enableSequencer, Sequencer::instance().enable));

    ui_button_config_t startBtn{
        .text = "Start",
        .callback = []
        {
            auto& quantizer = Quantizer::instance();
            quantizer.steps = 15;
            quantizer.ticks = 5;
            ESP_LOGI("Sequencer", "Restarting");
        }
    };
    add_element(std::make_unique<UIButton>(startBtn));

    ui_button_config_t tracksBtn{
        .text = "Tracks",
        .callback = [this]
        {
            show_tracks();
        }
    };
    add_element(std::make_unique<UIButton>(tracksBtn));
}

void SequencerScreen::show_tracks()
{
    row_offset = 0;
    pageFocus = 0;
    PadsManager::instance().is_enabled = true;
    elements.clear();

    elements.push_back(std::make_unique<UIText>("Tracks"));
    ui_button_config_t addTrackBtn{
        .text = "Add",
        .callback = [this]
        {
            Sequencer::instance().tracks.push_back({
                .loops = 1,
                .resolution = SEQ_RES_HALF_BEAT,
                .note = 60,
                .trigger = false,
                .triggers = {}
            });
            show_tracks();
        }
    };
    elements.push_back(std::make_unique<UIButton>(addTrackBtn));

    //ESP_LOGI("TRACKS", "%i", Sequencer::instance().tracks.size());
    for (size_t i = 0; i < Sequencer::instance().tracks.size(); ++i)
    {
        ui_button_config_t navTrackBtn{
            .text = std::to_string(i),
            .callback = [this, i]
            {
                edit_track(i);
            }
        };
        elements.push_back(std::make_unique<UIButton>(navTrackBtn));
    }
}

void SequencerScreen::edit_track(const uint16_t index)
{
    pageFocus = 1;
    row_offset = 0;
    PadsManager::instance().is_enabled = false;
    editingTrack = index;
    elements.clear();

    elements.push_back(std::make_unique<UIText>("Track " + std::to_string(editingTrack)));

    ui_button_config_t deleteBtn{
        .text = "Delete",
        .callback = [this]
        {
            Sequencer::instance().tracks.erase(Sequencer::instance().tracks.begin() + editingTrack);
            show_tracks();
        }
    };
    elements.push_back(std::make_unique<UIButton>(deleteBtn));
}

bool SequencerScreen::on_back()
{
    if (pageFocus >= 0)
    {
        if (Screen::on_back())
        {
            return true;
        }

        switch (pageFocus)
        {
        case 0:
            sequencer_page();
            break;
        case 1:
            show_tracks();
            break;
        default: break;
        }
        return true;
    }

    return Screen::on_back();
}

bool SequencerScreen::on_custom_event(uint32_t event)
{
    if (pageFocus == 1)
    {
        const uint8_t channel = event & 0b111;
        const bool long_press = (event & 0b1000) > 0;

        if (!long_press)
        {
            ESP_LOGI("SEQUENCER", "PRESSED %u", channel);
            auto& sequencer = Sequencer::instance();
            auto& triggers = sequencer.tracks[editingTrack].triggers;

            const auto trigger = pad_to_seq(channel);
            auto item = std::ranges::find(triggers, trigger);

            if (item != triggers.end())
            {
                ESP_LOGI("SEQUENCER", "REMOVING TRIGGER");
                triggers.erase(item);
            } else
            {
                ESP_LOGI("SEQUENCER", "ADDING TRIGGER");
                triggers.push_back(trigger);
            }
        }
        return true;
    }
    return false;
}

/*
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
*/
