extern crate midir;
mod microbrute;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, MidiOutput, Ignore};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let midi_out = try!(MidiOutput::new("Arturia Microbrute"));
    let out_port: u32 = get_midi_out_port(&midi_out);
    println!("\nOpening connections on port {}", out_port);
    let mut conn_out = try!(midi_out.connect(out_port, "midir-test").map_err(|e| e.kind()));
    try!(conn_out.send(&microbrute::start_communication_command()));
    sleep(Duration::from_millis(200));
    let mut counter: u8 = 0;
    for cmd in microbrute::init_data() {
        try!(conn_out.send(&microbrute::init_command(counter, cmd)));
        sleep(Duration::from_millis(200));
        counter += 1;
    }
    // try!(conn_out.send(&microbrute::set_command(counter, "NOTE_PRIORITY", "HIGH")));
    // try!(conn_out.send(&microbrute::set_command(1, "VELOCITY_RESPONSE", "/")));
    Ok(())
}

fn get_midi_out_port(midi_out: &MidiOutput) -> u32 {
    let mut out_port: Option<u32> = None;

    for i in 0..midi_out.port_count() {
        println!("{}: {}", i, midi_out.port_name(i).unwrap());
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
