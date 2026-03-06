#include "manager.h"
#include <esp_log.h>

using ArduinoJson::JsonDocument, ArduinoJson::serializeJsonPretty;

bool SettingsUtils::save_json(const char* path, const std::string& output)
{
    FILE* file = fopen(path, "w");
    if (file == nullptr) return false;

    const size_t written = fwrite(output.c_str(), sizeof(char), output.length(), file);
    fclose(file);

    if (written != output.length())
    {
        ESP_LOGE("SettingsManager", "Failed to write to file (sizes are not matching)");
        return false;
    }

    return true;
}

bool SettingsUtils::save_json(const char* path, const JsonDocument& document)
{
    std::string output;
    serializeJsonPretty(document, output);

    FILE* file = fopen(path, "w");
    if (file == nullptr) return false;

    const size_t written = fwrite(output.c_str(), sizeof(char), output.length(), file);
    fclose(file);

    if (written != output.length())
    {
        ESP_LOGE("SettingsManager", "Failed to write to file (sizes are not matching)");
        return false;
    }

    return true;
}

bool SettingsUtils::read_file(const char* path, std::string& buffer)
{
    FILE* file = fopen(path, "r");
    if (file == nullptr) return false;

    fseek(file, 0, SEEK_END);
    const size_t size = ftell(file);
    fseek(file, 0, SEEK_SET);

    buffer.resize(size);

    const size_t read_bytes = fread(&buffer[0], sizeof(char), size, file);
    fclose(file);

    if (read_bytes != size)
        return false;

    return true;
}

bool SettingsUtils::read_json(const char* path, JsonDocument& document)
{
    std::string buffer;
    if (!read_file(path, buffer))
    {
        ESP_LOGE("SettingsManager", "Failed to read from file");
        return false;
    }
    if (deserializeJson(document, buffer))
    {
        ESP_LOGE("SettingsManager", "Failed to deserialize json");
        return false;
    }
    return true;
}