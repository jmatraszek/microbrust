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
        "Last" => 0x00,
        "Low" => 0x01,
        "High" => 0x02,
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
        "Hold" => 0x00,
        "Note on" => 0x01,
        _ => panic!("Unknown play value!")
    }
}

fn seq_retrig_value(value: &str) -> u8 {
    match value {
        "Reset" => 0x00,
        "Legato" => 0x01,
        "None" => 0x02,
        _ => panic!("Unknown seq retrig value!")
    }
}

fn next_seq(value: &str) -> u8 {
    match value {
        "End" => 0x00,
        "Instant reset" => 0x01,
        "Instant continue" => 0x02,
        _ => panic!("Unknown next seq value!")
    }
}

fn step_on_value(value: &str) -> u8 {
    match value {
        "Clk" => 0x00,
        "Gate" => 0x01,
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
        "Off" => 0x00,
        "On" => 0x01,
        _ => panic!("Unknown LFO key retrig value!")
    }
}

fn env_legato_mode_value(value: &str) -> u8 {
    match value {
        "Off" => 0x00,
        "On" => 0x01,
        _ => panic!("Unknown env legato mode value!")
    }
}

fn gate_value(value: &str) -> u8 {
    match value {
        "Short" => 0x01,
        "Medium" => 0x02,
        "Long" => 0x03,
        _ => panic!("Unknown gate value!")
    }
}

fn sync_value(value: &str) -> u8 {
    match value {
        "Auto" => 0x00,
        "Internal" => 0x01,
        "External" => 0x02,
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
        "All" => 0x10,
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
