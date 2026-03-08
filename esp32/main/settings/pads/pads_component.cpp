#include "pads_component.h"

#include <esp_log.h>
#include <settings/manager.h>
#include <utils/utils.h>

using ArduinoJson::JsonArray, ArduinoJson::JsonObject;

void PadsComponent::pad_config_to_json(const pad_config_t& src, const ArduinoJson::JsonVariant dst)
{
    dst["note"] = src.note;
    dst["channel"] = src.channel;
    dst["press_type"] = static_cast<int>(src.press_type);
    dst["threshold"] = src.threshold;
}

bool PadsComponent::json_to_pad_config(const ArduinoJson::JsonVariant src, pad_config_t& dst)
{
    if (!src["note"].is<uint8_t>()) return false;
    if (!src["channel"].is<uint8_t>()) return false;
    if (!src["press_type"].is<int>()) return false;
    if (!src["threshold"].is<uint8_t>()) return false;

    dst.note = src["note"].as<uint8_t>();
    dst.channel = src["channel"].as<uint8_t>();
    dst.press_type = static_cast<pad_press_type_t>(src["press_type"].as<int>());
    dst.threshold = src["threshold"].as<uint16_t>();

    return true;
}

void PadsComponent::load_defaults()
{
    values.clear();
    values["version"] = VERSION;
    const auto array = values["pads"].to<JsonArray>();

    uint8_t note = 60;
    for (auto& config : configs)
    {
        config.note = note++;
        config.channel = 0;
        config.press_type = ONE_SHOT;
        config.threshold = 50;

        auto obj = array.add<JsonObject>();
        pad_config_to_json(config, obj);
    }
}

void PadsComponent::json_to_configs()
{
    const auto array = values["pads"].as<JsonArray>();
    if (array.isNull())
    {
        ESP_LOGE("PadsComponent", "Array is null");
        return;
    }
    const size_t size = array.size();
    if (size != 8)
    {
        ESP_LOGE("PadsComponent", "Invalid array size (%i)", size);
        return;
    }

    for (size_t i = 0; i < size; i++)
    {
        if (!json_to_pad_config(array[i], configs[i]))
        {
            ESP_LOGE("PadsComponent", "Failed parsing config %i", i);
            continue;
        }

        ESP_LOGI("PadsComponent", "Config: %u %u %u", configs[i].note, configs[i].press_type, configs[i].threshold);
    }
}

PadsComponent::PadsComponent() : SettingsComponent("pads")
{
    load_defaults();
}

pad_config_t PadsComponent::get_pad_config(const uint8_t index) const
{
    return configs[index];
}

void PadsComponent::set_pad_note(const uint8_t index, const uint8_t note)
{
    if (note > Utils::MAX_MIDI_NOTE) return;

    std::lock_guard lock(mut);
    const auto array = values["pads"].as<JsonArray>();
    array[index]["note"] = note;
    //configs[index].note = note;
}

void PadsComponent::set_pad_channel(const uint8_t index, const uint8_t channel)
{
    if (channel > Utils::MAX_MIDI_CHANNELS) return;

    std::lock_guard lock(mut);
    const auto array = values["pads"].as<JsonArray>();
    array[index]["channel"] = channel;
}

void PadsComponent::on_load()
{
    std::lock_guard lock(mut);
    if (!SettingsUtils::read_json(filename, values))
    {
        ESP_LOGI("PadsComponent", "File doesn't exist, writing");
        SettingsUtils::save_json(filename, values);
    }

    if (!values["version"].is<uint32_t>() || values["version"].as<uint32_t>() != VERSION)
    {
        load_defaults();
        SettingsUtils::save_json(filename, values);
    }

    json_to_configs();
}

void PadsComponent::save()
{
    std::lock_guard lock(mut);
    if (!SettingsUtils::save_json(filename, values))
    {
        ESP_LOGE("PadsComponent", "Failed to save file");
        return;
    }

    json_to_configs();

    auto& padsManager = PadsManager::instance();

    padsManager.pause_task();

    for (size_t i = 0; i < 8; i++)
    {
        auto& config = padsManager.pads_settings[i];
        const auto& settings = configs[i];

        config.note = settings.note;
        config.channel = settings.channel;
        config.press_type = settings.press_type;
        config.threshold = settings.threshold;
    }

    padsManager.resume_task();
    // TODO
}

std::string PadsComponent::direct_read(const std::string& arg)
{
    std::string buffer;
    SettingsUtils::read_file(filename, buffer);
    return buffer;
}
