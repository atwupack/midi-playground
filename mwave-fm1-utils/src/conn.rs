use log::debug;
use midir::{MidiOutput, MidiOutputConnection};
use crate::error::{convert_error, FM1Error};

const FM1_NAME: &'static str = "FM-1";

pub(crate) fn connect_out() -> Result<MidiOutputConnection, FM1Error> {
    let midi_out = MidiOutput::new(FM1_NAME).map_err(convert_error)?;
    let fm1_out_conn = midi_out.ports()
        .iter()
        .find(|port| midi_out.port_name(port).unwrap().starts_with(FM1_NAME))
        .map(|port| {
            return midi_out.connect(port, FM1_NAME).unwrap();
        }).ok_or(FM1Error("".to_string()))?;
    debug!("FM-1 MIDI output connected");

    Ok(fm1_out_conn)
}
