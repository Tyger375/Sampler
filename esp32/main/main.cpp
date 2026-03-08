#include "freertos/FreeRTOS.h"
#include <driver/i2c_master.h>
#include <graphics/drivers/lcd1602/lcd1602.h>
#include <graphics/drivers/logger/logger.h>
#include <sequencer/sequencer.h>
#include "esp_log.h"
#include "nvs_flash.h"
#include "tinyusb.h"
#include "pads/pads.h"
#include "usb/usb.h"
#include "quantizer/quantizer.h"
#include "selector/selector.h"
#include "screens/screens.h"
#include <graphics/manager/graphics_manager.h>
#include <settings/manager.h>
#include <settings/config/config_component.h>
#include <settings/pads/pads_component.h>
#include <string>
#include <iterator>
#include <sstream>

[[noreturn]] void sequencer_task(void* /*pvParameters*/) {
    const auto& quantizer = Quantizer::instance();
    auto& sequencer = Sequencer::instance();

    while (true) {
        if (ulTaskNotifyTake(pdTRUE, portMAX_DELAY) > 0) {
            if (tud_midi_mounted())
            {
                constexpr uint8_t packet[4] = { 0x0F, 0xF8, 0x00, 0x00 };
                tud_midi_packet_write(packet);
            }

            if (sequencer.enable)
            {
                if (quantizer.ticks == 0) { // new quarter note
                    //handle_step_trigger(quantizer.steps);
                    sequencer.step_trigger_on(quantizer.steps);
                } else if (quantizer.ticks == 5) // end of quarter note
                {
                    sequencer.step_trigger_off(quantizer.steps);
                }
            }
        }
    }
}

[[noreturn]] void drumpad_task(void* /*pvParameters*/)
{
    auto& padsManager = PadsManager::instance();
    pad_midi_event_t value;
    while (true)
    {
        if (xQueueReceive(padsManager.pads_midi_events, &value, portMAX_DELAY) == pdTRUE)
        {
            //ESP_LOGI("DRUMPAD", "%u %u %i", value.channel, value.velocity, value.type);
            //ESP_LOGI("DRUMPAD", "%u %u", value.channel, value.velocity);
            if (tud_midi_mounted() && padsManager.is_enabled)
            {
                if (value.type == NOTE_ON)
                {
                    const uint8_t packet[4] = { 0x09, static_cast<uint8_t>(0x90 | (value.channel & 0x0F)), value.note, value.velocity };
                    tud_midi_packet_write(packet);
                } else
                {
                    const uint8_t packet[4] = { 0x08, static_cast<uint8_t>(0x80 | (value.channel & 0x0F)), value.note, value.velocity };
                    tud_midi_packet_write(packet);
                }
            }
            //vTaskDelay(1);
        }
    }
}

/*
TODO: delete this task
QueueHandle_t settings_updates = nullptr;

void settings_task(void*)
{
    ESP_LOGI("SAMPLER", "Settings Task!");
    auto& settings = SettingsManager::instance();
    auto& quantizer = Quantizer::instance();

    uint32_t update;
    while (true)
    {
        if (xQueueReceive(settings_updates, &update, portMAX_DELAY) == pdTRUE)
        {
            if (update == EVENT_UPDATE_BPM)
            {
                const auto config = settings.get_component<ConfigComponent>("config");
                quantizer.start(config->bpm());
            }
        }
    }
}
*/

static TaskHandle_t vendor_task_h = nullptr;

extern "C" void tud_vendor_rx_cb(uint8_t, const uint8_t*, uint16_t)
{
    if (vendor_task_h)
    {
        xTaskNotifyGive(vendor_task_h);
    }
}

void vendor_respond(std::string string)
{
    string += "\n";
    tud_vendor_write(string.c_str(), string.size());
    tud_vendor_write_flush();
}

