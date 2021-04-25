use crate::consts::*;
use bevy::input::{keyboard::KeyCode, Input};
use core::f32::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    Click1,
    Click2,
}

impl Directions {
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Right => [KeyCode::Right, KeyCode::D],
            Directions::Left => [KeyCode::Left, KeyCode::A],
            Directions::Click1 => [KeyCode::J],
            Directions::Click2 => [KeyCode::K],
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

    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => 150.,
            Directions::Down => 50.,
            Directions::Right => -50.,
            Directions::Left => -150.,
            _ => {}
        }
    }

    pub fn x(&self) -> f32 {
        match self {
            Directions::Up => 80.,
            Directions::Down => 120.,
            Directions::Right => 100,
            Directions::Left => 100,
            _ => {},
        }
    }
}