idf_component_register(SRCS
        main.cpp

        ads1015/ads1015.cpp
        ads1015/ads1015.hpp

        usb/usb.cpp
        usb/usb.h

        settings/settings_manager.cpp
        settings/settings_manager.h

        quantizer/quantizer.cpp
        quantizer/quantizer.h

        sequencer/sequencer.cpp
        sequencer/sequencer.h

        pads/pads.cpp
        pads/pads.h

        utils/utils.cpp
        utils/utils.h

        selector/selector.cpp
        selector/selector.h

        screens/screens.h

        screens/home/home.cpp
        screens/home/home.h

#        screens/settings/settings.cpp
#        screens/settings/settings.h
#
#        screens/pad_settings/pad_settings.cpp
#        screens/pad_settings/pad_settings.h
#
#        screens/sequencer/sequencer.cpp
#        screens/sequencer/sequencer.h

        INCLUDE_DIRS "."
        REQUIRES
            graphics_lib
            nvs_flash
            esp_driver_i2c
            esp_driver_gptimer
            esp_timer
)
