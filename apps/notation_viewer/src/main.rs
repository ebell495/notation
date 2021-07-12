// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use std::sync::Arc;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

use bevy_inspector_egui::{bevy_egui, egui};

use notation_bevy::prelude::{AddLineEvent, AddTabEvent, ConfigPlugin, NotationDevPlugins, NotationPlugins, TabAsset};
use notation_model::prelude::{Bar, BarLayer, CoreEntry, Duration, GuitarEntry, GuitarHandShape, GuitarString, GuitarTuning, GuitarUtil, Key, Line, ParseError, Pick, ProtoEntry, Roman, Scale, Section, SectionKind, Signature, Slice, Solfege, Tab, TabMeta, Tempo, Track, TrackKind};

#[cfg(target_arch = "wasm32")]
pub mod bevy_web_fullscreen;

pub struct CameraPanning(bool);

pub struct AppState {
    pub tab_asset: Handle<TabAsset>,
    pub tab: Option<Arc<Tab>>,
    pub parse_error: Option<ParseError>,
}

impl AppState {
    pub fn new(tab_asset: Handle<TabAsset>) -> Self {
        Self { tab_asset, tab: None, parse_error: None }
    }
}

fn main() {
    let mut app = App::build();
    ConfigPlugin::insert_window_descriptor(&mut app, String::from("Notation Viewer"));
    app.insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugins(NotationPlugins)
        .add_startup_system(setup.system());

    #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
    app.insert_resource(CameraPanning(false))
        .add_system(update_camera.system())
        .add_plugins(NotationDevPlugins)
        .add_system(setup_ui.system());

    app.add_system(load_tab.system());

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_web_fullscreen::FullViewportPlugin);

    app.run();
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let tab_asset = server.load("amateur_guitar/1_right_hand.ron");
    commands.insert_resource(AppState::new(tab_asset));
}

fn load_tab(
    mut state: ResMut<AppState>,
    assets: ResMut<Assets<TabAsset>>,
    mut evts: EventWriter<AddTabEvent>,
) {
    if state.tab.is_none() && state.parse_error.is_none() {
        if let Some(asset) = assets.get(&state.tab_asset) {
            match Tab::try_parse_arc(asset.tab.clone()) {
                Ok(tab) => {
                    state.tab = Some(tab.clone());
                    evts.send(AddTabEvent(tab));
                },
                Err(err) => {
                    println!("Parse Tab Failed: {:?}", err);
                    state.parse_error = Some(err);
                }
            }
        }
    }
}

fn update_camera(
    _keyboard_input: Res<Input<KeyCode>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut camera_panning: ResMut<CameraPanning>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut get_cam: Query<(&mut Transform, &mut OrthographicProjection)>,
) {
    if keyboard_input.just_released(KeyCode::Space) {
        *camera_panning = match camera_panning.0 {
            true => CameraPanning(false),
            false => CameraPanning(true),
        }
    }

    if camera_panning.0 {
        for event in mouse_motion_events.iter() {
            if mouse_input.pressed(MouseButton::Left) {
                let (mut cam, _) = get_cam.single_mut().unwrap();
                let trans = cam.translation;
                *cam =
                    Transform::from_xyz(trans.x - event.delta.x, trans.y + event.delta.y, trans.z);
            }
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    egui_context: ResMut<bevy_egui::EguiContext>,
    mut camera_panning: ResMut<CameraPanning>,
    tab_query: Query<Entity, With<Arc<Tab>>>,
    line_query: Query<Entity, With<Arc<Line>>>,
    tab_evts: EventWriter<AddTabEvent>,
    line_evts: EventWriter<AddLineEvent>,
) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        if ui
            .button(format!("[space] Camera Panning: {:?}", camera_panning.0))
            .clicked()
        {
            *camera_panning = match camera_panning.0 {
                true => CameraPanning(false),
                false => CameraPanning(true),
            }
        }
        ui.separator();
        if ui.button("Clear Tabs").clicked() {
            for tab in tab_query.iter() {
                commands.entity(tab).despawn_recursive();
            }
        }
        if ui.button("Load Tab").clicked() {
            state.tab = None;
            state.parse_error = None;
        }
    });
}
