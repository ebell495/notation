use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::WindowResized;

use bevy_asset_loader::prelude::*;

use crate::theme::theme_colors::UiColors;
use crate::prelude::*;
use super::tab_viewer::TabViewerPlugin;

#[cfg(feature = "with_egui")]
use super::egui_control_panel::EguiControlPanel;

#[cfg(feature = "midi")]
use notation_midi::prelude::{
    MidiPlugin,
};

pub struct NotationPlugins;
impl PluginGroup for NotationPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        #[cfg(feature = "with_egui")]
        group.add(EguiPlugin);

        group.add(EntryPlugin);
        group.add(MelodyPlugin);
        group.add(LyricsPlugin);
        group.add(BarPlugin);
        group.add(MelodyPlugin);
        group.add(HarmonyPlugin);
        group.add(StringsPlugin);
        group.add(ShapesPlugin);
        group.add(MiniPlugin);
        group.add(TabPlugin);
        group.add(PlayPlugin);
        group.add(TabViewerPlugin);

        //crates plugins
        #[cfg(feature = "midi")]
        group.add(MidiPlugin);
        //external plugins
        group.add(bevy_prototype_lyon::prelude::ShapePlugin);
        //group.add(bevy_svg::prelude::SvgPlugin);
    }
}

pub struct NotationApp;

impl NotationApp {
    pub const TITLE: &'static str = "Fun Notation";

    pub fn new_app<A: ExtraAssets>(args: NotationArgs, title: &str) -> App {
        let mut app = App::new();
        app.insert_resource(args);

        app.insert_resource(Msaa { samples: 4 });
        app.add_plugins(DefaultPlugins);
        app.insert_resource(ClearColor(UiColors::default().app_background));
        app.add_plugin(bevy_easings::EasingsPlugin);

        app.add_plugin(UtilsPlugin);

        app.init_resource::<NotationTheme>();
        app.init_resource::<NotationSettings>();
        app.add_plugins(NotationPlugins);

        app.add_loading_state(LoadingState::new(NotationAssetsStates::Loading)
            .continue_to_state(NotationAssetsStates::Loaded)
            .with_collection::<NotationAssets>()
            .with_collection::<A>()
        );
        app.add_state(NotationAssetsStates::Init);

        Self::insert_window_descriptor(&mut app, String::from(title));
        super::events::add_notation_app_events(&mut app);

        //#[cfg(target_arch = "wasm32")]
        //app.add_plugin(bevy_webgl2::WebGL2Plugin);

        // When building for WASM, print panics to the browser console
        #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(crate::wasm::bevy_web_fullscreen::FullViewportPlugin);

        #[cfg(feature = "with_egui")]
        app.add_plugin(crate::bevy_egui::EguiPlugin);

        app.add_plugin(NotationUiPlugin);

        #[cfg(feature = "dev")]
        app.add_plugins(crate::dev::NotationDevPlugins);

        app
    }

    pub fn run_with_extra<A, F>(args: NotationArgs, extra: F)
    where
        A: ExtraAssets,
        F: Fn(&mut App),
    {
        let mut app = NotationApp::new_app::<A>(args, Self::TITLE);

        app.init_resource::<NotationState>();

        app.add_startup_system(Self::setup_camera);

        #[cfg(debug_assertions)]
        app.add_startup_system(Self::setup_hot_reloading);

        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Init)
                .with_system(NotationAssets::setup_keys::<A>)
        );

        app.add_system_set(
            SystemSet::on_enter(NotationAssetsStates::Loaded)
                .with_system(NotationAssets::add_extra_assets::<A>)
                .with_system(Self::setup_window_size),
        );
        #[cfg(feature = "with_egui")]
        app.add_system_set(
            SystemSet::on_enter(NotationAssetsStates::Loaded)
                .with_system(crate::egui::egui_fonts::setup_egui_fonts::<A>),
        );
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(TabViewer::on_add_tab)
                .with_system(TabViewer::on_window_resized)
                .with_system(TabViewer::on_added),
        );
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(Self::on_window_resized)
                .with_system(Self::on_tab_asset),
        );
        #[cfg(feature = "with_egui")]
        app.add_system_set(
            SystemSet::on_update(NotationAssetsStates::Loaded)
                .with_system(EguiControlPanel::control_ui),
        );
        extra(&mut app);
        app.run();
    }
    pub fn run<A: ExtraAssets>(args: NotationArgs) {
        Self::run_with_extra::<A, _>(args, |_app|{})
    }
}

impl NotationApp {
    fn insert_window_descriptor(app: &mut App, title: String) {
        app.insert_resource(WindowDescriptor {
            title,
            //width: 1920.,
            //height: 1080.,
            ..WindowDescriptor::default()
        });
    }

    fn setup_camera(mut commands: Commands) {
        commands.spawn_bundle(Camera2dBundle::default());
    }

    /*
    * The open tab logic is using some trick in the asset server, which can load from absolute path
    * (outside assets folder), but the hot reloading is not working this way.
    * Ideally can use hot-reloading to update tabs automatically, but that means need to patch bevy
    * to bypass the assumption with asset path under assets folder in reloading.
    *
    * Only enabling for debug build, the hot reloading works really nice for help pages.
    *
    * The crash error is:
    *
    * thread 'Compute Task Pool (2)' panicked at 'called `Result::unwrap()` on an `Err` value: StripPrefixError(())', C:\Users\yjpark\scoop\persist\rustup-msvc\.cargo\registry\src\github.com-1ecc6299db9ec823\bevy_asset-0.5.1\src\io\file_asset_io.rs:135:84
    */
    fn setup_hot_reloading(asset_server: Res<AssetServer>) {
        asset_server.watch_for_changes().unwrap();
    }

