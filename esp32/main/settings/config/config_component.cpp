#include "config_component.h"

ConfigComponent::ConfigComponent(const QueueHandle_t updates): SettingsComponent("config")
{
    this->updates = updates;

    values["bpm"] = 120;
}

void ConfigComponent::on_load()
{
    if (!SettingsUtils::read_json(filename, values))
    {
        ESP_LOGI("ConfigComponent", "File doesn't exist, writing");
        SettingsUtils::save_json(filename, values);
    }

    xQueueSend(updates, &EVENT_UPDATE_BPM, 0);
}

int ConfigComponent::bpm() const
{
    return values["bpm"];
}

void ConfigComponent::set_bpm(const int bpm)
{
    std::lock_guard lock(mut);
    values["bpm"] = bpm;
}

void ConfigComponent::save()
{
    std::string output;
    {
        std::lock_guard lock(mut);
        ArduinoJson::serializeJsonPretty(values, output);
    }

    if (!SettingsUtils::save_json(filename, output))
    {
        ESP_LOGE("ConfigComponent", "Failed to save file");
        return;
    }

    xQueueSend(updates, &EVENT_UPDATE_BPM, 0);
}
