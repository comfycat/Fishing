use bevy::{prelude::*, transform::commands, math::vec2, sprite::collide_aabb::collide, render::view::VisibleEntities};

use crate::Player;

pub struct UIPlugin {

}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct UIToggle{
    
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct UIParent{
    
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_systems(Startup, ui_setup)
        .add_systems(Update, ui_visibility)
        //.add_systems(Update, ui_player_interface)
        ;
    }
}

// Setup UI panel
fn ui_setup (
    mut commands: Commands,
    window: Query<&Window>,
) {
    // spawn the main rectangle which holds the stuff
    let parent = 
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(window.single().resolution.width() * 0.4, window.single().resolution.height())),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(window.single().resolution.width() * -0.3, 0., 0.1)),
            visibility: Visibility::Hidden,
            ..default()
        }, UIParent{},
        )).id();

    // spawn the buttons inside of the main rectangle
    let child = 
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0., 1.),
            custom_size: Some(Vec2::new(120.0, 60.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 120., 0.2 )),
        ..default()
    }).id();

    // add the child to the parent
    commands.entity(parent).push_children(&[child]);

    // spawn UI Toggle button
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 0., 0.),
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-270., 190., 0.2 )),
            ..default()
        },
        UIToggle {},
    ));
}

// Toggle UI Visibility
fn ui_visibility (
    commands: Commands,
    mouse_evr: Res<Input<MouseButton>>,
    mut toggler_query: Query<(Entity, &mut Transform, (With<UIToggle>, Without<Player>))>,
    mut characters: Query<(&mut Transform,  &mut Handle<Image>, &mut Player)>,
    mut ui_parent_query: Query<(&mut Visibility, With<UIParent>)>,
) {
    // Check to see if mouse was clicked
    if mouse_evr.just_pressed(MouseButton::Left) {
        // Check to see if the toggle button was hit
        // Iterate through the list of toggle
        let player_transform = characters.single_mut().0;
        for (_toggler_entity, toggler_transform, _empty) in &mut toggler_query {
            // If button hit toggle the UI visibility
            let collision = collide(
                player_transform.translation,
                player_transform.scale.truncate(),
                toggler_transform.translation,
                vec2(60.0, 60.0),
            );

            if collision.is_some() {
                // turn on UI visibility
                
                let new_visibility = match *ui_parent_query.single_mut().0 {
                    Visibility::Hidden => Visibility::Visible,
                    Visibility::Visible => Visibility::Hidden,
                    Visibility::Inherited => Visibility::Hidden,
                };
                *ui_parent_query.single_mut().0 = new_visibility;
            }
        }
    }
}

// Player button interface
fn ui_player_interface (

) {
    
}