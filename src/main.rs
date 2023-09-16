use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, render::camera::ScalingMode, input::common_conditions::input_toggle_active};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::ALICE_BLUE),
        },
        ..default()
    };
    
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);
}