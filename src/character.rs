use bevy::input::{ keyboard::KeyCode, Input};
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use core::f32::consts::PI;


pub struct Directions;

struct Materials {
    main_material: Handle<ColorMaterial>,
}


pub fn keyboard_input_system(input: Res<Input<KeyCode>>, mut path: Query<&mut Transform, With<Directions>>) {
    for mut transform in path.iter_mut() {
        if input.pressed(KeyCode::W) {
            transform.rotation.z = PI * 0.5;
        }
        else if input.pressed(KeyCode::S) {
            transform.rotation.z = PI * -0.5;
        }
        else if input.pressed(KeyCode::A) {
            transform.rotation.z = PI * 1.;
        }
        else if input.pressed(KeyCode::D) {
            transform.rotation.z = PI * 0.;
        }
    }
}


pub fn spawn_character(mut commands: Commands, material: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle{
            material: material.main_material.clone(),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            ..Default::default()
        })
        .insert(Directions);
}