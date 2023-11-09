use radial_background::create_radial_gradient;
use std::{fs, path::PathBuf};

use bevy::{prelude::*, render::texture::ImageType, window::PrimaryWindow};
pub struct GamePlugin;

#[derive(Resource, Deref, DerefMut)]
pub struct DataPath(pub PathBuf);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    data_path: Res<DataPath>,
) {
    commands.spawn(Camera2dBundle::default());
    let triangle = asset_server.load("tBlue.png");
    // It seems that path.join changes the path to assets if assets not cloned
    let bg_path = data_path.clone().join("background.png");
    if let Err(_) = fs::metadata(&bg_path) {
        println!("generating asset: {:?}", &bg_path);
        let color1 = &[255u8, 255u8, 255u8, 255u8];
        let color2 = &[145u8, 28u8, 139u8, 255u8];
        let window = window.single();
        println!(
            "creating directory for dynamic data: {:?}",
            data_path.as_path()
        );
        if let Err(e) = fs::DirBuilder::new()
            .recursive(true)
            .create(data_path.as_path())
        {
            println!("Failed to create data directory: {:?}", e);
        }
        if let Err(e) = create_radial_gradient(
            0.5,
            window.physical_width() as u32,
            window.physical_height() as u32,
            color1,
            color2,
            bg_path.to_str().unwrap(),
        ) {
            println!("background gradient generation failed: {}", e);
        }
    }
    match image::open(&bg_path) {
        Ok(bg) => {
            let bg = asset_server.add(Image::from_dynamic(bg, false));
            commands.spawn((SpriteBundle {
                texture: bg,
                ..default()
            },));
        }
        Err(e) => println!("background not loaded: {:?}\n bg_path:{:?}", e, &bg_path),
    }

    commands.spawn((SpriteBundle {
        texture: triangle,
        ..default()
    },));
}
