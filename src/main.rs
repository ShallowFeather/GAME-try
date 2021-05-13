use bevy::prelude::*;
use bevy::asset::AssetServer;
use crate::consts::AppState;


mod consts;
mod character;
mod monsters;

pub struct Materials {
    pub(crate) character_material: Handle<ColorMaterial>,
    pub(crate) monster_material: Handle<ColorMaterial>,
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
        //.add_system(Character::spawn_character.system())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(character::spawn_main.system()))
        .add_system(character::move_main.system())
        .insert_resource(monsters::SpawnTimer(Timer::from_seconds(1.0, true)))
        .add_system(monsters::spawn_monster_up.system())
        .add_system(monsters::spawn_monster_down.system())
        .add_system(monsters::spawn_monster_left.system())
        .add_system(monsters::spawn_monster_right.system())
        .add_system(monsters::move_monster_up.system())
        .add_system(monsters::move_monster_down.system())
        .add_system(monsters::move_monster_left.system())
        .add_system(monsters::move_monster_right.system())
        .add_system(monsters::despawns_monster_up.system())
        .add_system(monsters::despawns_monster_down.system())
        .add_system(monsters::despawns_monster_left.system())
        .add_system(monsters::despawns_monster_right.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let main_handle = asset_server.load("images/arrow_blue.png");
    let monster_handle = asset_server.load("images/monster.png");
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .insert_resource(Materials {
            character_material: materials.add(main_handle.into()),
            monster_material: materials.add(monster_handle.into()),
        });
}
