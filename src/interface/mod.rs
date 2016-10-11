extern crate midir;

mod response_handler;
mod request_handler;
mod error;

use self::midir::{MidiInput, MidiOutput};
use std::error::Error;
use std::result::Result;
use std::thread::sleep;
use std::time::Duration;
use state::State;
use interface::error::NotConnected;

pub struct Interface {
    pub counter: u8
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            counter: 0
        }
    }

    pub fn read_state<'a, 'b>(&'a mut self, state: State<'static>) -> Result<State<'b>, Box<Error>> {
        let midi_out = try!(MidiOutput::new("Arturia Microbrute"));
        let out_port: u32 = try!(Interface::get_midi_out_port(&midi_out));
        let mut conn_out = try!(midi_out.connect(out_port, "microbrute").map_err(|e| e.kind()));

        let midi_in = try!(MidiInput::new("Arturia Microbrute"));
        let in_port: u32 = try!(Interface::get_midi_in_port(&midi_in));
        let conn_in = try!(midi_in.connect(in_port, "microbrute", |_, message, state| {
            response_handler::handle_incoming_midi_message(state, message)
        }, state).map_err(|e| e.kind()));

        try!(conn_out.send(&request_handler::start_communication_command()));
        sleep(Duration::from_millis(100));
        self.counter = 0;
        for cmd in request_handler::init_data() {
            try!(conn_out.send(&request_handler::init_command(self.counter, cmd)));
            sleep(Duration::from_millis(100));
            self.counter += 1;
        }
        let (_, state) = conn_in.close();
        Ok(state)
    }

    pub fn set_state<'a, 'b>(&'a mut self, state: &'a mut State<'b>, param: &'b str, value: &'b str) -> Result<&State<'b>, Box<Error>> {
        let midi_out = try!(MidiOutput::new("Arturia Microbrute"));
        let out_port: u32 = try!(Interface::get_midi_out_port(&midi_out));
        let mut conn_out = try!(midi_out.connect(out_port, "microbrute").map_err(|e| e.kind()));
        try!(conn_out.send(&request_handler::set_command(self.counter, param, value)));
        sleep(Duration::from_millis(100));
        state.set(param, value);
        self.counter += 1;
        Ok(state)
    }

    fn get_midi_out_port(midi_out: &MidiOutput) -> Result<u32, NotConnected> {
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
            Some(port_num) => Ok(port_num),
            None => Err(NotConnected),
        }
    }

    fn get_midi_in_port(midi_in: &MidiInput) -> Result<u32, NotConnected> {
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
            Some(port_num) => Ok(port_num),
            None => Err(NotConnected),
        }
    }
}

