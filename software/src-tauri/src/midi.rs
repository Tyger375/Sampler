use std::sync::mpsc;
use std::thread;
use std::thread::park;
use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPorts};

pub enum MIDIEvent {
    NoteOn(u8, u8),
    NoteOff(u8)
}

pub fn create_midi_task() -> mpsc::Receiver<MIDIEvent> {
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    for port in in_ports.iter() {
        println!("MIDI port: {} ({})", midi_in.port_name(port).unwrap(), port.id());
    }
    let in_port = in_ports.get(1).unwrap().clone();

    let (events_tx, events_rx) = mpsc::channel::<MIDIEvent>();

    thread::spawn(move || {
        let _conn_in: MidiInputConnection<()> = midi_in.connect(
            &in_port,
            "midir-read-input",
            move |_, message, _| {
                match message[0] {
                    0xF8 => return,
                    0x90 => {
                        events_tx.send(MIDIEvent::NoteOn(message[1], message[2])).ok()
                    },
                    0x80 => events_tx.send(MIDIEvent::NoteOff(message[1])).ok(),
                    _ => return,
                };
            },
            (),
        ).expect("Failed to create connection");

        park();
    });

    events_rx
}