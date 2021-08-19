use serde::{Deserialize, Serialize};

use super::core_theme::CoreTheme;
use super::grid_theme::GridTheme;
use super::guitar_theme::GuitarTheme;
use super::lyrics_theme::LyricsTheme;
use super::melody_theme::MelodyTheme;
use super::shapes_theme::ShapesTheme;
use super::strings_theme::StringsTheme;
use super::theme_colors::ThemeColors;
use super::theme_sizes::ThemeSizes;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct NotationTheme {
    pub core: CoreTheme,
    pub grid: GridTheme,
    pub sizes: ThemeSizes,
    pub colors: ThemeColors,
    pub melody: MelodyTheme,
    pub lyrics: LyricsTheme,
    pub shapes: ShapesTheme,
    pub strings: StringsTheme,
    pub guitar: GuitarTheme,
}

