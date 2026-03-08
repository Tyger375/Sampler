#ifndef SAMPLER_PADS_H
#define SAMPLER_PADS_H

#include <ads1015/ads1015.hpp>
#include "FreeRTOS.h"
#include <atomic>
#include <soc/gpio_num.h>

typedef enum : uint8_t
{
    NOTE_ON,
    NOTE_OFF,
} midi_type_t;

typedef enum : uint8_t
{
    ONE_SHOT = 0
} pad_press_type_t;

typedef struct
{
    uint8_t pad_index;
    bool pressed;
} pad_input_event_t;

typedef struct
{
    uint8_t channel;
    uint8_t note;
    uint8_t velocity;
    midi_type_t type;
} pad_midi_event_t;

typedef enum : uint8_t
{
    PAD_IDLE,
    PAD_ATTACK,
    PAD_SUSTAIN,
    PAD_RELEASE
} pad_state_t;

typedef struct
{
    uint8_t note;                   // MIDI setting
    uint8_t channel;                // MIDI setting
    pad_press_type_t press_type;    // Internal setting
    pad_state_t state;              // state

    uint16_t threshold;             // Internal setting
    uint16_t peak;                  // state

    uint32_t timer_start;           // state
} drum_pad_t;

typedef struct
{
    i2c_port_num_t port_num;
    gpio_num_t sda_num;
    gpio_num_t scl_num;

    uint16_t adc1_addr;
    uint16_t adc2_addr;
} pads_manager_config_t;

class PadsManager
{
public:
    static PadsManager& instance()
    {
        static PadsManager padsManager;
        return padsManager;
    }

    void init_adc(const pads_manager_config_t& config);

    PadsManager(const PadsManager&) = delete;
    void operator=(const PadsManager&) = delete;

    void start_task();
    void pause_task();
    void resume_task();

    QueueHandle_t pads_midi_events;
    QueueHandle_t pads_input_events;

    std::atomic<bool> is_task_paused = false;
    std::atomic<bool> is_enabled = true;

    drum_pad_t pads_settings[8]{};
    TaskHandle_t padsSTaskHandle = nullptr;
private:
    ads1015* ads1 = nullptr;
    ads1015* ads2 = nullptr;

    PadsManager();
};

#endif //SAMPLER_PADS_H