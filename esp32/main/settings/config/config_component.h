#ifndef SAMPLER_CONFIG_COMPONENT_H
#define SAMPLER_CONFIG_COMPONENT_H

#include <settings/manager.h>

constexpr uint32_t EVENT_UPDATE_BPM = 1;

class ConfigComponent : public SettingsComponent
{
    ArduinoJson::JsonDocument values;
    std::mutex mut;

    static constexpr auto filename = "/data/config.json";
public:
    ConfigComponent();

    void on_load() override;

    [[nodiscard]] int bpm() const;
    void set_bpm(int bpm);

    void save() override;

    std::string direct_read(const std::string& arg) override;
    bool direct_write(const std::string& buffer, const std::string& arg) override;
};


#endif //SAMPLER_CONFIG_COMPONENT_H