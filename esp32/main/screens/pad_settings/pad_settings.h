#ifndef SAMPLER_PAD_SETTINGS_SCREEN_H
#define SAMPLER_PAD_SETTINGS_SCREEN_H

#include <graphics/screen/screen.h>

class PadSettingsScreen : public Screen
{
    int pageFocus = -1;

    void select_pad();
    void pad_selected();
public:
    PadSettingsScreen();

    void on_start() override;
    void on_end() override;

    bool on_back() override;
    bool on_custom_event(uint32_t) override;

    /*
    bool on_event(graphics_event_t, uint16_t) override;
    bool on_custom_event(uint32_t) override;
    */
};


#endif //SAMPLER_PAD_SETTINGS_SCREEN_H