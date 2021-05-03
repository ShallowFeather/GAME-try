use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, Input};

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
impl Directions {

    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Left => [KeyCode::Left, KeyCode::A],
            Directions::Right => [KeyCode::Right, KeyCode::D],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    pub fn key_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Left => [KeyCode::Left, KeyCode::A],
            Directions::Right => [KeyCode::Right, KeyCode::D],
        };
      keys.iter().any(|code| input.pressed(*code))
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
        match self {
            Directions::Up => {400.},
            Directions::Down => {-400.},
            Directions::Left => {100.},
            Directions::Right => {100.},
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Speed {
    OneClick,
    MoreClick,
}

impl Speed {
    pub fn value(&self) -> f32 {
        200. * self.multipler()
    }

    pub fn multipler(&self) -> f32 {
        match self {
            Speed::OneClick => 1.,
            Speed::MoreClick => 1.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MonsterTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direciton: Directions,
}

impl MonsterTime {
    fn new(monster: &MonsterTimeToml) -> Self {
        let speed_value = monster.speed.value();
        Self {
            spawn_time: monster.click_time - (200./ speed_value) as f64,,
            speed: monster.speed,
            direciton: monster.direction,
        }
    }
}



pub struct MonsterTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}


