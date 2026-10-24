use crate::conn::connect_out;
use midi_lib::output::MidiOutputConnectionEx;
use midir::MidiOutputConnection;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use wmidi::MidiMessage;

pub struct FM1Presets {
    fm1_out_conn: MidiOutputConnection,
}

impl FM1Presets {
    pub fn new() -> Result<Self, Box<dyn Error>>
    {
        let fm1_out_conn = connect_out()?;
        Ok(FM1Presets { fm1_out_conn })
    }

    pub fn upload_preset(&mut self, preset: u8, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let message = MidiMessage::from_bytes(&data)?;
        self.fm1_out_conn.send_midi_msg(&message)?;
        Ok(())
    }
}
