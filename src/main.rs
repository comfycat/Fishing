use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, render::camera::ScalingMode, input::common_conditions::input_toggle_active, window::PrimaryWindow};
use bevy_inspector_egui::{quick::WorldInspectorPlugin, InspectorOptions, prelude::ReflectInspectorOptions};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player{
    
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
    .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    )
    .add_systems(Startup, setup)
    //.add_systems(Update, character_movement)
    .add_systems(Update, cursor_events)
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

    let texture = asset_server.load("character2.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player {},
        Name::new("Player"),
    ));
}

fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
    mut characters: Query<(&mut Transform, &Player)>,
    window: Query<&Window>,
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
} 