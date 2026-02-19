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

        screens/settings/settings.cpp
        screens/settings/settings.h

        screens/pad_settings/pad_settings.cpp
        screens/pad_settings/pad_settings.h

        screens/sequencer/sequencer.cpp
        screens/sequencer/sequencer.h

        graphics/lcd.cpp
        graphics/lcd.h

        graphics/driver/lcd1602.cpp
        graphics/driver/lcd1602.h

        graphics/screen/screen.cpp
        graphics/screen/screen.h

        graphics/types.h
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
)
