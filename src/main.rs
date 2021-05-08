use bevy::prelude::*;
use bevy::asset::AssetServer;
use crate::consts::AppState;


mod consts;
mod character;
mod monsters;

pub struct Materialsa {
    character_material: Handle<ColorMaterial>,
    monster_material: Handle<ColorMaterial>,
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
        //.add_system(character::spawn_character.system())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(character::spawn_main.system()))
        .add_system(character::move_main.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let main_handle = asset_server.load("images/arrow_blue.png");
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .insert_resource(character::characterMaterial {
            main_material: materials.add(main_handle.into())
        });
}
