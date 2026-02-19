#ifndef SAMPLER_SETTINGS_SCREEN_H
#define SAMPLER_SETTINGS_SCREEN_H

#include <graphics/lcd.h>

#include "graphics/screen/screen.h"

class SettingsScreen : public Screen
{
public:
    SettingsScreen(lcd& lcd);
    bool on_custom_event(uint32_t) override;
};


#endif //SAMPLER_SETTINGS_SCREEN_H