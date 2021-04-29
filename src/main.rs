use bevy::prelude::*;
use bevy::render::render_graph::base::node::CAMERA_2D;
use bevy::render::camera::Camera;

mod monsters;
mod types;

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,) {
    let normal_texture = textures.get("images/monster.png");
    let mut camera = Camera::default();
    commands.spawn_bundle((
        // position the camera like bevy would do by default for 2D:
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GlobalTransform::default(),
        camera,
    ));
}