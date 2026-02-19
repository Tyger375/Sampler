#include "sequencer.h"

#include <class/midi/midi_device.h>

Sequencer::Sequencer()
{
    enable = false;
}

void Sequencer::step_trigger_on(uint8_t step)
{
    for (auto& track : tracks)
    {
        uint8_t track_step = step + (16 * (current_loop % track.loops));
        handle_track_on(track, track_step);
    }

    if (step == 15)
    {
        current_loop = ((current_loop + 1) % loops_num);
    }
}

void Sequencer::step_trigger_off(uint8_t step)
{
    for (auto& track : tracks)
    {
        uint8_t track_step = step + (16 * (current_loop % track.loops));
        handle_track_off(track, track_step);
    }
}

// step = local quarter note
void Sequencer::handle_track_on(sequencer_track_t& track, uint8_t step)
{
    if (track.trigger) return;

    for (auto trigger : track.triggers)
    {
        // converting trigger to a quarter note trigger
        uint8_t real_trigger = trigger * track.resolution;
        if (real_trigger == step)
        {
            // trigger
            track.trigger = true;

            if (tud_midi_mounted())
            {
                uint8_t packet[4] = { 0x09, 0x90, track.note, 127 };
                tud_midi_packet_write(packet);
            }
        }
    }
}

void Sequencer::handle_track_off(sequencer_track_t& track, uint8_t step)
{
    if (!track.trigger) return;

    for (auto trigger : track.triggers)
    {
        uint8_t real_trigger = ((trigger + 1) * track.resolution) - 1;
        if (real_trigger == step)
        {
            // trigger
            track.trigger = false;

            if (tud_midi_mounted())
            {
                uint8_t packet[4] = { 0x08, 0x80, track.note, 0 };
                tud_midi_packet_write(packet);
            }
        }
    }
}
