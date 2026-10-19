pub mod fx;
pub mod presets;
mod conn;
mod error;

use crate::fx::{Effect, EffectParameter};
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;
use log::debug;
use wmidi::Channel::{Ch1, Ch2};
use wmidi::MidiMessage::{ControlChange, NoteOff, NoteOn, ProgramChange};
use wmidi::{MidiMessage, Note, ProgramNumber, U7, Velocity};
use midi_lib::send_midi_message;
use crate::conn::connect_out;
use crate::error::{convert_error, FM1Error};

pub struct FM1Player {
    fm1_out_conn: MidiOutputConnection,
    fm1_in_conn: MidiInputConnection<()>,
}

impl FM1Player {



    pub fn new() -> Result<Self, Box<dyn Error>>
    {
        let fm1_out_conn = connect_out()?;

        let midi_in = MidiInput::new("My Test Input")?;
        let in_ports = midi_in.ports();
        let fm1_in_conn = in_ports
            .iter()
            .find(|port| midi_in.port_name(port).unwrap().starts_with("FM-1"))
            .map(|port| {
                return midi_in.connect(port, "FM-1", |time, data, _| {
                    let msg = MidiMessage::from_bytes(data);
                    println!("Received MIDI message: {:?} at time: {:?}", msg, time);
                }, ()).unwrap();
            })
            .ok_or("")?;
        debug!("FM-1 MIDI input connected");

        Ok(FM1Player { fm1_out_conn, fm1_in_conn })
    }


    pub fn change_voice(&mut self, voice: u8) -> Result<(), FM1Error> {
        self.send_message(ProgramChange(Ch1, ProgramNumber::from_u8_lossy(voice)))
    }

    pub fn start_note(&mut self, note: Note, velocity: Velocity) -> Result<(), FM1Error> {
        self.send_message(NoteOn(Ch1, note, velocity))
    }

    pub fn stop_note(&mut self, note: Note, velocity: Velocity) -> Result<(), FM1Error> {
        self.send_message(NoteOff(Ch1, note, velocity))
    }

    pub fn play_note(
        &mut self,
        note: Note,
        velocity: Velocity,
        duration: u64,
    ) -> Result<(), FM1Error> {
        self.start_note(note, velocity)
            .and_then(|_| {
                sleep(Duration::from_millis(duration * 150));
                Ok(())
            })
            .and_then(|_| self.stop_note(note, velocity))
    }

    pub fn enable_effect(&mut self, effect: Effect) -> Result<(), FM1Error> {
        self.send_message(ControlChange(Ch2, effect.0, U7::from_u8_lossy(1)))
    }

    pub fn disable_effect(&mut self, effect: Effect) -> Result<(), FM1Error> {
        self.send_message(ControlChange(Ch2, effect.0, U7::from_u8_lossy(0)))
    }

    pub fn set_effect_parameter(
        &mut self,
        parameter: impl EffectParameter,
    ) -> Result<(), FM1Error> {
        self.send_message(ControlChange(
            Ch2,
            parameter.control_function(),
            parameter.control_value(),
        ))
    }

    fn send_message(&mut self, message: MidiMessage) -> Result<(), FM1Error> {
        send_midi_message(&mut self.fm1_out_conn, &message).map_err(convert_error)?;
        Ok(())
    }
}

