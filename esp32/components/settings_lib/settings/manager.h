#ifndef SAMPLER_MANAGER_H
#define SAMPLER_MANAGER_H

#include <ArduinoJson.hpp>
#include <esp_log.h>
#include <unordered_map>
#include "component/component.h"

namespace SettingsUtils
{
    bool save_json(const char* path, const std::string& output);
    bool save_json(const char* path, const ArduinoJson::JsonDocument& document);
    bool read_file(const char* path, std::string& buffer);
    bool read_json(const char* path, ArduinoJson::JsonDocument& document);
}

class SettingsManager
{
public:
    static SettingsManager& instance()
    {
        static SettingsManager instance;
        return instance;
    }

    bool init();

    SettingsManager(const SettingsManager&) = delete;
    void operator=(const SettingsManager&) = delete;

    void add_component(component_t component);

    template <typename T>
    requires std::derived_from<T, SettingsComponent>
    T* get_component(const std::string& id);

private:
    QueueHandle_t saves_queue;
    bool mounted = false;

    std::unordered_map<std::string, component_t> components;

    SettingsManager();

    friend void save_task(void*);
};

template <typename T> requires std::derived_from<T, SettingsComponent>
T* SettingsManager::get_component(const std::string& id)
{
    if (!components.contains(id))
    {
        ESP_LOGE("SettingsManager", "Requested component (%s) does not exist", id.c_str());
        return nullptr;
    }
    return static_cast<T*>(components[id].get());
}

#endif //SAMPLER_MANAGER_H
