#include "utils.h"
#include <array>

std::string Utils::int_to_note(const int note)
{
    if (note < 0 || note > 127) return "";

    static const std::array<std::string, 12> note_names = {
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
    };

    const int octave = (note / 12) - 1;
    const int name_index = note % 12;

    return note_names[name_index] + std::to_string(octave);
}