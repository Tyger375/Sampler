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
