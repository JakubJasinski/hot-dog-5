use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::mouse::MouseMotion,
    prelude::*,
    text::FontSmoothing,
    window::PrimaryWindow,
};

struct Matrix {
    columns: usize,
    rows: usize,
}

#[derive(Component)]
struct Tile {
    x: f32,
    y: f32,
}

fn start_up(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
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
    q_windows: Query<&Window, With<PrimaryWindow>>,
    tiles: Query<(&mut MeshMaterial2d<ColorMaterial>, &Tile)>,
) {
    if let Ok(Some(cursor)) = q_windows.single().map(|c| c.cursor_position()) {
        if let Some((color, _)) = tiles.iter().find(|(_, tile)| {
            tile.x.floor() as u32 == cursor.x.floor() as u32
                && tile.y.floor() as u32 == cursor.y.floor() as u32
        }) {
            dbg!(color);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
