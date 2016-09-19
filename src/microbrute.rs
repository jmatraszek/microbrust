// pub struct MicrobruteMidi {
//     input: MidiInput,
//     output: MidiOutput,
//     state: State
// }

pub struct MicrobruteState<'a> {
    pub note_priority: &'a str,
    pub velocity_response: &'a str,
    pub play: &'a str,
    pub seq_retrig: &'a str,
    pub next_seq: &'a str,
    pub step_on: &'a str,
    pub step: &'a str,
    pub lfo_key_retrig: &'a str,
    pub env_legato_mode: &'a str,
    pub gate: &'a str,
    pub sync: &'a str,
    pub bend_range: &'a str,
    pub midi_recv_chan: &'a str,
    pub midi_send_chan: &'a str,
}

pub fn print_microbrute_state(state: &MicrobruteState) {
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
  match(value) {
    0x00 => "LAST",
    0x01 => "LOW",
    0x02 => "HIGH",
    _    => "UNKNOWN"
  }
}

pub fn set_microbrute_state(state: &mut MicrobruteState, message: &[u8]) {
    match(message[8]) {
      0x0b => {
        println!("NOTE PRIORITY: {}", read_note_priority(message[9]));
        state.note_priority = &read_note_priority(message[9])
      },
      _ => ()
    }
}

fn _command_type(command_type: &str) -> u8 {
    match command_type {
        "NOTE_PRIORITY" => 0x0b,
        "VELOCITY_RESPONSE" => 0x11,
        "PLAY" => 0x2e,
        "SEQ_RETRIG" => 0x34,
        "NEXT_SEQ" => 0x32,
        "STEP_ON" => 0x2a,
        "STEP" => 0x38,
        "LFO_KEY_RETRIG" => 0x0f,
        "ENV_LEGATO_MODE" => 0x0d,
        "GATE" => 0x36,
        "SYNC" => 0x3c,
        "BEND_RANGE" => 0x2c,
        "MIDI_RECV_CHAN" => 0x05,
        "MIDI_SEND_CHAN" => 0x07,
        _ => panic!("Unknown command!")
    }
}

fn _value(command_type: &str, value: &str) -> u8 {
    match command_type {
        "NOTE_PRIORITY" => note_priority_value(value),
        "VELOCITY_RESPONSE" => velocity_response_value(value),
        "PLAY" => play_value(value),
        "SEQ_RETRIG" => seq_retrig_value(value),
        "NEXT_SEQ" => next_seq(value),
        "STEP_ON" => step_on_value(value),
        "STEP" => step_value(value),
        "LFO_KEY_RETRIG" => lfo_key_retrig_value(value),
        "ENV_LEGATO_MODE" => env_legato_mode_value(value),
        "GATE" => gate_value(value),
        "SYNC" => sync_value(value),
        "BEND_RANGE" => bend_range_value(value),
        "MIDI_RECV_CHAN" => midi_recv_chan_value(value),
        "MIDI_SEND_CHAN" => midi_send_chan_value(value),
        _ => panic!("Unknown command!")
    }
}

fn note_priority_value(value: &str) -> u8 {
    match value {
        "LAST" => 0x00,
        "LOW" => 0x01,
        "HIGH" => 0x02,
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
        "HOLD" => 0x00,
        "NOTE_ON" => 0x01,
        _ => panic!("Unknown play value!")
    }
}

fn seq_retrig_value(value: &str) -> u8 {
    match value {
        "RESET" => 0x00,
        "LEGATO" => 0x01,
        "NONE" => 0x02,
        _ => panic!("Unknown seq retrig value!")
    }
}

fn next_seq(value: &str) -> u8 {
    match value {
        "END" => 0x00,
        "INSTANT_RESET" => 0x01,
        "INSTANT_CONT" => 0x02,
        _ => panic!("Unknown next seq value!")
    }
}

fn step_on_value(value: &str) -> u8 {
    match value {
        "CLK" => 0x00,
        "GATE" => 0x01,
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
        "OFF" => 0x00,
        "ON" => 0x01,
        _ => panic!("Unknown LFO key retrig value!")
    }
}

fn env_legato_mode_value(value: &str) -> u8 {
    match value {
        "OFF" => 0x00,
        "ON" => 0x01,
        _ => panic!("Unknown env legato mode value!")
    }
}

fn gate_value(value: &str) -> u8 {
    match value {
        "SHORT" => 0x01,
        "MEDIUM" => 0x02,
        "LONG" => 0x03,
        _ => panic!("Unknown gate value!")
    }
}

fn sync_value(value: &str) -> u8 {
    match value {
        "AUTO" => 0x00,
        "INTERNAL" => 0x01,
        "EXTERNAL" => 0x02,
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
