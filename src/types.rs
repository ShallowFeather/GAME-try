use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, ebug, PartialEq, Deserialize, Serialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
impl Directions {

    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCde::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Left => [KeyCode::Left, KeyCode::A],
            Directions::Right => [KeyCode::Right, KeyCode::D],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    pub fn key_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up  [KeyCode::Up, KeyCode::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Left => [KeyCode::Left, KeyCode::A],
            Directions::Right => [KeyCode::Right, KeyCode::D],
        };
        keys.iter().any(|code| input.just_pressed(*code))
    }

     fn x(&self) -> f32 {
        match self {
            Directions::Up => {100.},
            Directions::Down => {100.},
            Directions::Left => {-400.},
            Directions::Right => {400.},
        }
     }

    fn y(&self) -> f32 {
        match lf {
            Directions::Up => {400.},
            Directions::Down => {-400.},
            Directions::Left => {100.},
            Directions::Right => {100.},
        }
    }
}

