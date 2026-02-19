#ifndef GRAPHICS_LCD1602_H
#define GRAPHICS_LCD1602_H

#include "../driver.h"
#include <vector>
#include <string>
#include <driver/i2c_master.h>

class lcd1602_driver : public GraphicsDriver
{
    i2c_master_dev_handle_t dev_handle{};

    void send_command(bool, uint8_t) const;
    void send_nibble(uint8_t, bool) const;

    void set_cursor(uint8_t, uint8_t) const;
    void write(const std::string&) const;
public:
    lcd1602_driver(i2c_master_bus_handle_t, uint8_t);

	void init() override;
    void draw(std::vector<std::string>) override;
	void clear() override;
};


#endif //GRAPHICS_LCD1602_H