use bevy::prelude::*;
use bevy::asset::AssetServer;
use crate::consts::AppState;
use crate::character::CharacterPlugin;


//mod monsters;
mod character;
mod consts;

pub struct Materialsa {
    character_material: Handle<ColorMaterial>,
    monster_material: Handle<ColorMaterial>,
}

impl FromWorld for Materialsa {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let character_handle = asset_server.load("images/arrow_blue.png");
        let monster_handle = asset_server.load("images/arrow_red.png");
        Materialsa {
            character_material: material.add(character_handle.into()),
            monster_material: material.add(monster_handle.into()),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 300.,
            ..Default::default()
        })
        .add_state(AppState::Game)
        .add_plugin(CharacterPlugin)
        .run();
}


fn setup(mut commands: Commands, mut material: ResMut<Assets<ColorMaterial>>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

}
