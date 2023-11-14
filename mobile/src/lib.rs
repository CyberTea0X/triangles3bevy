use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::WindowMode;
use triangles3bevy::{DataPath, GamePlugin}; // ToDo: Replace bevy_game with your new crate name.

fn internal_data_path() -> Option<PathBuf> {
    let android_app = bevy::winit::ANDROID_APP
        .get()
        .expect("Bevy must be set up with the #[bevy_main] macro on Android");
    android_app.internal_data_path()
}

#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }),
        GamePlugin::default(),
    ));
    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);
    #[cfg(target_os = "android")]
    app.insert_resource(DataPath(
        internal_data_path().expect("App has no data path"),
    ));
    app.run();
}
