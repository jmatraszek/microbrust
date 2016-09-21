extern crate midir;

use self::midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::result::Result;
use std::thread::sleep;
use std::time::Duration;
use std::env;
use state::State;
use microbrute;
use utils;

pub struct MidiInterface {
    // pub conn_out: MidiOutputConnection,
    pub counter: u8
}

impl MidiInterface {
    pub fn new() -> MidiInterface {
        MidiInterface {
            counter: 0
        }
    }

    pub fn update_state<'a, 'b>(&'a mut self, mut state: State<'b>) -> Result<State<'b>, Box<Error>> {
        let midi_out = try!(MidiOutput::new("Arturia Microbrute"));
        let out_port: u32 = MidiInterface::get_midi_out_port(&midi_out);
        let mut conn_out = try!(midi_out.connect(out_port, "microbrute").map_err(|e| e.kind()));

        let midi_in = try!(MidiInput::new("Arturia Microbrute"));
        let in_port: u32 = self.get_midi_in_port(&midi_in);
        let mut conn_in = try!(midi_in.connect(in_port, "microbrute", |_, message, state| {
            self.handle_incoming_midi_message(state, message)
        }, state).map_err(|e| e.kind()));

        try!(conn_out.send(&microbrute::start_communication_command()));
        sleep(Duration::from_millis(100));
        self.counter = 0;
        for cmd in microbrute::init_data() {
            try!(conn_out.send(&microbrute::init_command(self.counter, cmd)));
            sleep(Duration::from_millis(100));
            self.counter += 1;
        }
        let (midi_in_, state) = conn_in.close();
        midi_in = midi_in_;
        Ok(state)
    }

    pub fn set_state<'a, 'b>(&'a mut self, param: &'b str, value: &'b str, mut state: State<'b>) -> Result<State<'b>, Box<Error>> {
        let midi_out = try!(MidiOutput::new("Arturia Microbrute"));
        let out_port: u32 = MidiInterface::get_midi_out_port(&midi_out);
        let mut conn_out = try!(midi_out.connect(out_port, "microbrute").map_err(|e| e.kind()));
        try!(conn_out.send(&microbrute::set_command(self.counter, "NOTE_PRIORITY", "LAST")));
        sleep(Duration::from_millis(100));
        state.set(param, value);
        self.counter += 1;
        Ok(state)
    }

    fn handle_incoming_midi_message(&self, state: &mut State, message: &[u8]) {
        match self.is_sysex_message(message) {
            true => self.handle_sysex_message(state, message),
            false => (),
        }
    }

    fn is_sysex_message(&self, message: &[u8]) -> bool {
        let len: usize = message.len();
        if message[0] == 0xf0 && message[len-1] == 0xf7 {
            true
        }
        else {
            false
        }
    }

    fn is_sysex_state_response(&self, message: &[u8]) -> bool {
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

    fn handle_sysex_state_response(&self, state: &mut State, message: &[u8]) {
        match env::var("DEBUG_MESSAGES") {
            Ok(_) => println!("State response: {:02x}: {:02x}", message[8], message[9]),
            Err(_) => (),
        }
        microbrute::set_microbrute_state(state, message);
    }

    fn handle_sysex_message(&self, state: &mut State, message: &[u8]) {
        match env::var("DEBUG_MESSAGES") {
            Ok(_) => println!("{} (len = {})", utils::to_hex_string(&message), message.len()),
            Err(_) => (),
        }
        match self.is_sysex_state_response(message) {
            true => self.handle_sysex_state_response(state, message),
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

    fn get_midi_in_port(&self, midi_in: &MidiInput) -> u32 {
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
}
