extern crate midir;

mod microbrute;
mod utils;

use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use std::env;
use midir::{MidiInput, MidiOutput};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut state = microbrute::MicrobruteState {
        note_priority: "UNKNOWN",
        velocity_response: "UNKNOWN",
        play: "UNKNOWN",
        seq_retrig: "UNKNOWN",
        next_seq: "UNKNOWN",
        step_on: "UNKNOWN",
        step: "UNKNOWN",
        lfo_key_retrig: "UNKNOWN",
        env_legato_mode: "UNKNOWN",
        gate: "UNKNOWN",
        sync: "UNKNOWN",
        bend_range: "UNKNOWN",
        midi_recv_chan: "UNKNOWN",
        midi_send_chan: "UNKNOWN",
    };

    let midi_out = try!(MidiOutput::new("Arturia Microbrute"));
    let out_port: u32 = get_midi_out_port(&midi_out);

    let mut midi_in = try!(MidiInput::new("Arturia Microbrute"));
    let in_port: u32 = get_midi_in_port(&midi_in);

    let conn_in = try!(midi_in.connect(in_port, "microbrute", |_, message, state| {
        handle_incoming_midi_message(state, message)
    }, state).map_err(|e| e.kind()));

    let mut conn_out = try!(midi_out.connect(out_port, "microbrute").map_err(|e| e.kind()));
    try!(conn_out.send(&microbrute::start_communication_command()));
    sleep(Duration::from_millis(100));
    let mut counter: u8 = 0;
    for cmd in microbrute::init_data() {
        try!(conn_out.send(&microbrute::init_command(counter, cmd)));
        sleep(Duration::from_millis(100));
        counter += 1;
    }

    let (midi_in_, state) = conn_in.close();
    midi_in = midi_in_;

    microbrute::print_microbrute_state(&state);
    try!(conn_out.send(&microbrute::set_command(counter, "NOTE_PRIORITY", "LAST")));
    sleep(Duration::from_millis(100));
    try!(conn_out.send(&microbrute::set_command(counter, "MIDI_RECV_CHAN", "9")));
    // try!(conn_out.send(&microbrute::set_command(1, "VELOCITY_RESPONSE", "/")));
    // microbrute::print_microbrute_state(&state);

    Ok(())
}

fn handle_incoming_midi_message(state: &mut microbrute::MicrobruteState, message: &[u8]) {
    match is_sysex_message(message) {
        true => handle_sysex_message(state, message),
        false => (),
    }
}

fn is_sysex_message(message: &[u8]) -> bool {
    let len: usize = message.len();
    if message[0] == 0xf0 && message[len-1] == 0xf7 {
        true
    }
    else {
        false
    }
}

fn is_sysex_state_response(message: &[u8]) -> bool {
    if message[1] == 0x00 && // Arturia's ID
        message[2] == 0x20 && // Arturia's ID
        message[3] == 0x6b && // Arturia's ID
        message[4] == 0x05 && // Microbrute's ID
        message[5] == 0x01 &&
        message[7] == 0x01
    {
        true
    }
    else {
        false
    }
}

fn handle_sysex_state_response(state: &mut microbrute::MicrobruteState, message: &[u8]) {
    match env::var("DEBUG_MESSAGES") {
        Ok(_) => println!("State response: {:02x}: {:02x}", message[8], message[9]),
        Err(_) => (),
    }
    microbrute::set_microbrute_state(state, message);
}

fn handle_sysex_message(state: &mut microbrute::MicrobruteState, message: &[u8]) {
    match env::var("DEBUG_MESSAGES") {
        Ok(_) => println!("{} (len = {})", utils::to_hex_string(&message), message.len()),
        Err(_) => (),
    }
    match is_sysex_state_response(message) {
        true => handle_sysex_state_response(state, message),
        false => (),
    }
}

fn get_midi_out_port(midi_out: &MidiOutput) -> u32 {
    let mut out_port: Option<u32> = None;

    for i in 0..midi_out.port_count() {
        match midi_out.port_name(i).unwrap().starts_with("MicroBrute") {
            true => {
                out_port = Some(i);
                break;
            },
            false => (),
        }
    }

    match out_port {
        Some(port_num) => return port_num,
        None => panic!("No Microbrute connected!"),
    }
}

fn get_midi_in_port(midi_in: &MidiInput) -> u32 {
    let mut in_port: Option<u32> = None;

    for i in 0..midi_in.port_count() {
        match midi_in.port_name(i).unwrap().starts_with("MicroBrute") {
            true => {
                in_port = Some(i);
                break;
            },
            false => (),
        }
    }

    match in_port {
        Some(port_num) => return port_num,
        None => panic!("No Microbrute connected!"),
    }
}
