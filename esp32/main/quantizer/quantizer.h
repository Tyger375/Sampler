#ifndef SAMPLER_QUANTIZER_H
#define SAMPLER_QUANTIZER_H

#include "FreeRTOS.h"
#include "task.h"
#include "driver/gptimer.h"
#include <atomic>

constexpr int PPQ = 24;
constexpr int TICKS_PER_STEP = PPQ / 4;
// 1 step = 1 quarter note
// 4 quarter notes = 1 beat
// 4 beats = 1 loop

class Quantizer
{
public:
    static Quantizer& instance()
    {
        static Quantizer instance;
        return instance;
    }

    Quantizer(const Quantizer&) = delete;
    void operator=(const Quantizer&) = delete;

    TaskHandle_t task_handle = nullptr;
    std::atomic<uint8_t> ticks{};
    std::atomic<uint8_t> steps{};

    void start(int);
private:
    Quantizer();

    bool started = false;
    gptimer_handle_t gptimer = nullptr;
};


#endif //SAMPLER_QUANTIZER_H