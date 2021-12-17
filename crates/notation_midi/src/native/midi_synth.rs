use std::path::PathBuf;

use helgoboss_midi::StructuredShortMessage;

use crate::prelude::{MidiMessage, MidiSettings, MidiState};
use notation_model::prelude::PlaySpeed;

use super::audio_stream::DoubleAudioBuffer;

pub struct MidiSynth {
    synth: fluidlite::Synth,
    buffer: DoubleAudioBuffer,
}
impl MidiSynth {
    pub const SOUND_FONT: &'static str = "sblive";

    #[cfg(target_os = "windows")]
    pub const VOLUME_FACTOR: f32 = 5.0; //Not sure why the volume is really low on windows.

    #[cfg(not(target_os = "windows"))]
    pub const VOLUME_FACTOR: f32 = 1.5;

    fn new(synth: fluidlite::Synth) -> Self {
        Self {
            synth,
            buffer: DoubleAudioBuffer::new(),
        }
    }
    fn check_path(root: PathBuf, name: &str) -> Option<PathBuf> {
        let mut path = root.clone();
        path.push("assets");
        path.push(name);
        path.set_extension("sf2");
        if path.exists() {
            Some(path)
        } else {
            println!("MidiSynth check_path() not exist: {:?} {} -> {:?}", root, name, path);
            None
        }
    }
    pub fn try_new() -> Option<MidiSynth> {
        fluidlite::Settings::new()
            .and_then(fluidlite::Synth::new)
            .and_then(|synth| {
                let mut path = None;
                if let Ok(root) = std::env::current_exe() {
                    if let Some(root) = root.parent() {
                        path = Self::check_path(root.to_path_buf(), Self::SOUND_FONT);
                    }
                }
                if path.is_none() {
                    if let Ok(root) = std::env::current_dir() {
                        path = Self::check_path(root, Self::SOUND_FONT);
                    }
                }
                match path {
                    Some(path) => {
                        println!("MidiSynth try_new() Loading: {:?}", path);
                        synth.sfload(path, true).map(|_| synth)
                    },
                    None => {
                        let path = format!("assets/{}.sf2", Self::SOUND_FONT);
                        synth.sfload(path, true).map(|_| synth)
                    }
                }
            })
            .map(Self::new)
            .map_err(|err| {
                println!("MidiSynth try_new() failed: {:?}", err);
                err
            })
            .ok()
    }
    pub fn get_buffer(&self) -> Option<DoubleAudioBuffer> {
        Some(self.buffer.clone())
    }
    pub fn check_buffer(&mut self) {
        let synth = &self.synth;
        // let use_buffer_2 = self.buffer.use_buffer_2;
        self.buffer.write_buffer(|data| {
            /*
            println!("NativeMidiSynth writing buffer: {} [{}]",
                if use_buffer_2 { 2 } else { 1 },
                data.len());
             */
            synth.write(data).unwrap();
        });
    }
    pub fn init_channels(&self, _settings: &MidiSettings, _state: &MidiState) {
    }
    pub fn send(&self, _speed: &PlaySpeed, msg: &MidiMessage, velocity: u8) -> Result<(), String> {
        match msg.midi {
            StructuredShortMessage::NoteOff {
                channel,
                key_number,
                velocity: _,
            } => self.synth.note_off(channel.into(), key_number.into()),
            StructuredShortMessage::NoteOn {
                channel,
                key_number,
                velocity: _,
            } => {
                let velocity = if velocity > 127 { 127 } else { velocity };
                self
                .synth
                .note_on(channel.into(), key_number.into(), velocity.into())
            },
            StructuredShortMessage::PolyphonicKeyPressure {
                channel: _,
                key_number: _,
                pressure_amount: _,
            } => todo!(),
            StructuredShortMessage::ControlChange {
                channel,
                controller_number,
                control_value,
            } => self.synth.cc(
                channel.into(),
                controller_number.into(),
                control_value.into(),
            ),
            StructuredShortMessage::ProgramChange {
                channel,
                program_number,
            } => self
                .synth
                .program_change(channel.into(), program_number.into()),
            StructuredShortMessage::ChannelPressure {
                channel: _,
                pressure_amount: _,
            } => todo!(),
            StructuredShortMessage::PitchBendChange {
                channel: _,
                pitch_bend_value: _,
            } => todo!(),
            StructuredShortMessage::SystemExclusiveStart => todo!(),
            StructuredShortMessage::TimeCodeQuarterFrame(_) => todo!(),
            StructuredShortMessage::SongPositionPointer { position: _ } => todo!(),
            StructuredShortMessage::SongSelect { song_number: _ } => todo!(),
            StructuredShortMessage::TuneRequest => todo!(),
            StructuredShortMessage::SystemExclusiveEnd => todo!(),
            StructuredShortMessage::TimingClock => todo!(),
            StructuredShortMessage::Start => todo!(),
            StructuredShortMessage::Continue => todo!(),
            StructuredShortMessage::Stop => todo!(),
            StructuredShortMessage::ActiveSensing => todo!(),
            StructuredShortMessage::SystemReset => todo!(),
            StructuredShortMessage::SystemCommonUndefined1 => todo!(),
            StructuredShortMessage::SystemCommonUndefined2 => todo!(),
            StructuredShortMessage::SystemRealTimeUndefined1 => todo!(),
            StructuredShortMessage::SystemRealTimeUndefined2 => todo!(),
        }
        .map_err(|err| format!("{:?}", err))
    }
}
