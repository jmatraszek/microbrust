use state::State;

pub fn title<'a>() -> &'a str {
    "ARTURIA MICROBRUTE"
}

pub fn note_priority<'a>(state: &'a State) -> String {
   return format!("{}{}", "[n] Note priority: ", state.note_priority);
}

pub fn velocity_response<'a>(state: &'a State) -> String {
   return format!("{}{}", "[v] Velocity response: ", state.velocity_response);
}

pub fn play<'a>(state: &'a State) -> String {
   return format!("{}{}", "[p] Play: ", state.play);
}

pub fn next_seq<'a>(state: &'a State) -> String {
   return format!("{}{}", "[x] Next sequence: ", state.next_seq);
}

pub fn seq_retrig<'a>(state: &'a State) -> String {
   return format!("{}{}", "[q] Sequence retrig: ", state.seq_retrig);
}

pub fn step<'a>(state: &'a State) -> String {
   return format!("{}{}", "[t] Step: ", state.step);
}

pub fn step_on<'a>(state: &'a State) -> String {
   return format!("{}{}", "[o] Step on: ", state.step_on);
}

pub fn lfo_key_retrig<'a>(state: &'a State) -> String {
   return format!("{}{}", "[l] LFO key retrig: ", state.lfo_key_retrig);
}

pub fn env_legato_mode<'a>(state: &'a State) -> String {
   return format!("{}{}", "[e] Env legato mode: ", state.env_legato_mode);
}

pub fn gate<'a>(state: &'a State) -> String {
   return format!("{}{}", "[g] Gate: ", state.gate);
}

pub fn sync<'a>(state: &'a State) -> String {
   return format!("{}{}", "[y] Sync: ", state.sync);
}

pub fn bend_range<'a>(state: &'a State) -> String {
   return format!("{}{}", "[b] Bend range: ", state.bend_range);
}

pub fn midi_recv_chan<'a>(state: &'a State) -> String {
   return format!("{}{}", "[r] MIDI receive channel: ", state.midi_recv_chan);
}

pub fn midi_send_chan<'a>(state: &'a State) -> String {
   return format!("{}{}", "[s] MIDI send channel: ", state.midi_send_chan);
}
