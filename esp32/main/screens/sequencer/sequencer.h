#ifndef SAMPLER_SEQUENCER_SCREEN_H
#define SAMPLER_SEQUENCER_SCREEN_H

#include "graphics/screen/screen.h"

class SequencerScreen : public Screen
{
    int focus = -1;
    uint8_t editingTrack = 0;

    void on_start();
    void show_tracks();
    void edit_track(uint16_t);
public:
    SequencerScreen();

    bool on_event(graphics_event_t, uint16_t) override;
    bool on_custom_event(uint32_t) override;
};


#endif //SAMPLER_SEQUENCER_SCREEN_H