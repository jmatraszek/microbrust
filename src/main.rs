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
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    loop {
        match run(&rustbox) {
            Ok(_) => { break },
            Err(_) => {
                ui::print_not_connected(&rustbox);
                rustbox.present();
                match rustbox.poll_event(false) {
                    Ok(rustbox::Event::KeyEvent(key)) => {
                        match key {
                            Key::Esc => { break; } // Quit
                            _ => { continue; }
                        }
                    },
                    Err(e) => panic!("{}", e.description()),
                    _ => { }
                }
            }
        }
    }
}

fn run(rustbox: &RustBox) -> Result<(), Box<Error>> {
    let mut state = State::new();
    let mut midi_interface = Interface::new();
    state = try!(midi_interface.read_state(state));

    ui::print_state(&rustbox, &state);
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc => { break; } // Quit
                    Key::Char('n') => { try!(ui::change_note_priority(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('v') => { try!(ui::change_velocity_response(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('p') => { try!(ui::change_play(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('q') => { try!(ui::change_seq_retrig(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('x') => { try!(ui::change_next_seq(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('o') => { try!(ui::change_step_on(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('t') => { try!(ui::change_step(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('l') => { try!(ui::change_lfo_key_retrig(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('e') => { try!(ui::change_env_legato_mode(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('g') => { try!(ui::change_gate(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('y') => { try!(ui::change_sync(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('b') => { try!(ui::change_bend_range(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('r') => { try!(ui::change_midi_recv_chan(&rustbox, &mut midi_interface, &mut state)); },
                    Key::Char('s') => { try!(ui::change_midi_send_chan(&rustbox, &mut midi_interface, &mut state)); },
                    _ => { }
                }
                ui::print_state(&rustbox, &state);
                rustbox.present();
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
    Ok(())
}

