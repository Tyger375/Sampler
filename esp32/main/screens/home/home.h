#ifndef SAMPLER_HOME_SCREEN_H
#define SAMPLER_HOME_SCREEN_H

#include "graphics/screen/screen.h"
#include "graphics/lcd.h"

class HomeScreen : public Screen
{
public:
    HomeScreen(lcd&);
    bool on_custom_event(uint32_t) override;
};

#endif //SAMPLER_HOME_SCREEN_H