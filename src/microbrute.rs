use state::State;

pub fn print_state(state: &State) { // UI
    println!("ARTURIA MICROBRUTE STATE");
    println!("Note priority: {}", state.note_priority);
    println!("Velocity response: {}", state.velocity_response);
    println!("Play: {}", state.play);
    println!("Sequence retrig: {}", state.seq_retrig);
    println!("Next sequence: {}", state.next_seq);
    println!("Step on: {}", state.step_on);
    println!("Step: {}", state.step);
    println!("LFO key retrig: {}", state.lfo_key_retrig);
    println!("Env legatio mode: {}", state.env_legato_mode);
    println!("Gate: {}", state.gate);
    println!("Sync: {}", state.sync);
    println!("Bend range: {}", state.bend_range);
    println!("MIDI receive channel: {}", state.midi_recv_chan);
    println!("MIDI send channel: {}", state.midi_send_chan);
}

pub fn start_communication_command() -> Vec<u8> {
    vec![
        0xf0,
        0x7e,
        0x7f,
        0x06,
        0x01,
        0xf7
    ]
}

pub fn init_command(counter: u8, datum: u8) -> Vec<u8> {
    vec![
        0xf0,
        0x00,
        0x20,
        0x6b,
        0x05,
        0x01,
        counter,
        0x00,
        datum,
        0xf7
    ]
}

pub fn init_data() -> Vec<u8> {
    vec![
        0x06,
        0x08,
        0x35,
        0x10,
        0x2f,
        0x0c,
        0x0e,
        0x12,
        0x33,
        0x2d,
        0x39,
        0x37,
        0x2b,
        0x3d
    ]
}

pub fn set_command(counter: u8, command_type: &str, value: &str) -> Vec<u8> {
    vec![
        0xf0,
        0x00,
        0x20,
        0x6b,
        0x05,
        0x01,
        counter,
        0x01,
        _command_type(command_type),
        _value(command_type, value),
        0xf7
    ]
}

pub fn read_note_priority(value: u8) -> &'static str {
    match value {
        0x00 => "LAST",
        0x01 => "LOW",
        0x02 => "HIGH",
        _    => "UNKNOWN"
    }
}

pub fn read_velocity_response(value: u8) -> &'static str {
    match value {
        0x00 => "/",
        0x01 => "(",
        0x02 => ")",
        _    => "UNKNOWN"
    }
}

pub fn read_play(value: u8) -> &'static str {
    match value {
        0x00 => "HOLD",
        0x01 => "NOTE ON",
        _    => "UNKNOWN"
    }
}

pub fn read_seq_retrig(value: u8) -> &'static str {
    match value {
        0x00 => "RESET",
        0x01 => "LEGATO",
        0x02 => "NONE",
        _    => "UNKNOWN"
    }
}

pub fn read_next_seq(value: u8) -> &'static str {
    match value {
        0x00 => "END",
        0x01 => "INSTANT RESET",
        0x02 => "INSTANT CONTINUE",
        _    => "UNKNOWN"
    }
}

pub fn read_step_on(value: u8) -> &'static str {
    match value {
        0x00 => "CLK",
        0x01 => "GATE",
        _    => "UNKNOWN"
    }
}

pub fn read_step(value: u8) -> &'static str {
    match value {
        0x04 => "1/4",
        0x08 => "1/8",
        0x10 => "1/16",
        0x20 => "1/32",
        _    => "UNKNOWN"
    }
}

pub fn read_lfo_key_retrig(value: u8) -> &'static str {
    match value {
        0x00 => "OFF",
        0x01 => "ON",
        _    => "UNKNOWN"
    }
}

pub fn read_env_legato_mode(value: u8) -> &'static str {
    match value {
        0x00 => "OFF",
        0x01 => "ON",
        _    => "UNKNOWN"
    }
}

pub fn read_gate(value: u8) -> &'static str {
    match value {
        0x01 => "SHORT",
        0x02 => "MEDIUM",
        0x03 => "LONG",
        _    => "UNKNOWN"
    }
}

pub fn read_sync(value: u8) -> &'static str {
    match value {
        0x00 => "AUTO",
        0x01 => "INTERNAL",
        0x02 => "EXTERNAL",
        _    => "UNKNOWN"
    }
}

pub fn read_bend_range(value: u8) -> &'static str {
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
        _    => "UNKNOWN"
    }
}

pub fn read_midi_recv_chan(value: u8) -> &'static str {
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
        0x10 => "ALL",
        _    => "UNKNOWN"
    }
}

pub fn read_midi_send_chan(value: u8) -> &'static str {
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
        _    => "UNKNOWN"
    }
}

