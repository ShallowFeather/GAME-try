use bevy::input::{ eyboard::KeyCode, Input};
use bevy::prelude::*;


pub struct Directions;

struct Materials {
    main_material: Handle<ColorMaterial>,
}

pub fn move_character(input: Res<Input<KeyCode>>, mut path: Query<(&Directions, &mut Transform)>) {
    for mut transform in path.iter_mut() {
        if input.pressed(KeyCode::W) {
            transform
        }
        else if input.pressed(KeyCode::S) {

        }
        else if input.pressed(KeyCode::A) {

        }
        else if input.pressed(KeyCode::D) {

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