use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

use bevy::prelude::*;

use bevy_utils::prelude::{BevyUtil, GridData, GridView, LayoutAnchor, LayoutChangedQuery, LayoutQuery, LayoutSize, View, ViewAddedQuery, ViewQuery};
use notation_model::prelude::{Tab, TabBar};

use crate::bar::bar_layout::BarLayoutData;
use crate::bar::bar_view::BarView;
use crate::lane::lane_layout::LaneLayoutData;
use crate::prelude::{
    BarBundle, NotationAppState, NotationSettings,
    NotationTheme, PlayPlugin,
};
use crate::ui::layout::NotationLayout;

use super::tab_events::TabBarsDoLayoutEvent;

pub struct TabBars {
    pub tab: Arc<Tab>,
    pub bar_layouts: Arc<Vec<BarLayoutData>>,
}
impl Display for TabBars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TabBars>({})", self.tab.bars.len())
    }
}
impl TabBars {
    pub fn new(tab: Arc<Tab>, bar_layouts: Arc<Vec<BarLayoutData>>) -> Self {
        Self { tab, bar_layouts }
    }
}
impl<'a> View<NotationLayout<'a>> for TabBars {}

impl<'a> GridView<NotationLayout<'a>, BarView> for TabBars {
    fn calc_grid_data(&self, engine: &NotationLayout<'a>, grid_size: LayoutSize) -> GridData {
        let sizes = engine.theme.sizes.bar;
        let bar_beats = self.tab.bar_beats() as f32;
        let bar_width_range = (sizes.beat_size_range.0 * bar_beats, sizes.beat_size_range.1 * bar_beats);
        let (rows, cols, cell_width) = GridData::cals_fixed_rows_cols_by_width(
            grid_size.width - sizes.row_margin * 2.0, bar_width_range, 0.0, self.tab.bars.len());
        let height = self.bar_layouts.get(0)
            .map(|x| x.height)
            .unwrap_or(240.0);
        let size = LayoutSize::new(cell_width, height);
        let margin = LayoutSize::new(0.0, engine.theme.sizes.layout.bar_margin);
        GridData::new_fixed(grid_size, rows, cols, size, margin, LayoutAnchor::TOP_LEFT)
    }
}

impl TabBars {
    fn calc_all_lane_layouts(
        theme: &NotationTheme,
        settings: &NotationSettings,
        tab: &Tab
    ) -> Vec<LaneLayoutData> {
        let mut lane_layouts: HashMap<String, LaneLayoutData> = HashMap::new();
        for bar in tab.bars.iter() {
            for lane in bar.lanes.iter() {
                let lane_id = lane.id();
                if !lane_layouts.contains_key(&lane_id) {
                    let height = theme.sizes.layout.calc_lane_height(lane.kind);
                    let margin = theme.sizes.layout.lane_margin;
                    let index = lane_layouts.len();
                    lane_layouts.insert(lane_id, LaneLayoutData::new(index, &lane, height, margin));
                }
            }
        }
        let result = lane_layouts.into_iter()
            .map(|(_, lane_layout)| lane_layout)
            .collect::<Vec<LaneLayoutData>>();
        settings.layout.sort_lane_layouts(&result)
    }
    fn calc_bar_layout_data(
        all_lane_layouts: &Vec<LaneLayoutData>,
        bar: &TabBar
    ) -> BarLayoutData {
        let mut lane_layouts = all_lane_layouts.clone();
        for lane_layout in lane_layouts.iter_mut() {
            lane_layout.lane = bar.get_lane_of_kind(lane_layout.lane_kind, Some(lane_layout.track_props.index));
            lane_layout.visible = lane_layout.height > 0.0;
        }
        BarLayoutData::new(lane_layouts)
    }
    pub fn calc_bar_layouts(
        theme: &NotationTheme,
        settings: &NotationSettings,
        tab: &Tab,
    ) -> Vec<BarLayoutData> {
        let all_lane_layouts = Self::calc_all_lane_layouts(theme, settings, tab);
        tab
            .bars
            .iter()
            .map(|bar| {
                Self::calc_bar_layout_data(&all_lane_layouts, bar)
            })
            .collect()
    }
}

impl TabBars {
    pub fn on_added(
        mut commands: Commands,
        theme: Res<NotationTheme>,
        query: ViewAddedQuery<TabBars>,
    ) {
        for (_parent, entity, view) in query.iter() {
            PlayPlugin::spawn_indicators(
                &mut commands,
                &theme,
                entity,
                &view.tab,
            );
            let bar_bundles: Vec<(&BarLayoutData, BarBundle)> = view
                .tab
                .bars
                .iter()
                .enumerate()
                .filter_map(|(index, bar)| {
                    view.bar_layouts.get(index).map(|bar_layout| {
                        //let transform = theme.grid.calc_bar_transform(&bar_layout);
                        (
                            bar_layout,
                            BarBundle::new(bar.clone(), bar_layout.clone()),
                        )
                    })
                })
                .collect();
            for (_bar_layout, bar_bundle) in bar_bundles.into_iter() {
                BevyUtil::spawn_child_bundle(&mut commands, entity, bar_bundle);
            }
        }
    }
    pub fn do_layout(
        mut evts: EventReader<TabBarsDoLayoutEvent>,
        mut commands: Commands,
        theme: Res<NotationTheme>,
        state: Res<NotationAppState>,
        settings: Res<NotationSettings>,
        mut layout_query: LayoutQuery,
        cell_query: ViewQuery<BarView>,
    ) {
        let engine = NotationLayout::new(&theme, &state, &settings);
        for evt in evts.iter() {
            evt.view.do_layout(
                &mut commands,
                &engine,
                &mut layout_query,
                &cell_query,
                evt.entity,
                evt.layout,
            )
        }
    }
    pub fn on_layout_changed(
        query: LayoutChangedQuery<TabBars>,
        mut evts: EventWriter<TabBarsDoLayoutEvent>,
    ) {
        for (entity, view, layout) in query.iter() {
            evts.send(TabBarsDoLayoutEvent::new(entity, &view, layout))
        }
    }}
