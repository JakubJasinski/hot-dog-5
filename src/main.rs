use bevy::prelude::*;

struct Matrix {
    columns: usize,
    rows: usize,
}

fn start_up(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let columns = 4;
    let rows = 3;
    let tiles = [0..rows].into_iter().map(|_| {
        [0..columns]
            .into_iter()
            .map(move |j| meshes.add(Rectangle::new(10., 10.)))
    });

    for (i, row) in tiles.into_iter().enumerate() {
        for (j, tile) in row.into_iter().enumerate() {
            let color = Color::hsl(280., 1.0, 0.7);
            commands.spawn((
                Mesh2d(tile),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz((i * 10) as f32, (j * 10) as f32, 0.),
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start_up)
        .run();
}
