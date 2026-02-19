#ifndef SAMPLER_PADS_H
#define SAMPLER_PADS_H

#include <ads1015/ads1015.hpp>
#include "FreeRTOS.h"
#include <atomic>

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
    uint8_t channel;
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
    // settings
    uint8_t note;
    pad_press_type_t press_type;
    uint16_t threshold;

    // states
    uint16_t peak;
    uint32_t timer_start;
    pad_state_t state;
} drum_pad_t;

class PadsManager
{
public:
    static PadsManager& instance()
    {
        static PadsManager padsManager;
        return padsManager;
    }

    PadsManager(const PadsManager&) = delete;
    void operator=(const PadsManager&) = delete;

    void start_task();
    void pause_task() const;
    void resume_task() const;

    QueueHandle_t pads_midi_events;
    QueueHandle_t pads_input_events;

    std::atomic<bool> enable = true;

    drum_pad_t pads_settings[8]{};
    TaskHandle_t padsSTaskHandle = nullptr;
private:
    ads1015* ads1;
    ads1015* ads2;

    PadsManager();
};

#endif //SAMPLER_PADS_H