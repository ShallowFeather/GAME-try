use bevy::prelude::*;


mod monsters;
mod character;

pub struct Materials {
    character_material: Handle<ColorMaterial>,
    monster_material: Handle<ColorMaterial>,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 500.,
            height: 300.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands, mut material: ResMut<Assets<ColorMaterial>>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        character_material: asset_server.load("images/arrow_blue.png"),
        monster_material: asset_server.load("images/monster.png"),
    });
}
