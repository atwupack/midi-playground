use log::debug;
use midir::{MidiOutputConnection, SendError};
use wmidi::MidiMessage;

pub trait MidiOutputConnectionEx {
    fn send_midi_msg(&mut self, message: &MidiMessage) -> Result<(), SendError>;
}

impl MidiOutputConnectionEx for MidiOutputConnection {
    fn send_midi_msg(&mut self, message: &MidiMessage) -> Result<(), SendError> {
        send_midi_msg(self, message)
    }
}

pub fn send_midi_msg(conn: &mut MidiOutputConnection, message: &MidiMessage) -> Result<(), SendError> {
    conn.send(&message.to_vec())?;
    debug!("Sent message: {:?}", message);
    Ok(())
}