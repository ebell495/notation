use bevy::prelude::*;
use bevy_egui::egui::{self, Ui, color_picker::show_color};
use notation_bevy_utils::{prelude::BevyUtil, asset::markdown_asset::MarkDownAsset, easy_mark::{EasyMarkStyle, label_from_style}};
use notation_model::prelude::{TrackKind, Pitch, Semitones};

use crate::prelude::{NotationTheme, NotationAssets, NotationAppState};

use super::{help_panel::{HelpPageId, HelpPage}, page_helper::PageHelper};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct ChordsPage {
}

impl HelpPage for ChordsPage {
    fn page_id(&self) -> HelpPageId {
        HelpPageId::Chords
    }
    fn tab_label(&self) -> &'static str {
        "Chords"
    }
    fn page_ui(
        &mut self,
        ui: &mut Ui,
        _texts: &Assets<MarkDownAsset>,
        _assets: &NotationAssets,
        state: &NotationAppState,
        theme: &NotationTheme,
    ) {
        if state.tab.is_none() {
            ui.label("Tab not loaded...");
            return;
        }
        let chords = state.tab.as_deref().unwrap()
            .get_track_of_kind(TrackKind::Chord)
            .map(|x| x.get_tab_chords())
            .unwrap_or_default();
        let scale = state.tab.as_ref().map(|x| x.meta.scale.clone()).unwrap_or_default();
        let key = state.tab.as_ref().map(|x| x.meta.key.clone()).unwrap_or_default();
        PageHelper::add_key_scale(ui, &key, &scale);
        ui.separator();
        egui::Grid::new("chords").show(ui, |ui| {
            for chord in chords.iter() {
                ui.label(format!("({})", chord.bars.len()));
                PageHelper::add_syllable(ui, theme, true, &chord.chord.root, false, true);
                let mut index = 0;
                for interval in chord.chord.intervals.get_intervals().iter() {
                    PageHelper::add_interval_syllable(ui, theme, true, &chord.chord.root, interval, false);
                    index += 1;
                }
                for _ in index..=3 {
                    ui.label("");
                    ui.label("");
                }
                if let Some(bass) = chord.chord.bass {
                    PageHelper::add_interval_syllable(ui, theme, true, &chord.chord.root, &bass, true);
                }
                ui.end_row();
                ui.label("");
                ui.label("root");
                PageHelper::add_syllable_pitch(ui, theme, &scale, &key, &chord.chord.root, true);
                index = 0;
                for interval in chord.chord.intervals.get_intervals().iter() {
                    PageHelper::add_interval(ui, theme, interval, true, false);
                    let syllable = interval.syllable_on_root(&chord.chord.root);
                    PageHelper::add_syllable_pitch(ui, theme, &scale, &key, &syllable, false);
                    index += 1;
                }
                for _ in index..=3 {
                    ui.label("");
                    ui.label("");
                }
                if let Some(bass) = chord.chord.bass {
                    ui.label("bass");
                    let syllable = bass.syllable_on_root(&chord.chord.root);
                    PageHelper::add_syllable_pitch(ui, theme, &scale, &key, &syllable, true);
                }
                ui.end_row();
            }
        });
    }
}