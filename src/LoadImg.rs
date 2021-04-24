use bevy::prelude::*;

pub struct MainCharator {
    up_texture: Handle<ColorMaterial>,
    down_texture: Handle<ColorMaterial>,
    right_texture: Handle<ColorMaterial>,
    left_texture: Handle<ColorMaterial>,
}

impl FromResources for MainCharator {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();

        let up_handle = asset_server.load("");
        let down_handle = asset_server.load("");
        let right_handle = asset_server.load("");
        let left_handle = asset_server.load("");

        MainCharator {
            up_texture: materials.add(up_handle.into()),
            down_texture: materials.add(down_handle.into()),
            right_texture: materials.add(right_handle.into()),
            left_texture: materials.add(left_handle.into())
        }
    }
}