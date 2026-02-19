#include <freertos/FreeRTOS.h>
#include "ads1015.hpp"
#include <driver/i2c_master.h>
#include <esp_log.h>
#include <string>
#include <bitset>

std::string byte_to_bin_string(uint8_t byte)
{
    std::string str;
    for (int i = 0; i < 8; ++i)
    {
        str += std::to_string((byte >> (7 - i)) & 1);
    }
    return str;
}

ads1015::ads1015(
    i2c_master_bus_handle_t bus_handle,
    uint16_t address
)
{
    i2c_device_config_t device_config = {};
    device_config.dev_addr_length = I2C_ADDR_BIT_LEN_7;
    device_config.device_address = address;
    device_config.scl_speed_hz = 400000;

    ESP_ERROR_CHECK(i2c_master_bus_add_device(bus_handle, &device_config, &dev_handle));
    ESP_LOGI("ADS1015", "Device configured");

    // Read config register
    esp_err_t r = set_config_reg();

    if (r == ESP_OK)
    {
        uint8_t bytes[2];
        r = i2c_master_receive(dev_handle, &bytes[0], sizeof(bytes), 1000/portTICK_PERIOD_MS);

        if (r == ESP_OK)
        {
            uint16_t val = bytes[0] << 8 | bytes[1];
            std::string str = std::bitset<16>(val).to_string();
            ESP_LOGI("ADS1015", "Config: %s", str.c_str());

            cfgReg = val;
        } else
        {
            ESP_LOGE("ADS1015", "Error receiving");
        }
    } else
    {
        ESP_LOGE("ADS1015", "Error transmitting");
    }
}

esp_err_t ads1015::set_config_reg() const
{
    constexpr uint8_t regNum = 0x1;
    return i2c_master_transmit(dev_handle, &regNum, 1, 1000/portTICK_PERIOD_MS);
}

esp_err_t ads1015::set_config(const ads1015_config_t* config)
{
    uint8_t bytes[3] = { 0x01, 0, 0 };


    bytes[1] = ((config->mux_config & 0b111) << 4) | ((config->fsr_mode & 0b111) << 1) | ((config->op_mode & 0b1) << 0);
    bytes[2] = ((config->data_rate & 0b111) << 5) | ((config->comparator_mode & 0b1) << 4) | ((config->comparator_polarity & 0b1) << 3) | ((config->comparator_latching & 0b1) << 2) | ((config->queue_and_disable & 0b11) << 0);

    const esp_err_t r = i2c_master_transmit(dev_handle, bytes, sizeof(bytes), 1000/portTICK_PERIOD_MS);

    if (r == ESP_OK)
    {
        ESP_LOGI("ADS1015", "Configuration updated!");
        cfgReg = bytes[1] << 8 | bytes[2];
    }

    return r;
}

esp_err_t ads1015::read_config()
{
    esp_err_t r = set_config_reg();
    if (r != ESP_OK) return r;

    uint8_t bytes[2];
    r = i2c_master_receive(dev_handle, &bytes[0], sizeof(bytes), 1000/portTICK_PERIOD_MS);

    if (r != ESP_OK) return r;

    cfgReg = bytes[0] << 8 | bytes[1];

    std::string str = std::bitset<16>(cfgReg).to_string();
    ESP_LOGI("ADS1015", "Read Config: %s", str.c_str());

    return r;
}

esp_err_t ads1015::set_mux(ads1015_mux_config_t mux)
{
    uint8_t bytes[3] = { 0x01, 0, 0 };

    constexpr uint8_t mask = ~0b01110000;

    bytes[1] = (cfgReg >> 8) & 0xFF;
    bytes[1] &= mask;
    bytes[1] |= (mux & 0b111) << 4;

    bytes[2] = cfgReg & 0xFF;

    const esp_err_t r = i2c_master_transmit(dev_handle, bytes, sizeof(bytes), 1000/portTICK_PERIOD_MS);

    if (r == ESP_OK)
    {
        cfgReg = bytes[1] << 8 | bytes[2];
    }

    return r;
}

uint16_t ads1015::read() const
{
    constexpr uint8_t reg_addr = 0x0;
    uint8_t data[2];

    i2c_master_transmit_receive(dev_handle, &reg_addr, 1, data, 2, 1000/portTICK_PERIOD_MS);

    const auto raw_val = static_cast<int16_t>(data[0] << 8 | data[1]);
    return raw_val >> 4;
}
