#include "logger.h"
#include <esp_log.h>

void logger_driver::init() {}
void logger_driver::draw(std::vector<std::string> rows)
{
    ESP_LOGI("LOGGER_DRIVER", "---------------------");
    for (const auto& row : rows)
    {
        ESP_LOGI("LOGGER_DRIVER", "%s", row.c_str());
    }
    ESP_LOGI("LOGGER_DRIVER", "---------------------");
}

void logger_driver::clear() {}
