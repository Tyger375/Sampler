idf_component_register(SRCS
        # graphics/lcd.cpp
        # graphics/lcd.h

        graphics/manager/graphics_manager.cpp
        graphics/manager/graphics_manager.h

        graphics/graphics_types.h

        graphics/drivers/driver.h

        graphics/drivers/lcd1602/lcd1602.cpp
        graphics/drivers/lcd1602/lcd1602.h

        graphics/drivers/logger/logger.cpp
        graphics/drivers/logger/logger.h

        graphics/screen/screen.cpp
        graphics/screen/screen.h

        graphics/ui/element.cpp
        graphics/ui/element.h
        graphics/ui/event.h

        graphics/ui/text/text.cpp
        graphics/ui/text/text.h

        graphics/ui/button/button.cpp
        graphics/ui/button/button.h

        graphics/ui/row/row.cpp
        graphics/ui/row/row.h

        graphics/ui/intinput/intinput.cpp
        graphics/ui/intinput/intinput.h

        graphics/ui/checkbox/checkbox.cpp
        graphics/ui/checkbox/checkbox.h

        INCLUDE_DIRS "."
        PRIV_REQUIRES
            esp_driver_i2c
)