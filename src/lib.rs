use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use radial_background::create_radial_gradient;
use rand::Rng;
use std::{fs, path::PathBuf, str::FromStr};

use bevy::{asset, prelude::*, window::PrimaryWindow};
pub struct GamePlugin {
    default_resources: bool,
}

impl Default for GamePlugin {
    fn default() -> Self {
        GamePlugin {
            default_resources: true,
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_field).chain())
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
            )
            .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading)
            .add_systems(PostStartup, (calculate_cells).chain())
            .add_systems(
                Update,
                (
                    // rotate_field.run_if(run_once()),
                    spawn_squares,
                ),
            )
            .add_systems(
                OnExit(GameState::AssetLoading),
                spawn_triangles.run_if(run_once()),
            );

        if self.default_resources {
            let gradient = GradientConfig {
                color1: Color::rgb_u8(255, 255, 255),
                color2: Color::rgba_u8(145, 28, 139, 255),
                radius: 0.25,
            };
            app.insert_resource(Config {
                field_scale: 8,
                square_bg: Color::hex("911C8B").unwrap(),
                gradient,
            });
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
struct MyAssets {
    #[asset(path = "triangles/tRed.png")]
    t_red: Handle<Image>,
    #[asset(path = "triangles/tGreen.png")]
    t_green: Handle<Image>,
    #[asset(path = "triangles/tBlue.png")]
    t_blue: Handle<Image>,
    #[asset(path = "triangles/tOrange.png")]
    t_orange: Handle<Image>,
    #[asset(path = "triangles/tPurple.png")]
    t_purple: Handle<Image>,
    #[asset(path = "triangles/tTeal.png")]
    t_teal: Handle<Image>,
    #[asset(path = "triangles/tYellow.png")]
    t_yellow: Handle<Image>,
}

impl MyAssets {
    pub fn from_tcolor(&self, color: TriangleColor) -> Handle<Image> {
        match color {
            TriangleColor::Red => self.t_red.clone(),
            TriangleColor::Green => self.t_green.clone(),
            TriangleColor::Blue => self.t_blue.clone(),
            TriangleColor::Orange => self.t_orange.clone(),
            TriangleColor::Purple => self.t_purple.clone(),
            TriangleColor::Teal => self.t_teal.clone(),
            TriangleColor::Yellow => self.t_yellow.clone(),
        }
    }
}

// Path to the data directory. There should be stored all mutable data
#[derive(Resource, Deref, DerefMut)]
pub struct DataPath(pub PathBuf);

#[derive(Resource)]
pub struct Config {
    field_scale: u32,
    square_bg: Color,
    gradient: GradientConfig,
}

pub struct GradientConfig {
    color1: Color,
    color2: Color,
    radius: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Field {
    scale: u32,
    center: Vec2,
    size: f32,
    smaller: f32,
    cell_spacing: f32,
    cell_size: f32,
}

impl Field {
    pub fn calculate(scale: u32, center: Vec2, size: f32) -> Self {
        let smaller = size / 2.0f32.sqrt();
        // we subtract -2.0 because we want the size to be calculated for the smaller field.
        let cell_size = smaller / (scale as f32 - 2.0);
        let cell_spacing = cell_size * 0.05;
        let cell_size = cell_size * 0.95;
        Field {
            scale,
            center,
            size,
            smaller,
            cell_spacing,
            cell_size,
        }
    }
}

#[derive(Component)]
pub struct FieldBackground;

#[derive(Component, Debug, Clone, Copy)]
pub struct Cell {
    id: u32,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum TriangleCell {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Clone, Copy, Debug)]
pub enum SquareOrientation {
    Horizontal,
    Vertical,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Square {
    orientation: SquareOrientation,
}

// All possible colors of the triangle
#[derive(Debug, Clone, Copy)]
pub enum TriangleColor {
    Red,
    Green,
    Blue,
    Orange,
    Purple,
    Teal,
    Yellow,
}

impl TriangleColor {
    pub fn asset_path(&self) -> PathBuf {
        match self {
            Self::Red => PathBuf::from_str("tRed.png").unwrap(),
            Self::Green => PathBuf::from_str("tGreen.png").unwrap(),
            Self::Blue => PathBuf::from_str("tBlue.png").unwrap(),
            Self::Orange => PathBuf::from_str("tOrange.png").unwrap(),
            Self::Purple => PathBuf::from_str("tPurple.png").unwrap(),
            Self::Teal => PathBuf::from_str("tTeal.png").unwrap(),
            Self::Yellow => PathBuf::from_str("tYellow.png").unwrap(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Triangle {
    color: TriangleColor,
}

fn spawn_field(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    config: Res<Config>,
) {
    let window = window.single();
    let field = Field::calculate(
        config.field_scale,
        Vec2::new(0.0, 0.0),
        window.width().min(window.height()) * 0.5,
    );
    // Field (without sprite rotation of cells not working)
    let mut entity = commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.00),
            custom_size: Some(Vec2::new(field.size, field.size)),
            ..default()
        },
        transform: Transform::from_translation(field.center.extend(1.0)),
        ..default()
    });
    entity.insert(field);
    // Field Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.30),
            custom_size: Some(Vec2::new(field.size, field.size)),
            ..default()
        },
        transform: Transform::from_translation(field.center.extend(1.0)),
        ..default()
    });
}

