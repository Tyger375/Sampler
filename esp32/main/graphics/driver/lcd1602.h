#ifndef SAMPLER_LCD1602_H
#define SAMPLER_LCD1602_H

#include "driver/i2c_types.h"
#include <string>

class lcd1602
{
    i2c_master_dev_handle_t dev_handle{};

    void send_command(bool, uint8_t) const;
    void send_nibble(uint8_t, bool) const;
public:
    lcd1602(i2c_master_bus_handle_t, uint8_t address);

    void init() const;
    void clear_screen() const;
    void set_cursor(uint8_t, uint8_t) const;
    void write(const std::string&) const;
};


#endif //SAMPLER_LCD1602_H