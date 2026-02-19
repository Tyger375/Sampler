#include "settings_manager.h"

#include "esp_log.h"

bool SettingsManager::save_int(const char* key, int value)
{
    nvs_handle_t handle;
    if (nvs_open("storage", NVS_READWRITE, &handle) == ESP_OK)
    {
        nvs_set_i32(handle, key, value);
        nvs_commit(handle);
        nvs_close(handle);
        return true;
    }
    return false;
}

bool SettingsManager::save_bpm(int value)
{
    if (save_int("bpm", value))
    {
        bpm = value;
        constexpr settings_update_t update = UPDATE_BPM;
        xQueueSend(updates_queue, &update, 10);
        return true;
    }
    return false;
}

bool SettingsManager::load()
{
    nvs_handle_t handle;
    if (nvs_open("storage", NVS_READWRITE, &handle) == ESP_OK)
    {
        settings_update_t update;
        if (!load_or_save_bpm(handle)) return false;
        update = UPDATE_BPM;
        xQueueSend(updates_queue, &update, 10);



        nvs_close(handle);
        return true;
    }

    return false;
}

bool SettingsManager::load_or_save_bpm(const nvs_handle_t handle)
{
    int32_t val;
    if (nvs_get_i32(handle, "bpm", &val) != ESP_OK)
    {
        if (!save_bpm(DEFAULT_BPM))
        {
            ESP_LOGE("SettingsManager", "Failed to load BPM");
            return false;
        }
    }
    else
    {
        bpm = val;
    }

    return true;
}
