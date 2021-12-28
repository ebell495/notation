use bevy::prelude::*;
use bevy_egui::egui::{self, Ui, color_picker::show_color};
use notation_bevy_utils::{prelude::BevyUtil, asset::markdown_asset::MarkDownAsset, easy_mark::{EasyMarkStyle, label_from_style}};
use notation_model::prelude::{TrackKind, Pitch, Semitones, Scale, Syllable, Key, Interval};

use crate::prelude::{NotationTheme, NotationAssets, NotationAppState};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct PageHelper {
}

impl PageHelper {
    pub fn add_strong_text(
        ui: &mut Ui,
        text: &String,
    ) {
        let strong_style = EasyMarkStyle {
            strong: true,
            ..EasyMarkStyle::default()
        };
        ui.add(label_from_style(text.as_str(), &strong_style));
    }
    pub fn add_maybe_strong_text(
        ui: &mut Ui,
        strong: bool,
        text: &String,
    ) {
        if strong {
            Self::add_strong_text(ui, text);
        } else {
            ui.label(text);
        }
    }
    pub fn add_key_scale(
        ui: &mut Ui,
        key: &Key,
        scale: &Scale,
    ) {
        ui.horizontal(|ui|{
            ui.label("Key:");
            Self::add_strong_text(ui, &key.to_string());
            ui.label("Scale:");
            Self::add_strong_text(ui, &scale.to_string());
        });
    }
    pub fn add_syllable_color(
        ui: &mut Ui,
        theme: &NotationTheme,
        syllable: &Syllable,
    ) {
        let color = BevyUtil::rgb_to_egui(&theme.colors.of_syllable(syllable.clone()));
        show_color(ui, color, ui.spacing().interact_size);
    }
    pub fn add_syllable(
        ui: &mut Ui,
        theme: &NotationTheme,
        with_color: bool,
        syllable: &Syllable,
        show_ident: bool,
        strong: bool,
    ) {
        if with_color {
            Self::add_syllable_color(ui, theme, syllable);
        }
        let text = if show_ident { syllable.to_ident() } else { syllable.to_string() };
        Self::add_maybe_strong_text(ui, strong, &text);
    }
    pub fn add_syllable_pitch(
        ui: &mut Ui,
        _theme: &NotationTheme,
        scale: &Scale,
        key: &Key,
        syllable: &Syllable,
        strong: bool,
    ) {
        let pitch = scale.calc_pitch(&key, &syllable);
        let text = pitch.to_string();
        Self::add_maybe_strong_text(ui, strong, &text);
    }
    pub fn add_syllable_pitch_with_capo(
        ui: &mut Ui,
        _theme: &NotationTheme,
        capo: u8,
        scale: &Scale,
        key: &Key,
        syllable: &Syllable,
        strong: bool,
    ) {
        let pitch = scale.calc_pitch(&key, &syllable);
        let pitch = Pitch::from(Semitones::from(pitch) - Semitones(capo as i8));
        let text = pitch.to_string();
        Self::add_maybe_strong_text(ui, strong, &text);
    }
    pub fn add_interval_syllable(
        ui: &mut Ui,
        theme: &NotationTheme,
        with_color: bool,
        root: &Syllable,
        interval: &Interval,
        strong: bool,
    ) {
        let syllable = Syllable::from(Semitones::from(*root) + Semitones::from(*interval));
        Self::add_syllable(ui, theme, with_color, &syllable, false, strong);
    }
    pub fn add_interval(
        ui: &mut Ui,
        _theme: &NotationTheme,
        interval: &Interval,
        show_ident: bool,
        strong: bool,
    ) {
        let text = if show_ident { interval.to_ident() } else { interval.to_string() };
        Self::add_maybe_strong_text(ui, strong, &text);
    }
}