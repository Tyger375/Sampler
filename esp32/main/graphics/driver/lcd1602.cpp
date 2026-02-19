#include "lcd1602.h"
#include "esp_log.h"
#include "driver/i2c_master.h"
#include "driver/i2c.h"

#define RS_BIT      0x01  // P0 - Register Select
#define RW_BIT      0x02  // P1 - Read/Write (usually grounded, but keep 0)
#define EN_BIT      0x04  // P2 - Enable
#define BL_BIT      0x08  // P3 - Backlight Control (1 = On)

void lcd1602::send_command(const bool rs, const uint8_t value) const
{
    const uint8_t high = value & 0xF0;
    const uint8_t low  = (value << 4) & 0xF0;

    send_nibble(high, rs);
    send_nibble(low, rs);
}

void lcd1602::send_nibble(const uint8_t nibble, const bool rs) const
{
    const uint8_t data = nibble | BL_BIT | (rs ? RS_BIT : 0);

    /* EN high */
    uint8_t d = data | EN_BIT;
    i2c_master_transmit(dev_handle, &d, 1, 100);
    esp_rom_delay_us(1);

    /* EN low */
    d = data;
    i2c_master_transmit(dev_handle, &d, 1, 100);
    esp_rom_delay_us(40);
}

void lcd1602::set_cursor(uint8_t col, uint8_t row) const
{
    if (col > 15) col = 15;
    if (row > 1) row = 1;

    const uint8_t address = (row == 0) ? col : (0x40 + col);
    send_command(false, 0x80 | address);
}

lcd1602::lcd1602(const i2c_master_bus_handle_t bus_handle, const uint8_t address)
{
    i2c_device_config_t config{};
    config.dev_addr_length = I2C_ADDR_BIT_LEN_7;
    config.device_address = address;
    config.scl_speed_hz = 100000;

    ESP_ERROR_CHECK(i2c_master_bus_add_device(bus_handle, &config, &dev_handle));
    ESP_LOGI("LCD1602", "Device configured");
}

void lcd1602::init() const
{
    esp_rom_delay_us(50000); // wait for LCD power-up

    /* Reset sequence (8-bit mode) */
    send_nibble(0x30, false);
    esp_rom_delay_us(4500);

    send_nibble(0x30, false);
    esp_rom_delay_us(4500);

    send_nibble(0x30, false);
    esp_rom_delay_us(150);

    /* Switch to 4-bit mode */
    send_nibble(0x20, false);
    esp_rom_delay_us(150);

    /* Function set: 4-bit, 2-line, 5x8 font */
    send_command(false, 0x28);

    /* Display off */
    send_command(false, 0x08);

    /* Clear display */
    send_command(false, 0x01);
    esp_rom_delay_us(2000);

    /* Entry mode: increment, no shift */
    send_command(false, 0x06);

    /* Display on, cursor off, blink off */
    send_command(false, 0x0C);

    ESP_LOGI("LCD1602", "Device initialized");
}

void lcd1602::clear_screen() const
{
    send_command(false, 0x01);
    esp_rom_delay_us(2000);
}

void lcd1602::write(const std::string& str) const
{
    for (const char c : str)
    {
        send_command(true, static_cast<uint8_t>(c));
    }
}
