use bevy::input::{ eyboard::KeyCode, Input};
use bevy::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum Directions{
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Materials {
    main_material: Handle<ColorMaterial>,
}

pub struct Player {
    HP: i16,
    Path: Directions,
}

ub fn move_character(input: Res<Input<KeyCode>>, mut )

pub fn spawn_character(mut commands: Commands, material: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle{
            material: asset_server.load("images/arrow_blue.png"),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            ..Default::default()
        });
}