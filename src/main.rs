mod Charactor;
mod LoadImg;
mod consts;
mod types;
mod monsters;


use bevy::{input::system::exit_on_esc_system, prelude::*};
use crate::LoadImg::ArrowsPlugin;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "SF".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_system(Charactor::keyboard_input_system.system())
        .add_plugin(ArrowsPlugin)
        .run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}