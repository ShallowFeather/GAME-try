use bevy::input::{ keyboard::KeyCode, Input};
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use core::f32::consts::PI;

use crate::consts;
use crate::consts::AppState;


pub struct CharacterMaterials {
    character_material: Handle<ColorMaterial>,
}

impl FromWorld for CharacterMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let character_handle = asset_server.load("images/arrow_blue.png");
        CharacterMaterials {
            character_material: material.add(character_handle.into()),
        }
    }
}

pub fn spawn_character(mut commands: Commands, material: Res<CharacterMaterials>) {
    let material = material.character_material.clone();
    commands
        .spawn_bundle(SpriteBundle {
            material,
            sprite: Sprite::new(Vec2::new(250., 150.)),
            ..Default::default()
        });
}

pub struct  CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CharacterMaterials>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(spawn_character.system()),
            );
    }
}

pub fn keyboard_input_system(input: Res<Input<KeyCode>>, mut path: Query<&mut Transform>) {
    for mut transform in path.iter_mut() {
        if input.pressed(KeyCode::W) {
            transform.rotate(Quat::from_rotation_z(PI * 0.5));
        }
        else if input.pressed(KeyCode::S) {
            transform.rotate(Quat::from_rotation_z(PI* -0.5));
        }
        else if input.pressed(KeyCode::A) {
            transform.rotate(Quat::from_rotation_z(PI * 1.));
        }
        else if input.pressed(KeyCode::D) {
            transform.rotate(Quat::from_rotation_z(PI * 0.));
        }
    }
}