use bevy::{prelude::*, state::commands};

#[derive(Component)]
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
    let shapes = [meshes.add(Circle::new(5.))];

    for shape in shapes.into_iter() {
        let color = Color::hsl(280., 1.0, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(2., 0., 0.),
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, start_up)
        .run();
}