    fn on_tab_asset(mut evts: EventReader<AssetEvent<TabAsset>>) {
        for evt in evts.iter() {
            println!("AssetEvent<TabAsset> {:?}", evt);
        }
    }

    fn setup_window_size(window: Res<WindowDescriptor>, mut app_state: ResMut<NotationState>) {
        #[cfg(target_arch = "wasm32")]
        let (width, height) = crate::wasm::bevy_web_fullscreen::get_viewport_size();

        #[cfg(not(target_arch = "wasm32"))]
        let (width, height) = (window.width, window.height);

        println!("setup_window_size(): {} {} ", width, height);
        app_state.window_width = width;
        app_state.window_height = height;
    }

    fn on_window_resized(
        mut window: ResMut<WindowDescriptor>,
        mut evts: EventReader<WindowResized>,
        mut app_state: ResMut<NotationState>,
        mut window_resized_evts: EventWriter<WindowResizedEvent>,
    ) {
        if app_state.tab_path.len() > 0 && app_state.tab.is_none() {
            return;
        }
        for evt in evts.iter() {
            if evt.width as usize != window.width as usize
                || evt.height as usize != window.height as usize
            {
                println!(
                    "on_window_resized(): {} {} -> {} {} ",
                    window.width, window.height, evt.width, evt.height
                );
                let resized_evt = WindowResizedEvent::new(&app_state);
                window.width = evt.width;
                window.height = evt.height;
                app_state.window_width = evt.width;
                app_state.window_height = evt.height;
                app_state.scale_factor_override = window.scale_factor_override;
                window_resized_evts.send(resized_evt);
            }
        }
    }
    pub fn load_tab<F: Fn(String) -> Option<TabAsset>>(
        commands: &mut Commands,
        time: &Time,
        windows: &mut Windows,
        state: &mut NotationState,
        theme: &mut NotationTheme,
        settings: &NotationSettings,
        evts: &mut EventWriter<AddTabEvent>,
        entities_query: &Query<Entity, With<GlobalTransform>>,
        viewer_query: &Query<(Entity, &TabViewer), With<TabViewer>>,
        load_tab: F,
    ) {
        if state.window_width > 0.0
            && state.window_height > 0.0
            && state.tab.is_none()
            && state.tab_error.is_none()
        {
            let mut count = 0;
            for _ in entities_query.iter() {
                count += 1;
            }
            //A bit hacky to make sure despawning finished, otherwise might got panic with "Entity not exist"
            if count > 1 {
                if state._despawn_delay_seconds > 0.0 {
                    state._despawn_delay_seconds -= time.delta_seconds();
                    println!(
                        "load_tab(): Waiting to despawn: {} -> {}",
                        count, state._despawn_delay_seconds
                    );
                    return;
                }
                let mut despawn_count = 0;
                for (entity, _viewer) in viewer_query.iter() {
                    commands.entity(entity).despawn_recursive();
                    despawn_count += 1;
                }
                if despawn_count > 0 {
                    println!(
                        "load_tab(): Despawning viewers: {} {}",
                        despawn_count, count
                    );
                } else {
                    println!(
                        "load_tab(): Waiting for entities to be despawned: {}",
                        count
                    );
                }
                return;
            }
            if state._load_tab_delay_seconds > 0.0 {
                state._load_tab_delay_seconds -= time.delta_seconds();
                println!(
                    "load_tab(): Waiting to Load tab: -> {}",
                    state._load_tab_delay_seconds
                );
                return;
            }
            println!("\nload_tab(): Loading: {}", state.tab_path);
            if state.tab_error.is_none() {
                if let Some(tab_asset) = load_tab(state.tab_path.clone()) {
                    match tab_asset.tab {
                        Ok(tab) => {
                            match Tab::try_parse_arc(tab, settings.add_ready_section, state.bars_range) {
                                Ok(tab) => {
                                    state.tab = Some(tab.clone());
                                    if let Some(window) = windows.get_primary_mut() {
                                        let title = format!("{} - {}", NotationApp::TITLE, state.tab_path);
                                        window.set_title(title);
                                    }
                                    theme._bypass_systems = false;
                                    evts.send(AddTabEvent(tab));
                                }
                                Err(err) => {
                                    println!("nload_tab(): Parse Tab Failed: {:?}", err);
                                    state.tab_error = Some(TabError::ParseFailed(err));
                                }
                            }
                        },
                        Err(err) => {
                            state.tab_error = Some(err);
                        },
                    }
                }
            }
        }
    }
    pub fn load_tab_from_assets(
        asset_server: &AssetServer,
        assets: &Assets<TabAsset>,
        tab_path: String,
    ) -> Option<TabAsset> {
        let tab_asset: Handle<TabAsset> = asset_server.load(tab_path.as_str());
        if let Some(asset) = assets.get(&tab_asset) {
            Some(asset.clone())
        } else {
            None
        }
    }
}
