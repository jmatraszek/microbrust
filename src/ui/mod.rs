pub mod helpers;
use rustbox;
use rustbox::{Color, Key, RustBox};
use interface::Interface;
use state::State;

const BEND_RANGES: &'static [&'static str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12"];
const MIDI_SEND_CHANNELS: &'static [&'static str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16"];
const MIDI_RECV_CHANNELS: &'static [&'static str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "All"];

pub fn print_not_connected(rustbox: &RustBox) {
    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Microbrute not connected.");
    rustbox.print(1, 2, rustbox::RB_NORMAL, Color::White, Color::Black, "Quit with Escape or connect and press any other key.");
}

pub fn print_state(rustbox: &RustBox, state: &State) {
    rustbox.clear();

    rustbox.print(1, 0, rustbox::RB_BOLD, Color::White, Color::Black, helpers::name().as_ref());
    rustbox.print(1, 2, rustbox::RB_BOLD, Color::White, Color::Black, helpers::title());
    rustbox.print(1, 4, rustbox::RB_BOLD, Color::White, Color::Black, "Keyboard parameters");
    rustbox.print(1, 5, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::note_priority(&state).as_ref());
    rustbox.print(1, 6, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::velocity_response(&state).as_ref());

    rustbox.print(40, 4, rustbox::RB_BOLD, Color::White, Color::Black, "Sequence control");
    rustbox.print(40, 5, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::play(&state).as_ref());
    rustbox.print(40, 6, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::seq_retrig(&state).as_ref());
    rustbox.print(40, 7, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::next_seq(&state).as_ref());
    rustbox.print(40, 8, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::step_on(&state).as_ref());
    rustbox.print(40, 9, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::step(&state).as_ref());

    rustbox.print(1, 11, rustbox::RB_BOLD, Color::White, Color::Black, "MIDI channel select");
    rustbox.print(1, 12, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::midi_recv_chan(&state).as_ref());
    rustbox.print(1, 13, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::midi_send_chan(&state).as_ref());

    rustbox.print(40, 11, rustbox::RB_BOLD, Color::White, Color::Black, "Module parameters");
    rustbox.print(40, 12, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::lfo_key_retrig(&state).as_ref());
    rustbox.print(40, 13, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::env_legato_mode(&state).as_ref());
    rustbox.print(40, 14, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::bend_range(&state).as_ref());
    rustbox.print(40, 15, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::gate(&state).as_ref());
    rustbox.print(40, 16, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::sync(&state).as_ref());
}

pub fn change_note_priority<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Note priority", "1 - Low, 2 - Last, 3 - High.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "note_priority", "Low").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "note_priority", "Last").unwrap(); },
                Key::Char('3') => { midi_interface.set_state(state, "note_priority", "High").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_velocity_response<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Velocity response", "1 - /, 2 - (, 3 - ).");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "velocity_response", "/").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "velocity_response", "(").unwrap(); },
                Key::Char('3') => { midi_interface.set_state(state, "velocity_response", ")").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_play<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Play", "1 - Note On, 2 - Hold.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "play", "Note on").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "play", "Hold").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_seq_retrig<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Seq retrig", "1 - Reset, 2 - Legato, 3 - None.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "seq_retrig", "Reset").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "seq_retrig", "Legato").unwrap(); },
                Key::Char('3') => { midi_interface.set_state(state, "seq_retrig", "None").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_next_seq<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Next seq", "1 - End, 2 - Instant reset, 3 - Instant continue.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "next_seq", "End").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "next_seq", "Instant reset").unwrap(); },
                Key::Char('3') => { midi_interface.set_state(state, "next_seq", "Instant continue").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_step_on<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Step on", "1 - Clk, 2 - Gate.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "step_on", "Clk").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "step_on", "Gate").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_step<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Step", "1 - 1/4, 2 - 1/8, 3 - 1/16, 4 - 1/32.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "step", "1/4").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "step", "1/8").unwrap(); },
                Key::Char('3') => { midi_interface.set_state(state, "step", "1/16").unwrap(); },
                Key::Char('4') => { midi_interface.set_state(state, "step", "1/32").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_lfo_key_retrig<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "LFO key retrig", "1 - On, 2 - Off.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "lfo_key_retrig", "On").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "lfo_key_retrig", "Off").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_env_legato_mode<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Envelope legato mode", "1 - On, 2 - Off.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "env_legato_mode", "On").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "env_legato_mode", "Off").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_gate<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Gate", "1 - Short, 2 - Medium, 3 - Long.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "gate", "Short").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "gate", "Medium").unwrap(); },
                Key::Char('3') => { midi_interface.set_state(state, "gate", "Long").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_sync<'a>(rustbox: &RustBox, midi_interface: &'a mut Interface, state: &'a mut State) {
    helpers::print_select_value_message(rustbox, "Sync", "1 - External, 2 - Interrnal, 3 - Auto.");

    match rustbox.poll_event(false) {
        Ok(rustbox::Event::KeyEvent(key)) => {
            match key {
                Key::Char('1') => { midi_interface.set_state(state, "sync", "External").unwrap(); },
                Key::Char('2') => { midi_interface.set_state(state, "sync", "Internal").unwrap(); },
                Key::Char('3') => { midi_interface.set_state(state, "sync", "Auto").unwrap(); },
                _ => { }
            }
        },
        Err(_) => {  },
        _ => { }
    }
}

pub fn change_bend_range(rustbox: &RustBox, midi_interface: &mut Interface, state: &mut State) {
    helpers::print_select_value_message(rustbox, "Bend range", "Select values using up and down arrows and press Enter to confirm.");
    let mut index = BEND_RANGES.iter().position(|&r| r == state.bend_range).unwrap();
    rustbox.print(1, 20, rustbox::RB_NORMAL, Color::White, Color::Black, BEND_RANGES[index as usize]);
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Up => {
                        index = index + 1;
                        if index == 12 { index = 0; }
                        print_state(&rustbox, &state);
                        helpers::print_select_value_message(rustbox, "Bend range", "Select values using up and down arrows and press Enter to confirm.");
                    },
                    Key::Down => {
                        if index == 0 { index = 12; }
                        index = index - 1;
                        print_state(&rustbox, &state);
                        helpers::print_select_value_message(rustbox, "Bend range", "Select values using up and down arrows and press Enter to confirm.");
                    },
                    Key::Enter => {
                        midi_interface.set_state(state, "bend_range", BEND_RANGES[index as usize]).unwrap();
                        break;
                    },
                    Key::Esc => {
                        break;
                    },
                    _ => { }
                }
                rustbox.print(1, 20, rustbox::RB_NORMAL, Color::White, Color::Black, BEND_RANGES[index as usize]);
                rustbox.present();
            },
            Err(_) => {  },
            _ => { }
        }
    }
}

