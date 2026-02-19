#include "selector.h"
#include "esp_log.h"
#include <string>
#include "esp_timer.h"

static void IRAM_ATTR selector_clk_isr_handler(void* arg)
{
    auto* self = static_cast<Selector*>(arg);

    const uint64_t now = esp_timer_get_time();
    if (now - self->last_isr < 50000) return;

    const int clk = gpio_get_level(self->config->clk_gpio);
    const int data = gpio_get_level(self->config->data_gpio);

    const selector_event_t event = clk == data ? ROTATION_LEFT : ROTATION_RIGHT;
    BaseType_t high_priority_task_woken = pdFALSE;
    if (xQueueSendFromISR(*self->config->events, &event, &high_priority_task_woken))
    {
        self->last_isr = now;
    }

    if (high_priority_task_woken) {
        portYIELD_FROM_ISR();
    }
}

static void IRAM_ATTR selector_btn_isr_handler(void* arg)
{
    auto* self = static_cast<Selector*>(arg);

    const uint64_t now = esp_timer_get_time();
    if (now - self->last_isr_btn < 50000) return;

    const int btn = gpio_get_level(self->config->btn_gpio);

    const selector_event_t event = btn == 0 ? BUTTON_PRESSED : BUTTON_RELEASED;
    BaseType_t high_priority_task_woken = pdFALSE;
    if (xQueueSendFromISR(*self->config->events, &event, &high_priority_task_woken))
    {
        self->last_isr_btn = now;
    }

    if (high_priority_task_woken) {
        portYIELD_FROM_ISR();
    }
}

Selector::Selector(selector_config_t* cfg)
{
    config = cfg;

    ESP_ERROR_CHECK(gpio_install_isr_service(ESP_INTR_FLAG_LOWMED));

    gpio_config_t clk_cfg;
    clk_cfg.mode = GPIO_MODE_INPUT;
    clk_cfg.pin_bit_mask = 1ULL << config->clk_gpio;
    clk_cfg.intr_type = GPIO_INTR_NEGEDGE;
    ESP_ERROR_CHECK(gpio_config(&clk_cfg));
    ESP_ERROR_CHECK(gpio_isr_handler_add(config->clk_gpio, selector_clk_isr_handler, this));

    gpio_config_t data_cfg;
    data_cfg.mode = GPIO_MODE_INPUT;
    data_cfg.pin_bit_mask = 1ULL << config->data_gpio;
    data_cfg.intr_type = GPIO_INTR_DISABLE;
    ESP_ERROR_CHECK(gpio_config(&data_cfg));

    gpio_config_t btn_cfg;
    btn_cfg.mode = GPIO_MODE_INPUT;
    btn_cfg.pin_bit_mask = 1ULL << config->btn_gpio;
    btn_cfg.intr_type = GPIO_INTR_ANYEDGE;
    ESP_ERROR_CHECK(gpio_config(&btn_cfg));
    ESP_ERROR_CHECK(gpio_isr_handler_add(config->btn_gpio, selector_btn_isr_handler, this));

    ESP_LOGI("Selector", "Created");
}
