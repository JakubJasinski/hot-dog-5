use beavy_game::cursor::{CursorCoords, CursorPlugin};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Debug)]
struct Tile {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct DefaultColor(Color);

fn start_up(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));
    let columns = 20;
    let rows = 20;

    for i in 0..rows {
        for j in 0..columns {
            let color = Color::hsl((360 * (20 * i + j) / 420) as f32, 1.0, 0.7);
            let x = -100. + (i * 20) as f32;
            let y = -100. + (j * 20) as f32;
            commands.spawn((
                Tile { x, y },
                Sprite::from_color(color, Vec2 { x: 20., y: 20. }),
                DefaultColor(color),
                Transform::from_xyz(x, y, 0.),
            ));
        }
    }
}

fn mouse_motion(mut tiles: Query<(&mut Sprite, &DefaultColor, &Tile)>, cursor: Res<CursorCoords>) {
    for (mut sprite, DefaultColor(color), tile) in &mut tiles {
        let Vec2 {
            x: width,
            y: height,
        } = sprite.custom_size.expect("We know it has a color!");
        let in_x = tile.x - (width / 2.) <= cursor.0.x && cursor.0.x <= tile.x + width / 2.;
        let in_y = tile.y - (height / 2.) <= cursor.0.y && cursor.0.y <= tile.y + height / 2.;
        if in_x && in_y {
            sprite.color = color.darker(0.2);
        } else {
            sprite.color = *color;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CursorPlugin::<MainCamera>::new())
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
                ..default()
            },
        })
        .add_systems(Startup, start_up)
        .add_systems(Update, mouse_motion)
        .run();
}
