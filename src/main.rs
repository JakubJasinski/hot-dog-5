use beavy_game::{
    cursor::CursorPlugin,
    sprite_animation::{AnimatedSprite, AnimatedSpritePlugin, TurnDirection},
};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    input::keyboard::KeyboardInput,
    prelude::*,
    text::FontSmoothing,
};

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct MovableByKeybord;

fn start_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((MainCamera, Camera2d));
    commands.spawn((
        AnimatedSprite::new(
            asset_server.load("dog.png"),
            texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::splat(32),
                1,
                2,
                None,
                None,
            )),
        ),
        MovableByKeybord,
        Transform::from_scale(Vec3::splat(6.0)),
    ));
}

fn move_by_keyboard(
    query: Query<(&mut Transform, &mut TurnDirection), With<MovableByKeybord>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let velocity: Vec2 = keyboard
        .get_pressed()
        .map(|k| match k {
            KeyCode::ArrowRight => Vec2 { x: 1., y: 0. },
            KeyCode::ArrowUp => Vec2 { x: 0., y: 1. },
            KeyCode::ArrowLeft => Vec2 { x: -1., y: 0. },
            KeyCode::ArrowDown => Vec2 { x: 0., y: -1. },
            _ => Vec2 { x: 0., y: 0. },
        })
        .sum::<Vec2>()
        .normalize_or_zero();
    for (mut entity_trans, mut turn_direction) in query {
        entity_trans.translation += Vec3::from((velocity, 0.));
        if velocity != (0., 0.).into() {
            *turn_direction = TurnDirection(velocity);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(CursorPlugin::<MainCamera>::new())
        .add_plugins(AnimatedSpritePlugin)
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
        .add_systems(FixedUpdate, move_by_keyboard)
        .run();
}
