use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, io};

use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowMode};
use triangles3bevy::{DataPath, GamePlugin}; // ToDo: Replace bevy_game with your new crate name.

fn internal_data_path() -> io::Result<PathBuf> {
    if let Err(e) = fs::create_dir("./assets") {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(e);
        }
    }
    Ok(PathBuf::from_str("./assets").unwrap())
}

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
            GamePlugin::default(),
        ))
        .add_systems(Update, close_on_esc)
        .insert_resource(DataPath(internal_data_path().unwrap()))
        .run()
}
