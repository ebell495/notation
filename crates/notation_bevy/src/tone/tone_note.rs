use std::sync::Arc;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use notation_model::prelude::{BarPosition, Duration, Note, Semitones, Syllable, Units};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};
use notation_model::prelude::TabBar;

use super::tone_mode::ToneMode;

#[derive(Clone, Debug)]
pub struct ToneNoteData {
    pub bar_units: Units,
    pub bar_ordinal: usize,
    pub duration: Duration,
    pub position: BarPosition,
    pub note: Note,
    pub mode: ToneMode,
}

impl ToneNoteData {
    pub fn new(
        bar_units: Units,
        tab_bar: &Arc<TabBar>,
        duration: Duration,
        position: BarPosition,
        note: Note,
        mode: ToneMode,
    ) -> Self {
        let bar_ordinal = tab_bar.bar_ordinal;
        ToneNoteData {
            bar_units,
            bar_ordinal,
            duration,
            position,
            note,
            mode,
        }
    }
    pub fn syllable(&self) -> Syllable {
        self.note.syllable.unwrap_or_else(|| Semitones::from(self.note.pitch).into())
    }
}
pub struct ToneNoteShape<'a> {
    theme: &'a NotationTheme,
    data: ToneNoteData,
}

impl<'a> LyonShape<shapes::Rectangle> for ToneNoteShape<'a> {
    fn get_name(&self) -> String {
        format!(
            "{}:{}",
            self.data.bar_ordinal, self.data.note
        )
    }
    fn get_shape(&self) -> shapes::Rectangle {
        shapes::Rectangle {
            width: self.theme.grid.bar_size / self.data.bar_units.0
                * Units::from(self.data.duration).0
                - self.theme.grid.note_outline * 2.0,
            height: self.theme.grid.note_height,
            origin: shapes::RectangleOrigin::BottomLeft,
        }
    }
    fn get_colors(&self) -> ShapeColors {
        ShapeColors::new(self.theme.syllable.color_of_syllable(self.data.syllable()))
    }
    fn get_draw_mode(&self) -> DrawMode {
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(self.theme.grid.note_outline),
        }
    }
    fn get_transform(&self) -> Transform {
        let x = self.theme.grid.bar_size / self.data.bar_units.0 * self.data.position.in_bar_pos.0;
        let y = if self.data.mode.is_melody() {
            self.theme.melody.calc_note_y(self.data.note)
        } else {
            0.0
        };
        Transform::from_xyz(x, y, self.theme.fretted.pick_z)
    }
}

impl<'a> LyonShapeOp<'a, ToneNoteData, shapes::Rectangle, ToneNoteShape<'a>> for ToneNoteShape<'a> {
    fn new_shape(theme: &'a NotationTheme, data: ToneNoteData) -> ToneNoteShape<'a> {
        ToneNoteShape::<'a> { theme, data }
    }
}