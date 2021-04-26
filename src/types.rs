use crate::consts::*;
use bevy::input::{keyboard::KeyCode, Input};
use core::f32::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum PlayerAction {
    Move(Directions),
}

impl Directions {
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Right => [KeyCode::Right, KeyCode::D],
            Directions::Left => [KeyCode::Left, KeyCode::A],
        };
        keys.iter().any(|code| input.just_pressed((*code)))
    }

    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.,
        }
    }
}