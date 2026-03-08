#ifndef SAMPLER_UTILS_H
#define SAMPLER_UTILS_H

#include <string>

namespace Utils
{
    constexpr uint8_t MAX_MIDI_NOTE = 127;
    constexpr uint8_t MAX_MIDI_CHANNELS = 16;

    std::string int_to_note(int);
}


#endif //SAMPLER_UTILS_H