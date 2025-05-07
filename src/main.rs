use std::marker::PhantomData;

use beavy_game::cursor::{CursorCoords, CursorPlugin};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

struct Matrix {
    columns: usize,
    rows: usize,
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Debug)]
struct Tile {
    x: f32,
    y: f32,
}

fn start_up(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((MainCamera, Camera2d));
    let columns = 20;
    let rows = 20;
    let mut tiles = vec![];
    for _ in 0..rows {
        let mut row = vec![];
        for _ in 0..columns {
            row.push(meshes.add(Rectangle::new(20., 20.)));
        }
        tiles.push(row);
    }

    for (i, row) in tiles.into_iter().enumerate() {
        for (j, tile) in row.into_iter().enumerate() {
            let color = Color::hsl((360 * (20 * i + j) / 420) as f32, 1.0, 0.7);
            let x = -100. + (i * 20) as f32;
            let y = -100. + (j * 20) as f32;
            commands.spawn((
                Tile { x, y },
                Mesh2d(tile),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(x, y, 0.),
            ));
        }
    }
}

fn mouse_motion(
    tiles: Query<(&mut MeshMaterial2d<ColorMaterial>, &Tile)>,
    cursor: Res<CursorCoords>,
) {
    if let Some((color, _)) = tiles.iter().find(|(_, tile)| {
        let in_x = tile.x <= cursor.0.x && cursor.0.x <= tile.x + 20.;
        let in_y = tile.y <= cursor.0.y && cursor.0.y <= tile.y + 20.;
        in_x && in_y
    }) {
        dbg!(color);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CursorPlugin::<MainCamera> {
            phantom: PhantomData,
        })
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 42.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                    ..default()
                },
                // We can also change color of the overlay
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
                ..Default::default()
            },
        })
        .add_systems(Startup, start_up)
        .add_systems(Update, mouse_motion)
        .run();
}
