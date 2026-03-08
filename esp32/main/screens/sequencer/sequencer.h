#ifndef SAMPLER_SEQUENCER_SCREEN_H
#define SAMPLER_SEQUENCER_SCREEN_H
#include <graphics/manager/graphics_manager.h>
#include <graphics/screen/screen.h>

class SequencerScreen : public Screen
{
    int8_t pageFocus = -1;
    uint8_t editingTrack = 0;

    void sequencer_page();

    void show_tracks();
    void edit_track(uint16_t);

    bool on_back() override;
    bool on_custom_event(uint32_t) override;
public:
    explicit SequencerScreen(GraphicsManager&);
};


#endif //SAMPLER_SEQUENCER_SCREEN_H