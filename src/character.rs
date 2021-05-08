use bevy::input::{ keyboard::KeyCode, Input};
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use core::f32::consts::PI;

use crate::consts;
use crate::consts::AppState;
use bevy::ecs::system::Command;


pub struct character;
pub struct characterMaterial {
    pub(crate) main_material: Handle<ColorMaterial>,
}

pub fn move_main(key_input: Res<Input<KeyCode>>, mut positions: Query<&mut Transform, With<character>>) {
    for mut transform in positions.iter_mut() {
        if key_input.just_pressed(KeyCode::W) {
            transform.rotation = Quat::from_rotation_z(0.5 * PI);
        }
        else if key_input.just_pressed(KeyCode::A) {
            transform.rotation = Quat::from_rotation_z(1. * PI);
        }
        else if key_input.just_pressed(KeyCode::S) {
            transform.rotation = Quat::from_rotation_z(1.5 * PI);
        }
        else if key_input.just_pressed(KeyCode::D) {
            transform.rotation = Quat::from_rotation_z(0. * PI);
        }
    }
}

pub fn spawn_main(mut command: Commands, material: ResMut<characterMaterial>) {
    command
        .spawn_bundle(SpriteBundle{
            material: material.main_material.clone(),
            sprite: Sprite::new(Vec2::new(250., 150.)),
            ..Default::default()
        })
        .insert(character);
}
