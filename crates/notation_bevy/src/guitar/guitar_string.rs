use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_utils::prelude::{BevyUtil, LayoutSize};
use notation_model::prelude::{Duration, PlaySpeed, PlayingState, Units};

use crate::prelude::{LyonShape, LyonShapeOp, NotationTheme};

#[derive(Clone, Debug)]
pub struct GuitarStringData {
    pub string: u8,
    pub guitar_size: LayoutSize,
    pub state: PlayingState,
    pub hit: bool,
    pub hit_duration: Duration,
    pub hit_seconds: f32,
    pub hit_expired_seconds: f64,
}

impl GuitarStringData {
    pub fn new(string: u8) -> Self {
        Self {
            string: string,
            guitar_size: LayoutSize::ZERO,
            state: PlayingState::Idle,
            hit: false,
            hit_duration: Duration::Zero,
            hit_seconds: 0.0,
            hit_expired_seconds: 0.0,
        }
    }
    fn calc_hit_seconds(
        hit_duration: Duration,
        hit_string_seconds_range: (f32, f32),
        play_speed: PlaySpeed,
    ) -> f32 {
        let seconds = play_speed.calc_seconds(Units::from(hit_duration));
        BevyUtil::in_range(seconds * 0.5, hit_string_seconds_range)
    }
    pub fn set_hit(
        &mut self,
        hit: bool,
        hit_duration: Duration,
        time: &Time,
        hit_string_seconds_range: (f32, f32),
        play_speed: PlaySpeed,
    ) {
        if self.hit && !hit && time.seconds_since_startup() < self.hit_expired_seconds {
            return;
        }
        self.hit = hit;
        self.hit_duration = hit_duration;
        self.hit_seconds = if hit {
            Self::calc_hit_seconds(hit_duration, hit_string_seconds_range, play_speed)
        } else {
            0.0
        };
        self.hit_expired_seconds = time.seconds_since_startup() + self.hit_seconds as f64;
    }
}

pub struct GuitarString<'a> {
    theme: &'a NotationTheme,
    data: GuitarStringData,
}

impl<'a> LyonShape<shapes::Line> for GuitarString<'a> {
    fn get_name(&self) -> String {
        format!("<GuitarString>({})", self.data.string)
    }
    fn get_shape(&self) -> shapes::Line {
        shapes::Line(
            Vec2::new(
                0.0,
                self.data.guitar_size.height * self.theme.guitar.string_y_factor,
            ),
            Vec2::new(0.0, -self.data.guitar_size.height / 2.0),
        )
    }
    fn get_colors(&self) -> ShapeColors {
        let color = if self.data.hit {
            self.theme.colors.strings.hit
        } else {
            self.theme.colors.strings.string.of_state(&self.data.state)
        };
        ShapeColors::new(color)
    }
    fn get_draw_mode(&self) -> DrawMode {
        let line_width = self.theme.guitar.get_string_width(self.data.string);
        DrawMode::Stroke(StrokeOptions::default().with_line_width(line_width))
    }
    fn get_transform(&self) -> Transform {
        if self.data.guitar_size.width <= 0.0 {
            return BevyUtil::offscreen_transform();
        }
        let x = self
            .theme
            .guitar
            .calc_string_x(self.data.string, self.data.guitar_size.width);
        Transform::from_xyz(x, 0.0, self.theme.core.mini_bar_z + 1.0)
    }
}

impl<'a> LyonShapeOp<'a, NotationTheme, GuitarStringData, shapes::Line, GuitarString<'a>>
    for GuitarString<'a>
{
    fn new_shape(theme: &'a NotationTheme, data: GuitarStringData) -> GuitarString<'a> {
        GuitarString::<'a> { theme, data }
    }
}
