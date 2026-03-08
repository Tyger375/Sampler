#include "pads.h"

#include <settings/manager.h>
#include <settings/pads/pads_component.h>

#include "esp_log.h"
#include "esp_log_timestamp.h"
#include "ads1015/ads1015.hpp"
#include "driver/i2c_master.h"

struct input_scan_packet
{
    ads1015* ads1;
    ads1015* ads2;
};

ads1015_mux_config_t channel_to_mux_config(const uint8_t channel)
{
    switch (channel)
    {
        case 0: return MUX_0;
        case 1: return MUX_1;
        case 2: return MUX_2;
        case 3: return MUX_3;
        default: return MUX_DIFF_P0N1;
    }
}

void process_pad_physics(PadsManager& padsManager, const uint8_t channel, const uint16_t value)
{
    drum_pad_t* pad = &padsManager.pads_settings[channel];
    const uint32_t now = esp_log_timestamp();

    switch (pad->state)
    {
    case PAD_IDLE:
        {
            if (value > pad->threshold)
            {
                pad->state = PAD_ATTACK;
                pad->peak = value;
                pad->timer_start = now;
            }
        } break;
    case PAD_ATTACK:
        {
            // Update peak if sample is higher
            if (value > pad->peak)
                pad->peak = value;

            // Wait 5ms
            if (now - pad->timer_start >= 5)
            {
                const uint8_t velocity = (pad->peak > 2047) ? 127 : (pad->peak >> 4);

                const pad_midi_event_t midi_event =
                {
                    .channel = channel,
                    .note = pad->note,
                    .velocity = velocity,
                    .type = NOTE_ON
                };

                xQueueSend(padsManager.pads_midi_events, &midi_event, 0);

                const pad_input_event_t input_event = {
                    .channel = channel,
                    .pressed = true
                };
                xQueueSend(padsManager.pads_input_events, &input_event, 0);

                pad->state = PAD_SUSTAIN;
            }
        } break;
    case PAD_SUSTAIN:
        {
            // Look for the value to drop below threshold
            if (value < (pad->threshold * 0.8)) // 20% Hysteresis
            {
                pad->state = PAD_RELEASE;
                pad->timer_start = now;
            }
        } break;
    case PAD_RELEASE:
        {
            const pad_midi_event_t midi_event =
            {
                .channel = channel,
                .note = pad->note,
                .velocity = 0,
                .type = NOTE_OFF
            };

            xQueueSend(padsManager.pads_midi_events, &midi_event, 0);

            const pad_input_event_t input_event = {
                .channel = channel,
                .pressed = false
            };
            xQueueSend(padsManager.pads_input_events, &input_event, 0);

            pad->state = PAD_IDLE;
        } break;
    }
}

void input_scan_task(void* pvParameters)
{
    auto& padsManager = PadsManager::instance();
    const auto* packet = static_cast<struct input_scan_packet*>(pvParameters);
    uint8_t channel = 0;

    while (true)
    {
        if (padsManager.is_task_paused) {
            vTaskDelay(pdMS_TO_TICKS(100)); // Sleep efficiently
            continue; // Skip the ADC logic
        }

        esp_rom_delay_us(500);

        const uint16_t val1 = packet->ads1->read();
        const uint16_t val2 = packet->ads2->read();

        process_pad_physics(padsManager, channel, val1);
        process_pad_physics(padsManager, channel + 4, val2);

        channel = (channel + 1) % 4;
        ESP_ERROR_CHECK(packet->ads1->set_mux(channel_to_mux_config(channel)));
        ESP_ERROR_CHECK(packet->ads2->set_mux(channel_to_mux_config(channel)));
    }
}

PadsManager::PadsManager()
{
    pads_midi_events = xQueueCreate(64, sizeof(pad_midi_event_t));
    pads_input_events = xQueueCreate(64, sizeof(pad_input_event_t));

    //padsSTaskHandle =

    const auto padsComponent = SettingsManager::instance().get_component<PadsComponent>("pads");

    for (size_t i = 0; i < 8; i++)
    {
        auto& pad = pads_settings[i];
        const auto config = padsComponent->get_pad_config(i);

        pad.threshold = config.threshold;
        pad.press_type = config.press_type;
        pad.note = config.note;

        pad.state = PAD_IDLE;
        pad.peak = 0;
        pad.timer_start = 0;
    }
}

void PadsManager::init_adc(const pads_manager_config_t& config)
{
    i2c_master_bus_config_t bus_config = {};
    bus_config.i2c_port = config.port_num;
    bus_config.sda_io_num = config.sda_num;
    bus_config.scl_io_num = config.scl_num;
    bus_config.clk_source = I2C_CLK_SRC_DEFAULT;
    bus_config.glitch_ignore_cnt = 7;
    bus_config.flags.enable_internal_pullup = false;

    static i2c_master_bus_handle_t pads_bus_handle = {};

    ESP_ERROR_CHECK(i2c_new_master_bus(&bus_config, &pads_bus_handle));

    ads1 = new ads1015(pads_bus_handle, config.adc1_addr);
    ads2 = new ads1015(pads_bus_handle, config.adc2_addr);

    constexpr ads1015_config_t ads_cfg = {
        .mux_config = MUX_0,
        .fsr_mode = FSR_6_144,
        .op_mode = OP_CONTINUOUS,
        .data_rate = DATA_RATE_3300,
        .comparator_mode = COMP_TRADITIONAL,
        .comparator_polarity = COMP_POLARITY_ACTIVE_LOW,
        .comparator_latching = COMP_NON_LATCHING,
        .queue_and_disable = DISABLE_COMP
    };
    ESP_ERROR_CHECK(ads1->set_config(&ads_cfg));
    ESP_ERROR_CHECK(ads2->set_config(&ads_cfg));

    ads1->read_config();
    ads2->read_config();
}

void PadsManager::start_task()
{
    if (ads1 == nullptr || ads2 == nullptr)
    {
        ESP_LOGE("PadsManager", "Manager was not started");
        return;
    }

    static input_scan_packet packet{
        .ads1 = ads1,
        .ads2 = ads2
    };

    xTaskCreatePinnedToCore(
        input_scan_task,
        "pads_input_task",
        4096,
        &packet,
        15,
        &padsSTaskHandle,
        0
    );
}

void PadsManager::pause_task()
{
    is_task_paused = true;
}

void PadsManager::resume_task()
{
    is_task_paused = false;
}
