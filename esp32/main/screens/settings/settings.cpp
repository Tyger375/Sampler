#include "settings.h"

#include <graphics/ui/button/button.h>
#include <graphics/ui/intinput/intinput.h>
#include <graphics/ui/text/text.h>
#include <settings/manager.h>
#include <settings/config/config_component.h>
#include "../screens.h"
#include "esp_log.h"
#include "esp_heap_caps.h"

void log_memory() {
    size_t total = heap_caps_get_total_size(MALLOC_CAP_8BIT);
    size_t free_heap = heap_caps_get_free_size(MALLOC_CAP_8BIT);
    size_t min_free = heap_caps_get_minimum_free_size(MALLOC_CAP_8BIT);

    size_t used = total - free_heap;
    float percentage = (static_cast<float>(used) / static_cast<float>(total)) * 100.f;

    ESP_LOGI("MEM", "Free Heap: %u bytes", free_heap);
    ESP_LOGI("MEM", "Usage: %.2f%% (%u / %u bytes)", percentage, used, total);
    ESP_LOGI("MEM", "Lowest Heap reached since boot: %u bytes", min_free);
}

screen_t create_settings_screen(GraphicsManager& graphics_manager)
{
    return std::make_unique<SettingsScreen>(graphics_manager);
}

SettingsScreen::SettingsScreen(GraphicsManager& graphics_manager) : Screen("settings")
{
    auto& settings = SettingsManager::instance();
    const auto component = settings.get_component<ConfigComponent>("config");
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
            const auto config_component = SettingsManager::instance().get_component<ConfigComponent>("config");
            if (config_component == nullptr)
            {
                ESP_LOGE("Sampler", "Component not found");
                return;
            }
            config_component->set_bpm(value);
            config_component->commit();
        },
    };
    add_element(std::make_unique<UIIntInput>(bpmSettings, component->bpm()));

    ui_button_config_t btnPadSettings{
        .text = "Pad Settings",
        .callback = [&graphics_manager]
        {
            graphics_manager.navigate("pad_settings");
        }
    };
    add_element(std::make_unique<UIButton>(btnPadSettings));

    ui_button_config_t debugSettingsManager{
        .text = "Debug",
        .callback = []
        {
            log_memory();
            /*
            SettingsManager::instance().debug_list();
            SettingsManager::instance().debug_config();
            */
        }
    };
    add_element(std::make_unique<UIButton>(debugSettingsManager));
}
