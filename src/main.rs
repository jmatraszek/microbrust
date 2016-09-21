mod utils;
mod microbrute;
mod state;
mod midi_interface;

use std::error::Error;
use state::State;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut state = state::State::new();
    let mut midi_interface = midi_interface::MidiInterface::new();
    state = try!(midi_interface.update_state(state));

    microbrute::print_state(&state);
    // try!(conn_out.send(&microbrute::set_command(counter, "NOTE_PRIORITY", "LAST")));
    // sleep(Duration::from_millis(100));
    // try!(conn_out.send(&microbrute::set_command(counter, "MIDI_RECV_CHAN", "9")));
    // try!(conn_out.send(&microbrute::set_command(1, "VELOCITY_RESPONSE", "/")));
    // microbrute::print_state(&state);

    Ok(())
}