void on_vendor_cmd(const std::vector<std::string>& cmd)
{
    // Make abstract layer for vendor CMDs
    const auto& name = cmd.at(0);
    if (name == "ECHO")
    {
        vendor_respond("ECHO");
        return;
    }

    if (name == "READ_CONFIG")
    {
        if (cmd.size() != 2)
        {
            vendor_respond("invalid args");
            return;
        }

        const auto& component_name = cmd.at(1);
        auto* component = SettingsManager::instance().get_component(component_name);
        if (component == nullptr)
        {
            vendor_respond("invalid component_name");
            return;
        }
        const auto buffer = component->direct_read("");
        vendor_respond(buffer);
        return;
    }

    if (name == "WRITE_CONFIG")
    {
        if (cmd.size() < 3)
        {
            vendor_respond("invalid args");
            return;
        }

        const auto& component_name = cmd.at(1);
        auto* component = SettingsManager::instance().get_component(component_name);
        if (component == nullptr)
        {
            vendor_respond("invalid component_name");
            return;
        }
        auto arg = cmd.at(2);
        std::string buffer;
        if (arg.starts_with("-"))
        {
            arg.erase(0, 1);
            if (cmd.size() < 4)
            {
                vendor_respond("invalid args");
                return;
            }

            for (size_t i = 3; i < cmd.size(); i++)
            {
                buffer += cmd[i];
            }
        } else
        {
            arg = "";
            for (size_t i = 2; i < cmd.size(); i++)
            {
                buffer += cmd[i];
            }
        }
        if (component->direct_write(buffer, arg))
            vendor_respond("Success");
        else
            vendor_respond("Failed");
    }
}

[[noreturn]] void read_vendor_task(void* /*pvParameters*/) {
    vendor_task_h = xTaskGetCurrentTaskHandle();
    uint8_t buffer[64];

    std::string message;

    while (true) {
        ulTaskNotifyTake(pdTRUE, portMAX_DELAY);

        while (tud_vendor_available()) {
            const uint32_t count = tud_vendor_read(buffer, sizeof(buffer));
            if (count == 0) continue;

            message.append(reinterpret_cast<char*>(buffer), count);

            size_t pos;
            while ((pos = message.find('\n')) != std::string::npos)
            {
                std::string cmd = message.substr(0, pos);
                message.erase(0, pos + 1);

                ESP_LOGI("Vendor", "Received: %s", cmd.c_str());

                std::istringstream iss(cmd);
                std::vector results(
                    std::istream_iterator<std::string>{iss},
                    std::istream_iterator<std::string>()
                    );
                on_vendor_cmd(results);
            }
        }
    }
}

