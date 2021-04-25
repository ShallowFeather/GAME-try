use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy::{
    input::{keyboard::KeyCode, Input},
};

pub fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {

    }

    if keyboard_input.just_pressed(KeyCode::A) {
        println!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        println!("A' just released");
    }
}
