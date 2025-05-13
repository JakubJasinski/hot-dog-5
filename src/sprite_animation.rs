use bevy::{prelude::*, time::Timer};

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
enum AnimationIndicesCount {
    Count(usize),
    NotInitialised,
}

#[derive(Component, Clone, Copy)]
pub struct TurnDirection(pub Vec2);

#[derive(Bundle)]
pub struct AnimatedSprite {
    timer: AnimationTimer,
    sprite: Sprite,
    index: AnimationIndicesCount,
    direction: TurnDirection,
}

impl AnimatedSprite {
    pub fn new(image: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Self {
        let sprite = Sprite::from_atlas_image(image, TextureAtlas { layout, index: 0 });
        let timer = AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating));
        let direction = TurnDirection((1., 0.).into());
        AnimatedSprite {
            timer,
            sprite,
            index: AnimationIndicesCount::NotInitialised,
            direction,
        }
    }
}

fn update_counts(
    query: Query<(&mut AnimationIndicesCount, &Sprite)>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    for (mut count, sprite) in query {
        if let Some(layout) = sprite
            .texture_atlas
            .as_ref()
            .and_then(|atlas| texture_atlas_layouts.get(atlas.layout.id()))
        {
            *count = AnimationIndicesCount::Count(layout.textures.len() - 1);
        }
    }
}

fn turn_sprite(query: Query<(&mut Sprite, &TurnDirection)>) {
    for (mut sprite, TurnDirection(direction)) in query {
        sprite.flip_x = direction.x > 0.;
    }
}

fn animate_sprite(
    time: Res<Time>,
    query: Query<(&mut AnimationTimer, &mut Sprite, &AnimationIndicesCount)>,
) {
    for (mut timer, mut sprite, count) in query {
        if let AnimationIndicesCount::Count(max_index) = count {
            timer.tick(time.delta());
            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index == *max_index {
                        0
                    } else {
                        atlas.index + 1
                    };
                }
            }
        }
    }
}

pub struct AnimatedSpritePlugin;
impl Plugin for AnimatedSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_counts, animate_sprite, turn_sprite).chain());
    }
}
