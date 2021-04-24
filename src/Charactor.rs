use bevy::input::keyboard::keyboard_input_system;
use bevy::input::Input;
use bevy::prelude::KeyCode;
use bevy::ecs::{Res, With, Query};
use std::f32::consts::PI;
use bevy::ui::PositionType;

pub fn keyboard_input_systems(keyboard_input: Res<Input<KeyCode>>, mut head_positions: Query<&mut Position, With<Head>>) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) { //UP

        }
        if keyboard_input.pressed(KeyCode::A) { //LEFT

        }
        if keyboard_input.pressed(KeyCode::S) { //DOWN

        }
        if keyboard_input.pressed(KeyCode::D){ //RIGHT

        }
    }

}
