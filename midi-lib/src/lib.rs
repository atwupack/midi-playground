pub mod input;
pub mod output;

use midir::MidiOutput;
use nusb::{MaybeFuture, list_devices};

pub fn find_midi_devices() -> Result<(), Box<dyn std::error::Error>> {
    let devices = list_devices().wait()?;
    for device in devices {

        println!("{:?}", device);
    }
    Ok(())
}

pub fn find_midi_out_ports() -> Result<(), Box<dyn std::error::Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;
    let out_ports = midi_out.ports();
    for port in out_ports {
        println!("{:?}", midi_out.port_name(&port));
        println!("{:?}", port.id());
    }
    Ok(())
}

