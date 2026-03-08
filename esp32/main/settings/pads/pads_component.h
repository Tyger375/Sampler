#ifndef SAMPLER_PADS_COMPONENT_H
#define SAMPLER_PADS_COMPONENT_H

#include <settings/component/component.h>
#include <ArduinoJson.hpp>
#include <pads/pads.h>

typedef struct
{
    uint8_t note;
    uint8_t channel;
    pad_press_type_t press_type;
    uint16_t threshold;
} pad_config_t;

class PadsComponent : public SettingsComponent
{
    ArduinoJson::JsonDocument values;
    pad_config_t configs[8]{};

    std::mutex mut;

    /*
     * Update VERSION every time JSON structure changes
     * This *DELETES* the old file and creates a new one with default values
     */
    static constexpr uint32_t VERSION = 1;
    static constexpr auto filename = "/data/pads.json";

    static void pad_config_to_json(const pad_config_t& src, ArduinoJson::JsonVariant dst);
    static bool json_to_pad_config(ArduinoJson::JsonVariant src, pad_config_t& dst);

    void load_defaults();
    void json_to_configs();
public:
    PadsComponent();

    [[nodiscard]] pad_config_t get_pad_config(uint8_t index) const;

    void set_pad_note(uint8_t index, uint8_t note);
    void set_pad_channel(uint8_t index, uint8_t channel);

    void on_load() override;
    void save() override;

    std::string direct_read(const std::string& arg) override;
};


#endif //SAMPLER_PADS_COMPONENT_H