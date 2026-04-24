extern "C" {
    fn midi_mounted() -> bool;
    fn midi_out(packet: *const [u8; 4]);
}

pub struct MIDI;

impl MIDI {
    pub fn send(packet: &[u8; 4]) {
        unsafe {
            if midi_mounted() {
                midi_out(packet);
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MidiType {
    NoteOn,
    NoteOff,
}

impl From<MidiType> for (u8, u8) {
    fn from(value: MidiType) -> Self {
        match value {
            MidiType::NoteOn => (0x09, 0x90),
            MidiType::NoteOff => (0x08, 0x80),
        }
    }
}