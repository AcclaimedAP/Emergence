use bevy::{
    app::AppExit,
    asset::LoadState,
    prelude::*,
    reflect::TypeUuid,
    utils::{Duration, HashMap, Instant},
};

use std::fmt::{Display, Formatter};

pub(super) fn verify_assets_load() {
    App::new()
        .init_resource::<AssetHandles>()
        .insert_resource(TimeOut {
            start: Instant::now(),
            max: Duration::from_secs(10),
        })
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_startup_system(load_assets)
        .add_system(check_if_assets_loaded)
        .run()
}

#[derive(Default, Resource, Debug, Clone, PartialEq)]
struct AssetHandles {
    font_handles: HashMap<String, HandleStatus<Font>>,
}

impl Display for AssetHandles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        string += "Fonts\n";
        for (name, handle) in self.font_handles.iter() {
            string += &format!("    {} - {:?}\n", name, handle.load_state);
        }

        write!(f, "{string}")
    }
}

impl AssetHandles {
    fn all_loaded(&self) -> bool {
        self.font_handles
            .values()
            .all(|handle| handle.load_state == LoadState::Loaded)
    }
}

#[derive(Debug)]
struct HandleStatus<T: Send + Sync + TypeUuid + 'static> {
    handle: Handle<T>,
    load_state: LoadState,
}

impl<T: Send + Sync + TypeUuid + 'static> Clone for HandleStatus<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            load_state: self.load_state,
        }
    }
}

impl<T: Send + Sync + TypeUuid + 'static> Default for HandleStatus<T> {
    fn default() -> Self {
        Self {
            handle: Handle::default(),
            load_state: LoadState::NotLoaded,
        }
    }
}

impl<T: Send + Sync + TypeUuid + 'static> PartialEq for HandleStatus<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle && self.load_state == other.load_state
    }
}

#[derive(Resource, Debug)]
struct TimeOut {
    start: Instant,
    max: Duration,
}

impl TimeOut {
    fn timed_out(&self) -> bool {
        self.start.elapsed() > self.max
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut asset_handles: ResMut<AssetHandles>) {
    let all_fonts = vec!["fonts/FiraSans-Bold.ttf"];

    for font in all_fonts {
        let font_handle = asset_server.load(font);
        asset_handles.font_handles.insert(
            font.to_string(),
            HandleStatus {
                handle: font_handle,
                load_state: LoadState::NotLoaded,
            },
        );
    }
}

fn check_if_assets_loaded(
    mut asset_handles: ResMut<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    time_out: Res<TimeOut>,
    mut previous_asset_handles: Local<AssetHandles>,
) {
    *previous_asset_handles = asset_handles.clone();

    for mut handle_status in asset_handles.font_handles.values_mut() {
        if handle_status.load_state == LoadState::NotLoaded {
            handle_status.load_state =
                asset_server.get_load_state(handle_status.handle.clone_weak());
        }
    }

    if asset_handles.all_loaded() {
        println!("All assets loaded successfully, exiting.");
        app_exit_events.send(AppExit);
    } else {
        if *asset_handles != *previous_asset_handles {
            println!("{}", *asset_handles);
        }

        if time_out.timed_out() {
            panic!("Timed out waiting for assets to load.");
        }
    }
}
