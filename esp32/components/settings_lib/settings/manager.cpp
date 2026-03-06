#include "manager.h"
#include <esp_log.h>
#include <esp_littlefs.h>

SettingsManager::SettingsManager()
{
    saves_queue = xQueueCreate(10, sizeof(SettingsComponent*));
}

void save_task(void* pvParameters)
{
    auto* settings = static_cast<SettingsManager*>(pvParameters);

    //saves_queue_item_t item;
    SettingsComponent* item = nullptr;
    //uint32_t dirty_mask = 0;

    while (true)
    {
        if (xQueueReceive(settings->saves_queue, &item, portMAX_DELAY))
        {
            //dirty_mask |= (1 << item.type);

            item->save();
            // Debounce (keep collecting flags for 1 second)
            /*
            while (xQueueReceive(settings->saves_queue, &item, pdMS_TO_TICKS(1000))) {
                dirty_mask |= (1 << item.type);
            }

            // Check each bit and save
            if (dirty_mask & (1 << SAVE_CONFIG)) {
                if (settings->write_config())
                {
                    xQueueSend(settings->updates_queue, &item.event, 10);
                } else
                {
                    ESP_LOGE("SaveTask", "Failed to write configs.json");
                }
            }

            dirty_mask = 0;*/
        }
    }
}

bool SettingsManager::init()
{
    ESP_LOGI("SettingsManager", "Mounting LittleFS...");

    esp_vfs_littlefs_conf_t conf{};
    conf.base_path = "/data";
    conf.partition_label = "storage";
    conf.format_if_mount_failed = true;
    conf.dont_mount = false;

    esp_err_t ret = esp_vfs_littlefs_register(&conf);

    if (ret != ESP_OK)
    {
        if (ret == ESP_FAIL)
        {
            ESP_LOGE("SettingsManager", "Failed to mount or format filesystem");
        } else if (ret == ESP_ERR_NOT_FOUND)
        {
            ESP_LOGE("SettingsManager", "Partition not found in partition table");
        }
        return false;
    }

    size_t total = 0;
    size_t used = 0;

    ret = esp_littlefs_info(conf.partition_label, &total, &used);
    if (ret != ESP_OK)
        return false;

    mounted = true;
    ESP_LOGI("SettingsManager", "Filesystem mounted. Size: %d KB, Used: %d KB", total / 1024, used / 1024);

    xTaskCreate(
        save_task,
        "settings_manager_save",
        4096,
        this,
        4,
        nullptr
    );

    return true;
}

void SettingsManager::add_component(component_t component)
{
    const auto& id = component->id;
    if (components.contains(id))
    {
        ESP_LOGE("SettingsManager", "Group with id %s already exists", id.c_str());
        return;
    }
    component->init(saves_queue);
    components[id] = std::move(component);
}
