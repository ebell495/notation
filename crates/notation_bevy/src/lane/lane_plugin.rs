
use bevy::prelude::*;

use std::sync::Arc;

use crate::prelude::{
    LaneLayout, LyonShapeOp, NotationTheme, WindowResizedEvent,
};
use notation_model::prelude::{TabBar};

use super::lane_back::{LaneBack, LaneBackData};

pub struct LanePlugin;

impl Plugin for LanePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(on_config_changed.system());
        app.add_system(on_add_lane_layout.system());
    }
}

fn on_config_changed(
    mut commands: Commands,
    mut evts: EventReader<WindowResizedEvent>,
    theme: Res<NotationTheme>,
    back_query: Query<(Entity, &LaneBackData)>,
) {
    for _evt in evts.iter() {
        for (entity, data) in back_query.iter() {
            LaneBack::update(&mut commands, &theme, entity, data);
        }
    }
}

fn on_add_lane_layout(
    mut commands: Commands,
    theme: Res<NotationTheme>,
    query: Query<(Entity, &Arc<TabBar>, &LaneLayout), Added<LaneLayout>>,
) {
    for (entity, tab_bar, lane_layout) in query.iter() {
        let data = LaneBackData::new(tab_bar, lane_layout);
        LaneBack::create(&mut commands, entity, &theme, data);
    }
}