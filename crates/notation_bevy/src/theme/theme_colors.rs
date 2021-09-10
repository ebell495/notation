use notation_model::prelude::{Chord, IntervalQuality, Octave, PlayingState, Semitones, Signature, Syllable};
use serde::{Deserialize, Serialize};

use bevy::prelude::*;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct PlayingColors {
    pub idle: Color,
    pub current: Color,
    pub played: Color,
}
impl PlayingColors {
    pub fn new(idle: Color, current: Color, played: Color) -> Self {
        Self {
            idle,
            current,
            played,
        }
    }
    pub fn of_state(&self, state: &PlayingState) -> Color {
        match state {
            PlayingState::Idle => self.idle,
            PlayingState::Current => self.current,
            PlayingState::Played => self.played,
        }
    }
}
impl Default for PlayingColors {
    fn default() -> Self {
        Self::new(
            color_of_hex("FFFFFF"),
            color_of_hex("FF00FF"),
            color_of_hex("555555"),
        )
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct IntervalColors {
    pub perfect: Color,
    pub major: Color,
    pub minor: Color,
    pub augmented: Color,
    pub diminishd: Color,
    pub tritone: Color,
}
impl IntervalColors {
    pub fn of_quality(&self, quality: &IntervalQuality) -> Color {
        match quality {
            IntervalQuality::Perfect => self.perfect,
            IntervalQuality::Major => self.major,
            IntervalQuality::Minor => self.minor,
            IntervalQuality::Augmented => self.augmented,
            IntervalQuality::Diminished => self.diminishd,
            IntervalQuality::Tritone => self.tritone,
        }
    }
}
impl Default for IntervalColors {
    fn default() -> Self {
        Self {
            perfect: color_of_hex("FFFFFF"),
            major: color_of_hex("FFFFFF"),
            minor: color_of_hex("333333"),
            augmented: color_of_hex("FF00FFAA"),
            diminishd: color_of_hex("33333333"),
            tritone: color_of_hex("FF00FFAA"),
        }
    }
}

pub fn color_of_hex(hex: &str) -> Color {
    Color::hex(hex).unwrap()
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ThemeColors {
    pub syllables: SyllableColors,
    pub bar: BarColors,
    pub chord: ChordColors,
    pub lyrics: LyricsColors,
    pub section: SectionColors,
    pub strings: StringsColors,
    pub rhythm: RhythmColors,
    pub mini_map: MiniMapColors,
}

impl ThemeColors {
    pub fn color_of_hex(hex: &str) -> Color {
        color_of_hex(hex)
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct SyllableColors {
    pub outline: PlayingColors,
    pub syllables: [Color; 12],
    pub no_syllable: Color,
}
//https://meyerweb.com/eric/tools/color-blend/
impl Default for SyllableColors {
    fn default() -> Self {
        Self {
            outline: PlayingColors::default(),
            syllables: [
                color_of_hex("E94F4F"), // Do
                color_of_hex("99572C"), // Di, Ra
                color_of_hex("FFEB34"), // Re
                color_of_hex("558C7F"), // Ri, Me
                color_of_hex("59D7FF"), // Mi
                color_of_hex("C31F6E"), // Fa
                color_of_hex("992D42"), // Fi, Se
                color_of_hex("FF8F28"), // So
                color_of_hex("A17C2B"), // Si, Le
                color_of_hex("A3DC5B"), // La
                color_of_hex("5F785A"), // Li, Te
                color_of_hex("7C87E8"), // Ti
            ],
            no_syllable: color_of_hex("888888"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct BarColors {
    pub bar_indicator: Color,
}
impl Default for BarColors {
    fn default() -> Self {
        Self {
            bar_indicator: color_of_hex("FF00FF"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct ChordColors {
    pub background: Color,
    pub diagram_outline: PlayingColors,
    pub dot: IntervalColors,
    pub dot_outline: IntervalColors,
}
impl Default for ChordColors {
    fn default() -> Self {
        Self {
            background: color_of_hex("FFF9F2CC"),
            diagram_outline: PlayingColors::default(),
            dot: IntervalColors::default(),
            dot_outline: IntervalColors {
                perfect: color_of_hex("000000"),
                major: color_of_hex("000000"),
                minor: color_of_hex("FFFFFF"),
                augmented: color_of_hex("FF00FF"),
                diminishd: color_of_hex("333333"),
                tritone: color_of_hex("FF00FF"),
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LyricsColors {
    pub line: PlayingColors,
    pub text: PlayingColors,
}
impl Default for LyricsColors {
    fn default() -> Self {
        Self {
            line: PlayingColors::new(
                color_of_hex("000000"),
                color_of_hex("FF00FF"),
                color_of_hex("555555"),
            ),
            text: PlayingColors::new(
                color_of_hex("000000"),
                color_of_hex("FF00FF"),
                color_of_hex("555555"),
            ),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct StringsColors {
    pub outline: PlayingColors,
    pub hit: Color,
    pub string: PlayingColors,
    pub fret: PlayingColors,
    pub capo: Color,
}
impl Default for StringsColors {
    fn default() -> Self {
        Self {
            outline: PlayingColors::default(),
            hit: color_of_hex("FF00FF"),
            string: PlayingColors::new(
                color_of_hex("D3B59C"),
                color_of_hex("FFFFFF"),
                color_of_hex("D3B59C"),
            ),
            fret: PlayingColors::new(
                color_of_hex("000000"),
                color_of_hex("000000"),
                color_of_hex("555555"),
            ),
            capo: color_of_hex("333333"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct SectionColors {
    pub sections: [Color; 6],
}
impl Default for SectionColors {
    fn default() -> Self {
        Self {
            sections: [
                color_of_hex("CC4444"),
                color_of_hex("44CC44"),
                color_of_hex("4444CC"),
                color_of_hex("CCCC44"),
                color_of_hex("44CCCC"),
                color_of_hex("CC44CC"),
            ],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct RhythmColors {
    pub beats: [Color; 3],
    pub indicator: Color,
}
impl Default for RhythmColors {
    fn default() -> Self {
        Self {
            beats: [
                color_of_hex("444444"),
                color_of_hex("222222"),
                color_of_hex("333333"),
            ],
            indicator: color_of_hex("555555"),
        }
    }
}

impl RhythmColors {
    pub fn get_beat_color(&self, signature: &Signature, beat: u8) -> Color {
        if beat == 0 {
            return self.beats[0];
        }
        if signature.bar_beats % 4 == 0 {
            match beat % 4 {
                1 => self.beats[1],
                2 => self.beats[2],
                3 => self.beats[1],
                _ => self.beats[0],
            }
        } else if signature.bar_beats % 3 == 0 {
            match beat % 3 {
                1 => self.beats[1],
                2 => self.beats[2],
                _ => self.beats[0],
            }
        } else {
            match beat % 2 {
                1 => self.beats[1],
                _ => self.beats[0],
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct MiniMapColors {
    pub back: Color,
    pub bar_outline: PlayingColors,
}
impl Default for MiniMapColors {
    fn default() -> Self {
        Self {
            //back: color_of_hex("FFF9F2"),
            back: color_of_hex("333333"),
            bar_outline: PlayingColors::default(),
        }
    }
}

impl SyllableColors {
    pub fn of_semitones(&self, v: Semitones) -> Color {
        let pos_val = if v.0 >= 0 { v.0 % 12 } else { v.0 % 12 + 12 } as usize;
        self.syllables[pos_val]
    }
    pub fn of_syllable(&self, v: Syllable) -> Color {
        self.of_semitones(Semitones::from(v))
    }
    pub fn of_syllable_octave(&self, v: Syllable, _o: Octave) -> Color {
        self.of_semitones(Semitones::from(v))
    }
    pub fn of_option_syllable(&self, v: Option<Syllable>) -> Color {
        if let Some(syllable) = v {
            self.of_syllable(syllable)
        } else {
            self.no_syllable
        }
    }
    pub fn of_option_chord(&self, v: Option<Chord>) -> Color {
        if let Some(chord) = v {
            self.of_syllable(chord.root)
        } else {
            self.no_syllable
        }
    }
}

impl SectionColors {
    pub fn of_section(&self, v: usize) -> Color {
        self.sections[v % self.sections.len()]
    }
}

impl ThemeColors {
    pub fn of_syllable(&self, v: Syllable) -> Color {
        self.syllables.of_syllable(v)
    }
    pub fn of_option_syllable(&self, v: Option<Syllable>) -> Color {
        self.syllables.of_option_syllable(v)
    }
    pub fn of_option_chord(&self, v: Option<Chord>) -> Color {
        self.syllables.of_option_chord(v)
    }
    pub fn of_section(&self, v: usize) -> Color {
        self.section.of_section(v)
    }
}
