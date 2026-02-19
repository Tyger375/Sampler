#ifndef SAMPLER_SETTINGS_MANAGER_H
#define SAMPLER_SETTINGS_MANAGER_H

#include "FreeRTOS.h"
#include <atomic>
#include "nvs.h"
#include "queue.h"

enum settings_update_t
{
    UPDATE_BPM
};

class SettingsManager
{
public:
    static constexpr int DEFAULT_BPM = 120;

    static SettingsManager& instance()
    {
        static SettingsManager instance;
        return instance;
    }

    SettingsManager(const SettingsManager&) = delete;
    void operator=(const SettingsManager&) = delete;

    QueueHandle_t updates_queue;
    std::atomic<int> bpm{};

    static bool save_int(const char*, int);

    bool save_bpm(int);
    bool load();
private:
    SettingsManager()
    {
        updates_queue = xQueueCreate(10, sizeof(settings_update_t));
    }

    bool load_or_save_bpm(nvs_handle_t);
};


#endif //SAMPLER_SETTINGS_MANAGER_H