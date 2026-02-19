#include "usb.h"
#include "tinyusb.h"
#include "tinyusb_default_config.h"
#include "tinyusb_cdc_acm.h"
#include "tinyusb_console.h"

#define TUSB_DESCRIPTOR_TOTAL_LEN (TUD_CONFIG_DESC_LEN + TUD_MIDI_DESC_LEN + TUD_CDC_DESC_LEN + TUD_VENDOR_DESC_LEN)

enum interface_count {
    ITF_NUM_MIDI    = 0,
    ITF_NUM_CDC     = 2,
    ITF_NUM_VENDOR  = 4,
    ITF_COUNT
};

enum usb_endpoints {
    EPNUM_MIDI_OUT      = 0x01,
    EPNUM_MIDI_IN       = 0x81,

    EPNUM_CDC_NOTIF     = 0x82,
    EPNUM_CDC_OUT       = 0x03,
    EPNUM_CDC_IN        = 0x83,

    EPNUM_VENDOR_OUT    = 0x04,
    EPNUM_VENDOR_IN     = 0x84,
};

static const char* str_desc[] = {
    // array of pointer to string descriptors
    (char[]){0x09, 0x04}, // 0: is supported language is English (0x0409)
    "TK", // 1: Manufacturer
    "MPC", // 2: Product
    "123456", // 3: Serials, should use chip ID
    "MIDI Interface", // 4: MIDI
    "Log Console",
    "Vendor Interface"
};

static constexpr uint8_t s_composite_cfg_desc[] = {
    // Configuration number, interface count, string index, total length, attribute, power in mA
    TUD_CONFIG_DESCRIPTOR(1, ITF_COUNT, 0, TUSB_DESCRIPTOR_TOTAL_LEN, 0, 100),

    // MIDI (Intf 0, 1)
    // Interface number, string index, EP Out & EP In address, EP size
    TUD_MIDI_DESCRIPTOR(ITF_NUM_MIDI, 4, EPNUM_MIDI_OUT, EPNUM_MIDI_IN, 64),

    // CDC (Intf 2, 3) - String Index 5
    // Interface number, string index, EP Out & EP In address, EP size
    TUD_CDC_DESCRIPTOR(ITF_NUM_CDC, 5, EPNUM_CDC_NOTIF, 8, EPNUM_CDC_OUT, EPNUM_CDC_IN, 64),

    // Vendor (Intf 4) - String Index 6
    // Interface number, string index, EP Out & EP In address, EP size
    TUD_VENDOR_DESCRIPTOR(ITF_NUM_VENDOR, 6, EPNUM_VENDOR_OUT, EPNUM_VENDOR_IN, 64),
};

void USB::init()
{
    // Configuring USB device
    auto usb_config = TINYUSB_DEFAULT_CONFIG();
    usb_config.descriptor.string = str_desc;
    usb_config.descriptor.string_count = sizeof(str_desc) / sizeof(str_desc[0]);
    usb_config.descriptor.full_speed_config = s_composite_cfg_desc;
    ESP_ERROR_CHECK(tinyusb_driver_install(&usb_config));

    tinyusb_config_cdcacm_t acm_cfg = {};
    acm_cfg.cdc_port = TINYUSB_CDC_ACM_0;
    ESP_ERROR_CHECK(tinyusb_cdcacm_init(&acm_cfg));
    ESP_ERROR_CHECK(tinyusb_console_init(TINYUSB_CDC_ACM_0));
}

void usb_drain_task(void* /* pvParameters */)
{
    // Discard MIDI and CDC inputs
    while (true)
    {
        if (tud_mounted())
        {
            if (tud_midi_available())
            {
                uint8_t packet[4];
                while (tud_midi_packet_read(packet));
            }

            if (tud_cdc_n_available(TINYUSB_CDC_ACM_0))
            {
                uint8_t buffer[64];
                tud_cdc_read(buffer, sizeof(buffer));
            }
        }

        vTaskDelay(pdMS_TO_TICKS(10));
    }
}

void USB::drain_rx()
{
    xTaskCreate(
        usb_drain_task,
        "usb_drain_task",
        2048,
        nullptr,
        2,
        nullptr
    );
}
