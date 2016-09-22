use std::env;
use state::State;
use microbrute;
use utils;

pub fn handle_incoming_midi_message(state: &mut State, message: &[u8]) {
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

fn handle_sysex_message(state: &mut State, message: &[u8]) {
    match env::var("DEBUG_MESSAGES") {
        Ok(_) => println!("{} (len = {})", utils::to_hex_string(&message), message.len()),
        Err(_) => (),
    }
    match is_sysex_state_response(message) {
        true => handle_sysex_state_response(state, message),
        false => (),
    }
}

fn handle_sysex_state_response(state: &mut State, message: &[u8]) {
    match env::var("DEBUG_MESSAGES") {
        Ok(_) => println!("State response: {:02x}: {:02x}", message[8], message[9]),
        Err(_) => (),
    }
    microbrute::set_microbrute_state(state, message);
}
