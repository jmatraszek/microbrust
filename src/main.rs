extern crate rustbox;

mod utils;
mod state;
mod interface;
mod ui;

use std::error::Error;
use std::default::Default;
use state::State;
use interface::Interface;

use rustbox::{Key, RustBox};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut state = State::new();
    let mut midi_interface = Interface::new();
    state = midi_interface.read_state(state).unwrap();

    // state = midi_interface.set_state(state, "note_priority", "Last").unwrap();
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    ui::print_state(&rustbox, &state);
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc => { break; } // Quit
                    Key::Char('n') => {
                        ui::change_note_priority(&rustbox, &mut midi_interface, &mut state);
                    },
                    Key::Char('v') => {

                    },
                    Key::Char('p') => {

                    },
                    Key::Char('x') => {

                    },
                    Key::Char('q') => {

                    },
                    Key::Char('t') => {

                    },
                    Key::Char('o') => {

                    },
                    Key::Char('l') => {

                    },
                    Key::Char('e') => {

                    },
                    Key::Char('g') => {

                    },
                    Key::Char('y') => {

                    },
                    Key::Char('b') => {

                    },
                    Key::Char('r') => {

                    },
                    Key::Char('s') => {

                    },
                    _ => { }
                }
                ui::print_state(&rustbox, &state);
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }

    // state.print();
    // state = midi_interface.set_state(state, "note_priority", "Last").unwrap();
    // state.print();
    Ok(())
}