pub fn change_midi_recv_chan(rustbox: &RustBox, midi_interface: &mut Interface, state: &mut State) {
    helpers::print_select_value_message(rustbox, "MIDI receive channel", "Select values using up and down arrows and press Enter to confirm.");
    let mut index = MIDI_RECV_CHANNELS.iter().position(|&r| r == state.midi_recv_chan).unwrap();
    rustbox.print(1, 20, rustbox::RB_NORMAL, Color::White, Color::Black, MIDI_RECV_CHANNELS[index as usize]);
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Up => {
                        index = index + 1;
                        if index == 17 { index = 0; }
                        print_state(&rustbox, &state);
                        helpers::print_select_value_message(rustbox, "MIDI receive channel", "Select values using up and down arrows and press Enter to confirm.");
                    },
                    Key::Down => {
                        if index == 0 { index = 17; }
                        index = index - 1;
                        print_state(&rustbox, &state);
                        helpers::print_select_value_message(rustbox, "MIDI receive channel", "Select values using up and down arrows and press Enter to confirm.");
                    },
                    Key::Enter => {
                        midi_interface.set_state(state, "midi_recv_chan", MIDI_RECV_CHANNELS[index as usize]).unwrap();
                        break;
                    },
                    Key::Esc => {
                        break;
                    },
                    _ => { }
                }
                rustbox.print(1, 20, rustbox::RB_NORMAL, Color::White, Color::Black, MIDI_RECV_CHANNELS[index as usize]);
                rustbox.present();
            },
            Err(_) => {  },
            _ => { }
        }
    }
}

pub fn change_midi_send_chan(rustbox: &RustBox, midi_interface: &mut Interface, state: &mut State) {
    helpers::print_select_value_message(rustbox, "MIDI send channel", "Select values using up and down arrows and press Enter to confirm.");
    let mut index = MIDI_SEND_CHANNELS.iter().position(|&r| r == state.midi_send_chan).unwrap();
    rustbox.print(1, 20, rustbox::RB_NORMAL, Color::White, Color::Black, MIDI_SEND_CHANNELS[index as usize]);
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Up => {
                        index = index + 1;
                        if index == 16 { index = 0; }
                        print_state(&rustbox, &state);
                        helpers::print_select_value_message(rustbox, "MIDI send channel", "Select values using up and down arrows and press Enter to confirm.");
                    },
                    Key::Down => {
                        if index == 0 { index = 16; }
                        index = index - 1;
                        print_state(&rustbox, &state);
                        helpers::print_select_value_message(rustbox, "MIDI send channel", "Select values using up and down arrows and press Enter to confirm.");
                    },
                    Key::Enter => {
                        midi_interface.set_state(state, "midi_send_chan", MIDI_SEND_CHANNELS[index]).unwrap();
                        break;
                    },
                    Key::Esc => {
                        break;
                    },
                    _ => { }
                }
                rustbox.print(1, 20, rustbox::RB_NORMAL, Color::White, Color::Black, MIDI_SEND_CHANNELS[index]);
                rustbox.present();
            },
            Err(_) => {  },
            _ => { }
        }
    }
}
