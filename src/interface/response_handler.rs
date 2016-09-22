use std::env;
use state::State;
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
    set_microbrute_state(state, message);
}

fn set_microbrute_state(state: &mut State, message: &[u8]) {
    match message[8] {
        0x0b => {
            state.set("note_priority", &read_note_priority(message[9]))
        },
        0x11 => {
            state.set("velocity_response", &read_velocity_response(message[9]))
        },
        0x2e => {
            state.set("play", &read_play(message[9]))
        },
        0x34 => {
            state.set("seq_retrig", &read_seq_retrig(message[9]))
        },
        0x32 => {
            state.set("next_seq", &read_next_seq(message[9]))
        },
        0x2a => {
            state.set("step_on", &read_step_on(message[9]))
        },
        0x38 => {
            state.set("step", &read_step(message[9]))
        },
        0x0f => {
            state.set("lfo_key_retrig", &read_lfo_key_retrig(message[9]))
        },
        0x0d => {
            state.set("env_legato_mode", &read_env_legato_mode(message[9]))
        },
        0x36 => {
            state.set("gate", &read_gate(message[9]))
        },
        0x3c => {
            state.set("sync", &read_sync(message[9]))
        },
        0x2c => {
            state.set("bend_range", &read_bend_range(message[9]))
        },
        0x05 => {
            state.set("midi_recv_chan", &read_midi_recv_chan(message[9]))
        },
        0x07 => {
            state.set("midi_send_chan", &read_midi_send_chan(message[9]))
        },
        _ => ()
    }
}

fn read_note_priority<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "Last",
        0x01 => "Low",
        0x02 => "High",
        _    => "Unknown"
    }
}

fn read_velocity_response<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "/",
        0x01 => "(",
        0x02 => ")",
        _    => "Unknown"
    }
}

fn read_play<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "Hold",
        0x01 => "Note on",
        _    => "Unknown"
    }
}

fn read_seq_retrig<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "Reset",
        0x01 => "Legato",
        0x02 => "None",
        _    => "Unknown"
    }
}

fn read_next_seq<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "End",
        0x01 => "Instant reset",
        0x02 => "Instant continue",
        _    => "Unknown"
    }
}

fn read_step_on<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "Clk",
        0x01 => "Gate",
        _    => "Unknown"
    }
}

fn read_step<'a>(value: u8) -> &'a str {
    match value {
        0x04 => "1/4",
        0x08 => "1/8",
        0x10 => "1/16",
        0x20 => "1/32",
        _    => "Unknown"
    }
}

fn read_lfo_key_retrig<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "Off",
        0x01 => "On",
        _    => "Unknown"
    }
}

fn read_env_legato_mode<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "Off",
        0x01 => "On",
        _    => "Unknown"
    }
}

fn read_gate<'a>(value: u8) -> &'a str {
    match value {
        0x01 => "Short",
        0x02 => "Medium",
        0x03 => "Long",
        _    => "Unknown"
    }
}

fn read_sync<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "Auto",
        0x01 => "Internal",
        0x02 => "External",
        _    => "Unknown"
    }
}

fn read_bend_range<'a>(value: u8) -> &'a str {
    match value {
        0x01 => "1",
        0x02 => "2",
        0x03 => "3",
        0x04 => "4",
        0x05 => "5",
        0x06 => "6",
        0x07 => "7",
        0x08 => "8",
        0x09 => "9",
        0x0a => "10",
        0x0b => "11",
        0x0c => "12",
        _    => "Unknown"
    }
}

fn read_midi_recv_chan<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "1",
        0x01 => "2",
        0x02 => "3",
        0x03 => "4",
        0x04 => "5",
        0x05 => "6",
        0x06 => "7",
        0x07 => "8",
        0x08 => "9",
        0x09 => "10",
        0x0a => "11",
        0x0b => "12",
        0x0c => "13",
        0x0d => "14",
        0x0e => "15",
        0x0f => "16",
        0x10 => "All",
        _    => "Unknown"
    }
}

fn read_midi_send_chan<'a>(value: u8) -> &'a str {
    match value {
        0x00 => "1",
        0x01 => "2",
        0x02 => "3",
        0x03 => "4",
        0x04 => "5",
        0x05 => "6",
        0x06 => "7",
        0x07 => "8",
        0x08 => "9",
        0x09 => "10",
        0x0a => "11",
        0x0b => "12",
        0x0c => "13",
        0x0d => "14",
        0x0e => "15",
        0x0f => "16",
        _    => "Unknown"
    }
}
