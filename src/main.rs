extern crate midir;
mod utils;
mod microbrute;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::env;

use midir::{MidiInput, MidiOutput, Ignore};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let midi_out = try!(MidiOutput::new("Arturia Microbrute"));
    let out_port: u32 = get_midi_out_port(&midi_out);

    let midi_in = try!(MidiInput::new("Arturia Microbrute"));
    let in_port: u32 = get_midi_in_port(&midi_in);

    let conn_in = try!(midi_in.connect(in_port, "microbrute", |stamp, message, _| {
        handle_incoming_midi_message(message)
    }, ()).map_err(|e| e.kind()));

    let mut conn_out = try!(midi_out.connect(out_port, "microbrute").map_err(|e| e.kind()));
    try!(conn_out.send(&microbrute::start_communication_command()));
    sleep(Duration::from_millis(100));
    let mut counter: u8 = 0;
    for cmd in microbrute::init_data() {
        try!(conn_out.send(&microbrute::init_command(counter, cmd)));
        sleep(Duration::from_millis(100));
        counter += 1;
    }
    try!(conn_out.send(&microbrute::set_command(counter, "NOTE_PRIORITY", "LOW")));
    // try!(conn_out.send(&microbrute::set_command(1, "VELOCITY_RESPONSE", "/")));
    Ok(())
}

fn handle_incoming_midi_message(message: &[u8]) {
    match is_sysex_message(message) {
        true => handle_sysex_message(message),
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

fn handle_sysex_message(message: &[u8]) {
    match env::var("DEBUG_MESSAGES") {
        Ok(_) => println!("{} (len = {})", utils::to_hex_string(&message), message.len()),
        Err(_) => (),
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
