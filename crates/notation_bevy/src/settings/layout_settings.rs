use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingComponent, EasingType};
use float_eq::float_ne;
use std::sync::Arc;

use notation_model::prelude::{BarLane, LaneKind, Position, Units};
use serde::{Deserialize, Serialize};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::Inspectable;

use crate::bar::bar_layout::BarLayoutData;
use crate::lane::lane_layout::LaneLayoutData;
use crate::prelude::{TabBars, TabState};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub enum LayoutMode {
    Grid,
    Line,
}
impl Default for LayoutMode {
    fn default() -> Self {
        Self::Grid
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "inspector", derive(Inspectable))]
pub struct LayoutSettings {
    pub mode: LayoutMode,
    pub focus_bar_ease_ms: u64,
    pub shapes_lane_order: usize,
    pub strings_lane_order: usize,
    pub lyrics_lane_order: usize,
    pub melody_lane_order: usize,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            mode: LayoutMode::default(),
            focus_bar_ease_ms: 250,
            shapes_lane_order: 1,
            strings_lane_order: 2,
            lyrics_lane_order: 3,
            melody_lane_order: 4,
        }
    }
}

/*
impl LayoutSettings {
    pub fn calc_lane_order(&self, lane: &BarLane) -> u8 {
        match lane.kind {
            LaneKind::Lyrics => self.lyrics_lane_order,
            LaneKind::Melody => self.melody_lane_order,
            LaneKind::Strings => self.strings_lane_order,
            LaneKind::Shapes => self.shapes_lane_order,
            _ => 0,
        }
    }
    fn calc_lane_layout_data(
        &self,
        _app_state: &NotationAppState,
        lane: &BarLane,
    ) -> Option<LaneLayoutData> {
        let height = match lane.kind {
            LaneKind::Lyrics => self.lyrics_height,
            LaneKind::Melody => self.melody_height,
            LaneKind::Strings => self.strings_height,
            LaneKind::Shapes => self.shapes_height,
            _ => 0.0,
        };
        if height > 0.0 {
            let order = self.calc_lane_order(lane);
            Some(LaneLayoutData::new(order, 0.0, height, self.lane_margin))
        } else {
            None
        }
    }
    fn calc_lane_layouts_data(
        &self,
        app_state: &NotationAppState,
        bar: &TabBar,
    ) -> HashMap<String, LaneLayoutData> {
        bar.lanes
            .iter()
            .map(|lane| (lane, self.calc_lane_layout_data(app_state, lane)))
            .filter_map(|(lane, layout)| layout.map(|layout| (lane.id(), layout)))
            .collect()
    }
    fn calc_lane_layouts(
        &self,
        _app_state: &NotationAppState,
        lane_layouts_data: HashMap<String, LaneLayoutData>,
    ) -> HashMap<String, LaneLayoutData> {
        let mut layouts: Vec<(String, LaneLayoutData)> = lane_layouts_data.into_iter().collect();
        layouts.sort_by(|(_, a), (_, b)| a.order.cmp(&b.order));
        let mut y: f32 = 0.0;
        layouts
            .into_iter()
            .map(|(lane_id, data)| {
                let offset = y;
                y -= data.height + data.margin;
                (lane_id.clone(), LaneLayout::new(data, offset))
            })
            .collect()
    }
    fn _calc_bar_row_col(&self, cols: usize, index: usize) -> (usize, usize) {
        match self.mode {
            LayoutMode::Grid => {
                let row = index / cols as usize;
                let col = index % cols as usize;
                (row, col)
            }
            LayoutMode::Line => (0, index),
        }
    }
    fn calc_bar_layout_data(&self, cols: usize, app_state: &NotationAppState, bar: &TabBar) -> BarLayoutData {
        let (row, col) = self._calc_bar_row_col(cols, bar.props.bar_ordinal - 1);
        BarLayoutData::new(
            self.bar_margin,
            row,
            col,
            Arc::new(self.calc_lane_layouts_data(app_state, bar)),
        )
    }
    pub fn calc_pos_layout(&self, cols: usize, tab: &Tab, pos: TabPosition) -> (usize, usize) {
        let bar_units = tab.bar_units();
        let mut index = (pos.in_tab_pos.0 / bar_units.0) as usize;
        let bars = tab.bars.len();
        if index >= bars {
            index = bars - 1;
        }
        self._calc_bar_row_col(cols, index)
    }
    fn merge_row_lane_layouts_data(
        &self,
        row_lane_layouts_data: &mut HashMap<String, LaneLayoutData>,
        bar_layout_data: &BarLayoutData,
    ) {
        for (lane_id, lane_layout_data) in bar_layout_data.lane_layouts.iter() {
            if !row_lane_layouts_data.contains_key(lane_id) {
                row_lane_layouts_data.insert(lane_id.clone(), *lane_layout_data);
            }
        }
    }
    fn calc_lane_layouts_height(
        &self,
        _app_state: &NotationAppState,
        lane_layouts_data: &HashMap<String, LaneLayoutData>,
    ) -> f32 {
        let mut height = 0.0;
        for (index, (_, lane_layout_data)) in lane_layouts_data.iter().enumerate() {
            height += lane_layout_data.height;
            if index != lane_layouts_data.len() {
                height += lane_layout_data.margin;
            }
        }
        height
    }
    pub fn calc_bar_layouts(&self, cols: usize, app_state: &NotationAppState, tab: &Tab) -> Vec<BarLayoutData> {
        let with_layouts_data: Vec<(&Arc<TabBar>, BarLayoutData)> = tab
            .bars
            .iter()
            .map(|bar| (bar, self.calc_bar_layout_data(cols, app_state, bar)))
            .collect();
        let mut rows_lane_layouts_data: Vec<HashMap<String, LaneLayoutData>> = Vec::new();
        for (_bar, bar_layout_data) in with_layouts_data.iter() {
            while rows_lane_layouts_data.len() < bar_layout_data.row + 1 {
                rows_lane_layouts_data.push(HashMap::new());
            }
            let mut row_lane_layouts_data =
                rows_lane_layouts_data.get_mut(bar_layout_data.row).unwrap();
            self.merge_row_lane_layouts_data(&mut row_lane_layouts_data, &bar_layout_data);
        }
        let mut y: f32 = 0.0;
        let rows_lane_layouts: Vec<(f32, f32, Arc<HashMap<String, LaneLayoutData>>)> =
            rows_lane_layouts_data
                .into_iter()
                .map(|data| {
                    let offset = y;
                    let height = self.calc_lane_layouts_height(app_state, &data);
                    let lane_layouts = self.calc_lane_layouts(app_state, data);
                    y -= height + self.bar_margin;
                    (offset, height, Arc::new(lane_layouts))
                })
                .collect();
        with_layouts_data
            .into_iter()
            .map(|(_bar, bar_layout_data)| {
                let (offset, height, lane_layouts) =
                    rows_lane_layouts.get(bar_layout_data.row).unwrap();
                BarLayout::new(bar_layout_data, *offset, *height, lane_layouts.clone())
            })
            .collect()
    }
}
*/
impl LayoutSettings {
    pub fn calc_lane_order(&self, lane_layout: &LaneLayoutData) ->  (usize, usize) {
        let (track_index, lane_kind) = lane_layout.order();
        (track_index, match lane_kind {
            LaneKind::Lyrics => self.lyrics_lane_order,
            LaneKind::Melody => self.melody_lane_order,
            LaneKind::Strings => self.strings_lane_order,
            LaneKind::Shapes => self.shapes_lane_order,
            _ => 0,
        })
    }
    pub fn sort_lane_layouts(&self, lanes: &Vec<LaneLayoutData>) -> Vec<LaneLayoutData> {
        let mut sorted: Vec<LaneLayoutData> = lanes.clone();
        sorted.sort_by(|a, b| {
            self.calc_lane_order(a).cmp(&self.calc_lane_order(b))
        });
        sorted
    }
    pub fn bar_layout_of_pos(
        &self,
        //bar_layouts: &Arc<Vec<BarLayoutData>>,
        pos: Position,
    ) -> Option<BarLayoutData> {
        //bar_layouts.get(pos.bar.bar_ordinal - 1).map(|x| x.clone())
        None
    }
    pub fn pan_tab_bars(
        &self,
        tab_bars_query: &mut Query<(Entity, &mut Transform, &Arc<TabBars>)>,
        delta_x: f32,
        delta_y: f32,
    ) {
        if let Ok((_, mut camera_transform, _)) = tab_bars_query.single_mut() {
            let trans = camera_transform.translation;
            let (x, y) = match self.mode {
                LayoutMode::Grid => (trans.x, trans.y + delta_y),
                LayoutMode::Line => (trans.x - delta_x, trans.y),
            };
            *camera_transform = Transform::from_xyz(x, y, trans.z);
        }
    }
    pub fn set_tab_bars_xy(
        &self,
        tab_bars_query: &mut Query<(Entity, &mut Transform, &Arc<TabBars>)>,
        x: Option<f32>,
        y: Option<f32>,
    ) {
        if let Ok((_, mut camera_transform, _)) = tab_bars_query.single_mut() {
            let trans = camera_transform.translation;
            *camera_transform =
                Transform::from_xyz(x.unwrap_or(trans.x), y.unwrap_or(trans.y), trans.z);
        }
    }
    pub fn ease_tab_bars_xy(
        &self,
        commands: &mut Commands,
        tab_bars_query: &mut Query<(Entity, &mut Transform, &Arc<TabBars>)>,
        x: Option<f32>,
        y: Option<f32>,
    ) {
        if let Ok((camera_entity, camera_transform, _)) = tab_bars_query.single_mut() {
            let mut camera_commands = commands.entity(camera_entity);
            camera_commands.remove::<EasingComponent<Transform>>();
            let from = camera_transform.translation;
            let to = Vec3::new(x.unwrap_or(from.x), y.unwrap_or(from.y), from.z);
            if float_ne!(from.x, to.x, abs <= 0.01) || float_ne!(from.y, to.y, abs <= 0.01) {
                println!(
                    "ease_camera_xy: {}, {} -> {}, {}",
                    from.x, from.y, to.x, to.y
                );
                let ease_function = EaseFunction::CubicOut;
                camera_commands.insert(camera_transform.ease_to(
                    Transform::from_translation(to),
                    ease_function,
                    EasingType::Once {
                        duration: std::time::Duration::from_millis(self.focus_bar_ease_ms),
                    },
                ));
            }
        }
    }
    pub fn should_focus_bar(&self, old: &Position, new: &Position) -> bool {
        self.mode == LayoutMode::Line || old.bar.bar_ordinal != new.bar.bar_ordinal
    }
    fn calc_grid_focus_y(
        &self,
        //bar_layouts: &Arc<Vec<BarLayoutData>>,
        bar_layout: &BarLayoutData,
        _pos: &Position,
    ) -> f32 {
        0.0
    }
    fn calc_line_focus_x_units(&self, bar_layout: &BarLayoutData, pos: &Position) -> Units {
        Units(0.0)
        /*
        Units(if bar_layout.data.col > 0 {
            bar_layout.data.col as f32 - 1.0 + pos.bar.in_bar_pos.0 / pos.bar.bar_units.0
        } else {
            bar_layout.data.col as f32
        })
         */
    }
    pub fn focus_bar(
        &self,
        commands: &mut Commands,
        tab_bars_query: &mut Query<(Entity, &mut Transform, &Arc<TabBars>)>,
        //bar_layouts: &Arc<Vec<BarLayoutData>>,
        bar_size: f32,
        state: &TabState,
    ) {
        let pos = state.play_control.position;
        /*
        if let Some(bar_layout) = bar_layouts.get(pos.bar.bar_ordinal - 1) {
            match self.mode {
                LayoutMode::Grid => {
                    let y = self.calc_grid_focus_y(bar_layouts, bar_layout, &pos);
                    self.ease_tab_bars_xy(commands, tab_bars_query, None, Some(-y));
                }
                LayoutMode::Line => {
                    let x_units = self.calc_line_focus_x_units(bar_layout, &pos);
                    if state.play_control.play_state.is_playing() {
                        self.set_tab_bars_xy(tab_bars_query, Some(-x_units.0 * bar_size), None);
                    } else {
                        self.ease_tab_bars_xy(
                            commands,
                            tab_bars_query,
                            Some(-x_units.0 * bar_size),
                            None,
                        );
                    }
                }
            }
        }
         */
    }
}
