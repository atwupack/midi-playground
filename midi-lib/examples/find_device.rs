use midi_lib::{find_midi_devices, find_midi_out_ports};

fn main() {

    find_midi_devices().unwrap();
    find_midi_out_ports().unwrap();
}