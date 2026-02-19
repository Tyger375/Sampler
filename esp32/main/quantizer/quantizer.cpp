#include "quantizer.h"
#include "driver/gptimer.h"

static bool IRAM_ATTR quantizer_timer_callback(
    gptimer_handle_t timer,
    const gptimer_alarm_event_data_t* event_data,
    void* arg
)
{
    auto* quantizer = static_cast<Quantizer*>(arg);

    if (++quantizer->ticks >= TICKS_PER_STEP) {
        quantizer->ticks = 0;
        quantizer->steps = (quantizer->steps + 1) % 16; // Loop 16 steps
    }

    BaseType_t xHigherPriorityTaskWoken = pdFALSE;
    if (quantizer->task_handle != nullptr)
    {
        vTaskNotifyGiveFromISR(quantizer->task_handle, &xHigherPriorityTaskWoken);
    }
    return xHigherPriorityTaskWoken == pdTRUE;
}

Quantizer::Quantizer()
{
    gptimer_config_t timer_config = {};
    timer_config.clk_src = GPTIMER_CLK_SRC_DEFAULT;
    timer_config.direction = GPTIMER_COUNT_UP;
    timer_config.resolution_hz = 1000000; // 1 tick = 1 microsecond

    ESP_ERROR_CHECK(gptimer_new_timer(&timer_config, &gptimer));
}

void Quantizer::start(const int bpm)
{
    const uint64_t timer_step = (60ULL * 1000000ULL) / (bpm * PPQ);

    gptimer_alarm_config_t alarm_config = {};
    alarm_config.reload_count = 0;
    alarm_config.alarm_count = timer_step;
    alarm_config.flags.auto_reload_on_alarm = true;

    ESP_ERROR_CHECK(gptimer_set_alarm_action(gptimer, &alarm_config));

    ticks = 0;
    steps = 0;

    if (!started)
    {
        gptimer_event_callbacks_t callbacks = {};
        callbacks.on_alarm = quantizer_timer_callback;

        ESP_ERROR_CHECK(gptimer_register_event_callbacks(gptimer, &callbacks, this));
        ESP_ERROR_CHECK(gptimer_enable(gptimer));
        ESP_ERROR_CHECK(gptimer_start(gptimer));

        started = true;
    }
}