pub fn set_microbrute_state(state: &mut State, message: &[u8]) {
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

fn _command_type(command_type: &str) -> u8 {
    match command_type {
        "note_priority" => 0x0b,
        "velocity_response" => 0x11,
        "play" => 0x2e,
        "seq_retrig" => 0x34,
        "next_seq" => 0x32,
        "step_on" => 0x2a,
        "step" => 0x38,
        "lfo_key_retrig" => 0x0f,
        "env_legato_mode" => 0x0d,
        "gate" => 0x36,
        "sync" => 0x3c,
        "bend_range" => 0x2c,
        "midi_recv_chan" => 0x05,
        "midi_send_chan" => 0x07,
        _ => panic!("Unknown command!")
    }
}

fn _value(command_type: &str, value: &str) -> u8 {
    match command_type {
        "note_priority" => note_priority_value(value),
        "velocity_response" => velocity_response_value(value),
        "play" => play_value(value),
        "seq_retrig" => seq_retrig_value(value),
        "next_seq" => next_seq(value),
        "step_on" => step_on_value(value),
        "step" => step_value(value),
        "lfo_key_retrig" => lfo_key_retrig_value(value),
        "env_legato_mode" => env_legato_mode_value(value),
        "gate" => gate_value(value),
        "sync" => sync_value(value),
        "bend_range" => bend_range_value(value),
        "midi_recv_chan" => midi_recv_chan_value(value),
        "midi_send_chan" => midi_send_chan_value(value),
        _ => panic!("Unknown command!")
    }
}

fn note_priority_value(value: &str) -> u8 {
    match value {
        "last" => 0x00,
        "low" => 0x01,
        "high" => 0x02,
        _ => panic!("Unknown note priority value!")
    }
}

fn velocity_response_value(value: &str) -> u8 {
    match value {
        "/" => 0x00,
        "(" => 0x01,
        ")" => 0x02,
        _ => panic!("Unknown velocity response value!")
    }
}

fn play_value(value: &str) -> u8 {
    match value {
        "hold" => 0x00,
        "note_on" => 0x01,
        _ => panic!("Unknown play value!")
    }
}

fn seq_retrig_value(value: &str) -> u8 {
    match value {
        "reset" => 0x00,
        "legato" => 0x01,
        "none" => 0x02,
        _ => panic!("Unknown seq retrig value!")
    }
}

fn next_seq(value: &str) -> u8 {
    match value {
        "end" => 0x00,
        "instant_reset" => 0x01,
        "instant_cont" => 0x02,
        _ => panic!("Unknown next seq value!")
    }
}

fn step_on_value(value: &str) -> u8 {
    match value {
        "clk" => 0x00,
        "gate" => 0x01,
        _ => panic!("Unknown step on vlaue!")
    }
}

fn step_value(value: &str) -> u8 {
    match value {
        "1/4" => 0x04,
        "1/8" => 0x08,
        "1/16" => 0x10,
        "1/32" => 0x20,
        _ => panic!("Unknown step value!")
    }
}

fn lfo_key_retrig_value(value: &str) -> u8 {
    match value {
        "off" => 0x00,
        "on" => 0x01,
        _ => panic!("Unknown LFO key retrig value!")
    }
}

fn env_legato_mode_value(value: &str) -> u8 {
    match value {
        "off" => 0x00,
        "on" => 0x01,
        _ => panic!("Unknown env legato mode value!")
    }
}

fn gate_value(value: &str) -> u8 {
    match value {
        "short" => 0x01,
        "medium" => 0x02,
        "long" => 0x03,
        _ => panic!("Unknown gate value!")
    }
}

fn sync_value(value: &str) -> u8 {
    match value {
        "auto" => 0x00,
        "internal" => 0x01,
        "external" => 0x02,
        _ => panic!("Unknown sync value!")
    }
}

fn bend_range_value(value: &str) -> u8 {
    match value {
        "1" => 0x01,
        "2" => 0x02,
        "3" => 0x03,
        "4" => 0x04,
        "5" => 0x05,
        "6" => 0x06,
        "7" => 0x07,
        "8" => 0x08,
        "9" => 0x09,
        "10" => 0x0a,
        "11" => 0x0b,
        "12" => 0x0c,
        _ => panic!("Unknown bend range value!")
    }
}

fn midi_recv_chan_value(value: &str) -> u8 {
    match value {
        "1" => 0x00,
        "2" => 0x01,
        "3" => 0x02,
        "4" => 0x03,
        "5" => 0x04,
        "6" => 0x05,
        "7" => 0x06,
        "8" => 0x07,
        "9" => 0x08,
        "10" => 0x09,
        "11" => 0x0a,
        "12" => 0x0b,
        "13" => 0x0c,
        "14" => 0x0d,
        "15" => 0x0e,
        "16" => 0x0f,
        "ALL" => 0x10,
        _ => panic!("Unknown MIDI receive channel value!")
    }
}

fn midi_send_chan_value(value: &str) -> u8 {
    match value {
        "1" => 0x00,
        "2" => 0x01,
        "3" => 0x02,
        "4" => 0x03,
        "5" => 0x04,
        "6" => 0x05,
        "7" => 0x06,
        "8" => 0x07,
        "9" => 0x08,
        "10" => 0x09,
        "11" => 0x0a,
        "12" => 0x0b,
        "13" => 0x0c,
        "14" => 0x0d,
        "15" => 0x0e,
        "16" => 0x0f,
        _ => panic!("Unknown MIDI send channel value!")
    }
}
