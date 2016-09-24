pub mod helpers;
use rustbox;
use rustbox::{Color, RustBox};
use state::State;

pub fn print_state(rustbox: &RustBox, state: &State) {
    rustbox.clear();

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, helpers::title());
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black, "Keyboard parameters");
    rustbox.print(1, 4, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::note_priority(&state).as_ref());
    rustbox.print(1, 5, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::velocity_response(&state).as_ref());

    rustbox.print(40, 3, rustbox::RB_BOLD, Color::White, Color::Black, "Sequence control");
    rustbox.print(40, 4, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::play(&state).as_ref());
    rustbox.print(40, 5, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::seq_retrig(&state).as_ref());
    rustbox.print(40, 6, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::next_seq(&state).as_ref());
    rustbox.print(40, 7, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::step_on(&state).as_ref());
    rustbox.print(40, 8, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::step(&state).as_ref());

    rustbox.print(1, 10, rustbox::RB_BOLD, Color::White, Color::Black, "MIDI channel select");
    rustbox.print(1, 11, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::midi_recv_chan(&state).as_ref());
    rustbox.print(1, 12, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::midi_send_chan(&state).as_ref());

    rustbox.print(40, 10, rustbox::RB_BOLD, Color::White, Color::Black, "Module parameters");
    rustbox.print(40, 11, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::lfo_key_retrig(&state).as_ref());
    rustbox.print(40, 12, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::env_legato_mode(&state).as_ref());
    rustbox.print(40, 13, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::bend_range(&state).as_ref());
    rustbox.print(40, 14, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::gate(&state).as_ref());
    rustbox.print(40, 15, rustbox::RB_NORMAL, Color::White, Color::Black, helpers::sync(&state).as_ref());

    rustbox.present();
}
