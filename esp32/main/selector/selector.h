#ifndef SAMPLER_SELECTOR_H
#define SAMPLER_SELECTOR_H

#include "FreeRTOS.h"
#include "queue.h"
#include "driver/gpio.h"

typedef struct
{
    gpio_num_t clk_gpio;
    gpio_num_t data_gpio;
    gpio_num_t btn_gpio;
    QueueHandle_t events;
} selector_config_t;

enum selector_event_t
{
    ROTATION_RIGHT,
    ROTATION_LEFT,
    BUTTON_PRESSED,
    BUTTON_RELEASED,
};

class Selector
{
public:
    selector_config_t* config = nullptr;
    uint64_t last_isr{};
    uint64_t last_isr_btn{};

    explicit Selector(selector_config_t*);
};


#endif //SAMPLER_SELECTOR_H