use beavy_game::{
    cursor::CursorPlugin,
    sprite_animation::{AnimatedSprite, AnimatedSpritePlugin, TurnDirection},
};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct MovableByKeybord;

#[derive(Event)]
struct CollisionEvent;

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct PickableObject;

fn spawn_objects(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    if spawn_timer.0.tick((time.delta())).just_finished() {
        commands.spawn((
            PickableObject,
            Transform::from_translation(Vec3::new(
                rand::random::<f32>() * window.resolution.width() - window.resolution.width() / 2.,
                rand::random::<f32>() * window.resolution.height()
                    - window.resolution.height() / 2.,
                0.0,
            )),
            Sprite::from_color(
                Color::srgb(
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                ),
                Vec2::splat(20.0),
            ),
        ));
    }
}

fn detect_collision(
    mut commands: Commands,
    mut collision_events: EventWriter<CollisionEvent>,
    query_player: Query<(&Transform, &Sprite), With<MovableByKeybord>>,
    mut query_boxes: Query<(Entity, &Transform, &Sprite), With<PickableObject>>,
    textures: Res<Assets<Image>>,
) {
    if let Ok((player_transform, player_sprite)) = query_player.single() {
        for (box_entity, box_transform, box_sprite) in query_boxes.iter_mut() {
            let player_size = player_sprite.custom_size.unwrap_or_else(|| {
                textures
                    .get(&player_sprite.image)
                    .map(|image| image.size().as_vec2())
                    .unwrap_or(Vec2::ZERO)
            }) * player_transform.scale.truncate();

            let box_size = box_sprite.custom_size.unwrap_or_else(|| {
                textures
                    .get(&box_sprite.image)
                    .map(|image| image.size().as_vec2())
                    .unwrap_or(Vec2::ZERO)
            }) * box_transform.scale.truncate();

            let collision_radius = (player_size.x + box_size.x) / 2.0;

            let distance = player_transform
                .translation
                .truncate()
                .distance(box_transform.translation.truncate());

            if distance < collision_radius {
                println!("Collision detected with a box!");
                collision_events.write(CollisionEvent);

                commands.entity(box_entity).despawn();
            }
        }
    }
}

fn handle_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut score: ResMut<Score>,
) {
    for _ in collision_events.read() {
        score.0 += 1;
        println!("Score: {}", score.0);
    }
}

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
            turn_direction.0 = velocity;
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
        .add_systems(Update, spawn_objects)
        .add_systems(Update, detect_collision)
        .add_systems(Update, handle_collision_events)
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(Score::default())
        .add_event::<CollisionEvent>()
        .run();
}