extern "C" [[noreturn]] void app_main()
{
    ESP_ERROR_CHECK(nvs_flash_init());
    USB::init();

    vTaskDelay(pdMS_TO_TICKS(2000));

    USB::drain_rx();
    xTaskCreate(
        read_vendor_task,
        "vendor_rx",
        4096,
        nullptr,
        4,
        &vendor_task_h
    );

    auto& settings = SettingsManager::instance();
    if (!settings.init())
    {
        ESP_LOGE("SettingsManager", "Failed to load SettingsManager");
        while (true)
        {
            vTaskDelay(10);
        }
    }

    settings.add_component(std::make_unique<ConfigComponent>());
    settings.add_component(std::make_unique<PadsComponent>());

    //xTaskCreate(settings_task, "settings", 2048, nullptr, 10, nullptr);

    /*
     * I2C bus for:
     *  - LCD
     *  - Buttons' LEDs
     */
    static i2c_master_bus_handle_t i2c_bus_handle = nullptr;
    i2c_master_bus_config_t bus_config = {};
    bus_config.clk_source = I2C_CLK_SRC_DEFAULT;
    bus_config.glitch_ignore_cnt = 7;
    bus_config.i2c_port = I2C_NUM_1;
    bus_config.sda_io_num = GPIO_NUM_21;
    bus_config.scl_io_num = GPIO_NUM_18;
    bus_config.clk_source = I2C_CLK_SRC_DEFAULT;
    bus_config.flags.enable_internal_pullup = true;
    ESP_ERROR_CHECK(i2c_new_master_bus(&bus_config, &i2c_bus_handle));

    //
    auto& quantizer = Quantizer::instance();
    auto& sequencer = Sequencer::instance();
    (void)sequencer;

    auto& padsManager = PadsManager::instance();
    padsManager.init_adc({
        .port_num = I2C_NUM_0,
        .sda_num = GPIO_NUM_12,
        .scl_num = GPIO_NUM_11,
        .adc1_addr = 0x48,
        .adc2_addr = 0x49
    });

    xTaskCreatePinnedToCore(
        sequencer_task,
        "sequencer",
        4096,
        nullptr,
        configMAX_PRIORITIES - 1,
        &quantizer.task_handle,
        1
    );

    xTaskCreatePinnedToCore(
        drumpad_task,
        "drumpad",
        4096,
        nullptr,
        configMAX_PRIORITIES - 1,
        nullptr,
        1
    );

    // Graphics
    GraphicsManager graphics_manager;
    graphics_manager.install_driver(std::make_unique<lcd1602_driver>(i2c_bus_handle, 0x27));
    //graphics_manager.install_driver(std::make_unique<logger_driver>());

    graphics_manager.load_screen("home", create_home_screen);
    graphics_manager.load_screen("settings", create_settings_screen);
    graphics_manager.load_screen("sequencer", create_sequencer_screen);
    graphics_manager.load_screen("pad_settings", create_pad_settings_screen);

    graphics_manager.navigate("home");

    // Hardware inputs
    QueueHandle_t selector_events = xQueueCreate(10, sizeof(selector_event_t));
    selector_config_t selector_config = {
        .clk_gpio = GPIO_NUM_8,
        .data_gpio = GPIO_NUM_7,
        .btn_gpio = GPIO_NUM_9,
        .events = selector_events
    };
    Selector selector(&selector_config);
    (void)selector;

    /* Graphics task */
    QueueSetHandle_t graphics_inputs_events = xQueueCreateSet(10 + 64);

    xQueueAddToSet(selector_events, graphics_inputs_events);
    xQueueAddToSet(padsManager.pads_input_events, graphics_inputs_events);

    padsManager.start_task();

    selector_event_t selector_event;
    pad_input_event_t pad_input_event;

    uint32_t press_start_time = 0;
    uint32_t pads_press_start_time[8] = {};

    graphics_manager.update();
    while (true)
    {
        constexpr uint32_t LONG_PRESS_THRESHOLD_MS = 500;
        QueueSetMemberHandle_t input = xQueueSelectFromSet(graphics_inputs_events, portMAX_DELAY);

        if (input == selector_events)
        {
            xQueueReceive(selector_events, &selector_event, 0);

            switch (selector_event)
            {
            case ROTATION_LEFT:
                graphics_manager.send_event(EVENT_SCROLL_LEFT);
                break;
            case ROTATION_RIGHT:
                graphics_manager.send_event(EVENT_SCROLL_RIGHT);
                break;
            case BUTTON_PRESSED:
                press_start_time = esp_log_timestamp();
                break;
            case BUTTON_RELEASED:
                {
                    const uint32_t duration = esp_log_timestamp() - press_start_time;

                    if (duration >= LONG_PRESS_THRESHOLD_MS)
                    {
                        graphics_manager.send_event(EVENT_BACK);
                    } else
                    {
                        graphics_manager.send_event(EVENT_CLICK);
                    }
                } break;
            default:
                break;
            }
        } else if (input == padsManager.pads_input_events)
        {
            xQueueReceive(padsManager.pads_input_events, &pad_input_event, 0);

            const auto channel = pad_input_event.channel;

            if (pad_input_event.pressed)
            {
                pads_press_start_time[channel] = esp_log_timestamp();
                continue;
            } else
            {
                const uint32_t duration = esp_log_timestamp() - pads_press_start_time[channel];
                uint32_t custom_event = 0 | (channel & 0b111);

                if (duration >= LONG_PRESS_THRESHOLD_MS)
                {
                    custom_event |= (1 << 3);
                }
                if (!graphics_manager.send_custom_event(custom_event))
                    continue;
            }
        }

        graphics_manager.update();
        /*
        TODO: delete this code
        QueueSetMemberHandle_t input = xQueueSelectFromSet(graphics_inputs_events, portMAX_DELAY);

        if (input == selector_events)
        {
            xQueueReceive(selector_events, &selector_event, 0);

            if (selector_event == ROTATION_LEFT)
                lcd.action_left();
            else if (selector_event == ROTATION_RIGHT)
                lcd.action_right();
            else if (selector_event == BUTTON_PRESSED)
            {
                press_start_time = esp_log_timestamp();
            } else if (selector_event == BUTTON_RELEASED)
            {
                const uint32_t duration = esp_log_timestamp() - press_start_time;

                if (duration >= LONG_PRESS_THRESHOLD_MS)
                {
                    lcd.action_long_click();
                } else
                {
                    lcd.action_click();
                }
            }
        }
        else if (input == padsManager.pads_input_events)
        {
            xQueueReceive(padsManager.pads_input_events, &pad_input_event, 0);

            const auto channel = pad_input_event.channel;

            if (pad_input_event.pressed)
            {
                pads_press_start_time[channel] = esp_log_timestamp();
                continue;
            } else
            {
                const uint32_t duration = esp_log_timestamp() - pads_press_start_time[channel];
                uint32_t custom_event = 0 | (channel & 0b111);

                if (duration >= LONG_PRESS_THRESHOLD_MS)
                {
                    custom_event |= (1 << 3);
                }
                if (!lcd.custom_event(custom_event))
                    continue;
            }
        }

        lcd.render();*/
    }
}
