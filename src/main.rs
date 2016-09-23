extern crate rustbox;

mod utils;
mod state;
mod interface;
mod ui;

use std::error::Error;
use std::default::Default;
use state::State;
use interface::Interface;

use rustbox::{Color, RustBox};
use rustbox::Key;

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

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::title());
    rustbox.print(1, 2, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::note_priority(&state).as_ref());
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::velocity_response(&state).as_ref());
    rustbox.print(1, 4, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::play(&state).as_ref());
    rustbox.print(1, 5, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::next_seq(&state).as_ref());
    rustbox.print(1, 6, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::seq_retrig(&state).as_ref());
    rustbox.print(1, 7, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::step(&state).as_ref());
    rustbox.print(1, 8, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::step_on(&state).as_ref());
    rustbox.print(1, 9, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::lfo_key_retrig(&state).as_ref());
    rustbox.print(1, 10, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::env_legato_mode(&state).as_ref());
    rustbox.print(1, 11, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::gate(&state).as_ref());
    rustbox.print(1, 12, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::sync(&state).as_ref());
    rustbox.print(1, 13, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::bend_range(&state).as_ref());
    rustbox.print(1, 14, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::midi_recv_chan(&state).as_ref());
    rustbox.print(1, 15, rustbox::RB_BOLD, Color::White, Color::Black, ui::helpers::midi_send_chan(&state).as_ref());
    rustbox.print(1, 16, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }

    // state.print();
    // state = midi_interface.set_state("note_priority", "Last", state).unwrap();
    // state = midi_interface.set_state("play", "Note on", state).unwrap();
    // state.print();
    Ok(())
}