fn calculate_cells(mut commands: Commands, field: Query<&Field>) {
    let field = field.single();
    let left = -field.smaller / 2.0 + field.cell_size / 2.0 + field.cell_spacing / 2.0;
    let bottom = -field.smaller / 2.0 + field.cell_size / 2.0 + field.cell_spacing / 2.0;
    let field_scale = field.scale as i32;
    for i in 0..field_scale {
        let x_pos = left + (i - 1) as f32 * (field.cell_size + field.cell_spacing);
        for j in 0..field_scale {
            let y_pos = bottom + (j - 1) as f32 * (field.cell_size + field.cell_spacing);
            if (i == 0 || i == field_scale - 1) && (j == 0 || j == field_scale - 1) {
                continue;
            }
            // /** Формула выбирает элементы по диагонали и проверяет на чётность */
            //const getSplit = (i: number, s: number): SquareSplit => (((i + Math.floor(i / s - 1)) % 2 != 0) ? 0 : 1)

            commands.spawn((
                Transform::from_translation(Vec3::new(x_pos, y_pos, 2.0)),
                Cell { id: i as u32 },
                Square {
                    orientation: SquareOrientation::Horizontal,
                },
            ));
        }
    }
}

fn spawn_squares(
    mut commands: Commands,
    field: Query<(Entity, &Field)>,
    config: Res<Config>,
    squares: Query<(Entity, &mut Transform), (With<Square>, Without<Sprite>)>,
) {
    let (field_id, field) = field.single();
    for (id, transform) in &squares {
        commands
            .entity(id)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: config.square_bg,
                    custom_size: Some(Vec2::new(field.cell_size, field.cell_size)),
                    ..default()
                },
                transform: *transform,
                ..default()
            })
            .set_parent(field_id);
        commands.entity(field_id).add_child(id);
    }
}

fn spawn_triangles(
    mut commands: Commands,
    my_assets: Res<MyAssets>,
    squares: Query<(Entity, &Transform, &Square)>,
) {
    let mut rng = rand::thread_rng();
    for (id, transform, square) in &squares {
        let color = match rng.gen_range(0..4) {
            0 => TriangleColor::Red,
            1 => TriangleColor::Green,
            2 => TriangleColor::Blue,
            _ => TriangleColor::Teal,
        };
        let mut entity = match square.orientation {
            _ => {
                let texture_handle = my_assets.from_tcolor(color);
                commands.spawn((
                    Triangle { color },
                    SpriteBundle {
                        texture: texture_handle,
                        transform: transform
                            .clone()
                            .with_rotation(Quat::from_rotation_z(90f32.to_radians())),
                        ..default()
                    },
                ))
            }
        };
        entity.set_parent(id);
        let triangle_id = entity.id();
        commands.entity(id).add_child(triangle_id);
    }
}
/// rotates the parent, which will result in the child also rotating
fn rotate_field(mut query: Query<&mut Transform, With<Field>>) {
    for mut transform in &mut query {
        transform.rotate_z(45.0f32.to_radians());
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    data_path: Res<DataPath>,
    config: Res<Config>,
) {
    commands.spawn(Camera2dBundle::default());
    // It seems that path.join changes the path to assets if assets not cloned
    let bg_path = data_path.clone().join("background.png");
    if let Err(_) = fs::metadata(&bg_path) {
        println!("generating asset: {:?}", &bg_path);
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
            config.gradient.radius,
            window.physical_width() as u32,
            window.physical_height() as u32,
            config.gradient.color1.as_rgba_u8(),
            config.gradient.color2.as_rgba_u8(),
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
}
