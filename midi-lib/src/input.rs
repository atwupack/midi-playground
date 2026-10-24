use midir::{ConnectError, MidiInput, MidiInputConnection, MidiInputPort};
use wmidi::MidiMessage;

pub trait MidiInputEx {
    fn connect_midi<F, T: Send>(
        self,
        port: &MidiInputPort,
        port_name: &str,
        callback: F,
        data: T,
    ) -> Result<MidiInputConnection<T>, ConnectError<MidiInput>>
    where
        F: FnMut(u64, &MidiMessage, &mut T) + Send + 'static;
}

impl MidiInputEx for MidiInput {
    fn connect_midi<F, T: Send>(self, port: &MidiInputPort, port_name: &str, callback: F, data: T) -> Result<MidiInputConnection<T>, ConnectError<MidiInput>>
    where
        F: FnMut(u64, &MidiMessage, &mut T) + Send + 'static
    {
        connect_midi_input(self, port, port_name, callback, data)
    }
}

pub fn connect_midi_input<F, T: Send>(input: MidiInput, port: &MidiInputPort, port_name: &str, mut callback: F, data: T) -> Result<MidiInputConnection<T>, ConnectError<MidiInput>>
where
    F: FnMut(u64, &MidiMessage, &mut T) + Send + 'static
{
    input.connect(port, port_name, move |time, msg, data| {
        let midi_msg = MidiMessage::from_bytes(msg).unwrap();
        callback(time, &midi_msg, data);
    }, data)
}