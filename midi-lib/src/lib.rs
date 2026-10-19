use log::debug;
use midir::{MidiOutput, MidiOutputConnection, SendError};
use nusb::{MaybeFuture, list_devices};
use wmidi::MidiMessage;

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
    }
    Ok(())
}

pub fn send_midi_message(midi_out: &mut MidiOutputConnection, message: &MidiMessage) -> Result<(), SendError> {
    midi_out.send(&message.to_vec())?;
    debug!("Sent message: {:?}", message);
    Ok(())
}