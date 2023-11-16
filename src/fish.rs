use std::{time::Duration, default};

use bevy::{prelude::{*, default}, core_pipeline::clear_color::ClearColorConfig};
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};

pub struct FishPlugin {
    
}
#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Fish {
    
}

#[derive(Resource)]
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
        .add_systems(Update, fish_respawn_cycle);
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
        Fish {},
        Name::new("Fish"),
    ));
}