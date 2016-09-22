mod utils;
mod microbrute;
mod state;
mod interface;

use std::error::Error;
use state::State;
use interface::Interface;

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

    state.print();
    state = midi_interface.set_state("note_priority", "Last", state).unwrap();
    state = midi_interface.set_state("play", "Note on", state).unwrap();
    state.print();
    Ok(())
}

