use std::{time::Duration, default};

use bevy::{prelude::{*, default}, core_pipeline::clear_color::ClearColorConfig};
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};
use rand::prelude::*;

pub struct FishPlugin {
    
}
#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Fish {
    velocity: Vec2<>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct FishRespawnTimer {
    pub respawn_timer: Timer,
}

impl Default for FishRespawnTimer{
    fn default() -> Self {
        Self { respawn_timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once) }
    }
}

impl Plugin for FishPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<FishRespawnTimer>()
        .register_type::<FishRespawnTimer>()
        .register_type::<Fish>()
        .add_systems(Update, fish_respawn_cycle)
        .add_systems(Update, swim_fish);
    }
}

fn fish_respawn_cycle(
    commands: Commands,
    asset_server: Res<AssetServer>,
    fish_query: Query<&mut Fish>,
    timer: Res<Time>,
    mut fish_respawn_timer: ResMut<FishRespawnTimer>,
) {
    // Check to see if there is any fish
    if fish_query.is_empty() {
        // If there is no fish, advance the respawn timer
        // tick the respawn timer
        fish_respawn_timer.respawn_timer.tick(timer.delta());
        
        // if the the timer is done, make a fish
        if fish_respawn_timer.respawn_timer.just_finished() {
            spawn_fish(commands, asset_server);
        }
    }
}

fn spawn_fish(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("fish.png");
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture,
            ..default()
        },

        Fish {velocity: Vec2::new((rand::random::<f32>() - 0.5) * 500.0, (rand::random::<f32>() - 0.5) * 500.0)},
        Name::new("Fish"),
    ));
}

fn swim_fish(
    mut fish_query: Query<(&mut Transform, &mut Fish)>,
    timer: Res<Time>,
    window: Query<&Window>,
) {
    // iterate through fish to make sure all fish swim
    for (mut fish_transform, mut fish) in fish_query.iter_mut() {
        // make sure the fish does not go out of bounds
        let fish_x = fish_transform.translation.x + fish.velocity.x * timer.delta().as_secs_f32();
        let fish_y = fish_transform.translation.y + fish.velocity.y * timer.delta().as_secs_f32();
        // top check
        if fish_y > window.single().resolution.height() / 2.0 {
            fish.velocity.y *= -1.0;
        }
        // bottom check
        if fish_y < window.single().resolution.height() / -2.0 {
            fish.velocity.y *= -1.0;
        }
        // left check
        if fish_x < window.single().resolution.width() / -2.0 {
            fish.velocity.x *= -1.0;
        }
        // right check
        if fish_x > window.single().resolution.width() / 2.0 {
            fish.velocity.x *= -1.0;
        }

        // translate fish by velocity
        fish_transform.translation.x += fish.velocity.x * timer.delta().as_secs_f32();
        fish_transform.translation.y += fish.velocity.y * timer.delta().as_secs_f32();
        
    }

    
}