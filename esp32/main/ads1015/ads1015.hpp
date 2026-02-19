#ifndef SAMPLER_ADS1015_H
#define SAMPLER_ADS1015_H
#include "esp_err.h"
#include <driver/i2c_types.h>

typedef enum
{
    MUX_DIFF_P0N1 = 0x00,
    MUX_DIFF_P0N3 = 0x01,
    MUX_DIFF_P1N3 = 0x02,
    MUX_DIFF_P2N3 = 0x03,
    MUX_0 = 0x04,
    MUX_1 = 0x05,
    MUX_2 = 0x06,
    MUX_3 = 0x07,
} ads1015_mux_config_t;

typedef enum
{
    FSR_6_144 = 0x00,
    FSR_4_096 = 0x01,
    FSR_2_048 = 0x02,
    FSR_1_024 = 0x03,
    FSR_0_512 = 0x04,
    FSR_0_256 = 0x05,
} ads1015_fsr_config_t;

typedef enum
{
    OP_CONTINUOUS  = 0x0,
    OP_SINGLE_SHOT = 0x1
} ads1015_op_mode_config_t;

typedef enum
{
    DATA_RATE_128 = 0x0,
    DATA_RATE_250 = 0x1,
    DATA_RATE_490 = 0x2,
    DATA_RATE_920 = 0x3,
    DATA_RATE_1600 = 0x4,
    DATA_RATE_2400 = 0x5,
    DATA_RATE_3300 = 0x6,
} ads1015_data_rate_config_t;

typedef enum
{
    COMP_TRADITIONAL = 0x0,
    COMP_WINDOW = 0x1,
} ads1015_comp_mode_config_t;

typedef enum
{
    COMP_POLARITY_ACTIVE_LOW = 0x0,
    COMP_POLARITY_ACTIVE_HIGH = 0x1
} ads1015_comp_polarity_config_t;

typedef enum
{
    COMP_NON_LATCHING = 0x0,
    COMP_LATCHING = 0x1,
} ads1015_comp_latching_config_t;

typedef enum
{
    ASSERT_AFTER_ONE  = 0x0,
    ASSERT_AFTER_TWO  = 0x1,
    ASSERT_AFTER_FOUR = 0x2,
    DISABLE_COMP      = 0x3
} ads1015_comp_queue_disable_config_t;

typedef struct
{
    ads1015_mux_config_t mux_config;
    ads1015_fsr_config_t fsr_mode;
    ads1015_op_mode_config_t op_mode;
    ads1015_data_rate_config_t data_rate;
    ads1015_comp_mode_config_t comparator_mode;
    ads1015_comp_polarity_config_t comparator_polarity;
    ads1015_comp_latching_config_t comparator_latching;
    ads1015_comp_queue_disable_config_t queue_and_disable;
} ads1015_config_t;

class ads1015
{
    i2c_master_dev_handle_t dev_handle{};
    uint16_t cfgReg = 0;

    esp_err_t set_config_reg() const;
public:
    ads1015(i2c_master_bus_handle_t, uint16_t);

    esp_err_t set_config(const ads1015_config_t*);
    esp_err_t read_config();

    esp_err_t set_mux(ads1015_mux_config_t);

    uint16_t read() const;
};


#endif //SAMPLER_ADS1015_H