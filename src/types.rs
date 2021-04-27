use crate::consts::*;
use bevy::input::{keyboard::KeyCode, Input};


#[derive(Copy, Clone, Debug, PartialEq)]
 pub enum Directions {
    Up,
    Down,
    Right,
    Left,
}

impl Directions {
    pub fn key_pressed (&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => {[KeyCode::Up, KeyCode::W]}
            Directions::Down => {[KeyCode::Down, KeyCode::S]}
            Directions::Right => {[KeyCode::Right, KeyCode::D]}
            Directions::Left => {[KeyCode::Left, KeyCode::A]}
        };
        keys.iter().any(|code| input.just_pressed(*code))
    }

    pub fn x(&self) -> f32{
        match self {
            Directions::Up => {0.}
            Directions::Down => {0.}
            Directions::Right => {400.}
            Directions::Left => {-400.}
        }
    }

    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => {400.}
            Directions::Down => {-400.}
            Directions::Right => {0}
            Directions::Left => {0}
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
impl Speed {
    pub  fn value(&self) -> f32 {
        200. * self.multiplier()
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => {1.}
            Speed::Medium => {1.2}
            Speed::Fast => {1.5}
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Monstertime {
    pub spawn_time: f64,
    pub speed: Speed
}

#[derive(Debug)]
pub struct SongConfig {
    pub monsters: Vec<Monstertime>
}

impl Monstertime {
    fn new(click_time: f64, speed: Speed) -> Self {
        let speed_value = speed.value();
        Self {
            spawn_time: click_time - (DISTANCE / speed_value) as f64,
            speed,
        }
    }
}

pub fn load_config() -> SongConfig {
    SongConfig {
        monsters: vec![
            Monstertime::new(1., Speed::Slow),
            Monstertime::new(2., Speed::Medium),
            Monstertime::new(3., Speed::Fast),
        ]
    }
}