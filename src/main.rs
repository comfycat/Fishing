use std::time::Duration;
mod fish;

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, input::common_conditions::input_toggle_active, window::PrimaryWindow, sprite::collide_aabb::collide, math::vec2};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, InspectorOptions, prelude::ReflectInspectorOptions};
use fish::{FishPlugin, Fish, FishRespawnTimer};

// This resource tracks the cat's fish
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct FishPoints(pub usize);

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player{
    click_timer: Timer,
}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Fishing".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
    )
    .init_resource::<FishPoints>()
    .add_plugins(
        (WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        FishPlugin {}),
    )
    .add_systems(Startup, setup)
    .add_systems(Update, cursor_events)
    .register_type::<FishPoints>()
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::ALICE_BLUE),
        },
        ..default()
    };
    
    commands.spawn(camera);
    
    let texture = asset_server.load("paw.png");
    let mut timer = Timer::new(Duration::from_secs_f32(0.25), TimerMode::Once);
    timer.pause();

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player {click_timer: timer},
        Name::new("Player"),
    ));   
}

fn cursor_events(
    mut commands: Commands,
    mut cursor_evr: EventReader<CursorMoved>,
    mut characters: Query<(&mut Transform,  &mut Handle<Image>, &mut Player)>,
    mut fish_query: Query<(Entity, &mut Transform, (With<Fish>, Without<Player>))>,
    mouse_evr: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    timer: Res<Time>,
    window: Query<&Window>,
    mut fish_respawn_timer: ResMut<FishRespawnTimer>,
    mut fish_points: ResMut<FishPoints>,
) {
    // Get most recent cursor event's coords
    if let Some(cursor_event) = cursor_evr.iter().last() {
        let cursor_x = cursor_event.position.x;
        let cursor_y = cursor_event.position.y;
        
        // Get player's transform
        let mut player_transform = characters.single_mut().0;

        // Update transform's position to coords
        player_transform.translation.x = cursor_x - window.single().resolution.width() / 2.0;
        player_transform.translation.y = (cursor_y - window.single().resolution.height() / 2.0) * -1.0;
    }

    // Check to see if mouse was clicked
    if mouse_evr.just_pressed(MouseButton::Left) {
        // Update sprite to sprite 2
        let texture = asset_server.load("paw2.png");
        *characters.single_mut().1 = texture;

        //restart and unpause timer
        characters.single_mut().2.click_timer.reset();
        characters.single_mut().2.click_timer.unpause();

        // Check to see if a fish was hit
        // Iterate through the list of fish
        let player_transform = characters.single_mut().0;
        for (fish_entity, fish_transform, empty) in &mut fish_query {
            // If fish hit give point and remove fish
            let collision = collide(
                player_transform.translation,
                player_transform.scale.truncate(),
                fish_transform.translation,
                vec2(100.0, 100.0),
            );

            if collision.is_some() {
                fish_points.0 += 1;
                
                // delete the fish
                commands.entity(fish_entity).despawn();
                fish_respawn_timer.respawn_timer.reset();

                // tell the world they got a fish
                info!("Fish Caught! Fish: {}", fish_points.0);
            }   
        }
    }

    // tick the click timer
    if !characters.single_mut().2.click_timer.paused() {
        characters.single_mut().2.click_timer.tick(timer.delta());
    }
    
    // if the the timer is done, set the sprite back
    if characters.single_mut().2.click_timer.just_finished() {
        let texture = asset_server.load("paw.png");
        *characters.single_mut().1 = texture;
    }
}

