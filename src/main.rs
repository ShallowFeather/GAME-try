use bevy::prelude::*;
use bevy::asset::AssetServer;


//mod monsters;
mod character;

pub struct Materials {
    character_material: Handle<ColorMaterial>,
    monster_material: Handle<ColorMaterial>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let character_handle = asset_server.load("images/arrow_blue.png");
        let monster_handle = asset_server.load("images/arrow_red.png");
        Materials {
            character_material: material.add(character_handle.into()),
            monster_material: material.add(monster_handle.into()),
        }
    }
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
        .init_resource::<Materials>()
        .add_system(character::spawn_character.system())
        .add_plugins(DefaultPlugins)
        .run();
}


fn setup(mut commands: Commands, mut material: ResMut<Assets<ColorMaterial>>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
