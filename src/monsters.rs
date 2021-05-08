use bevy::prelude::*;
use crate::time::ControlledTime;

pub struct Monster {
    direction: Direction,
    speed: Speed,
}


pub fn spawn_monster_up(mut commands: Commands, materials: Res<crate::Materials>) {
    let transform = Transform::from_translation(Vec3::new( 0.,450. , 1.));
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.monster_material.clone(),
            sprite: Sprite::new(Vec2::new(100., 100.)),
            transform,
            ..Default::default()
        })
        .insert(Monster);
}

pub fn spawn_monster_down(mut commands: Commands, materials: Res<crate::Materials>) {
    let transform = Transform::from_translation(Vec3::new(0.,-450. , 1.));
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.monster_material.clone(),
            sprite: Sprite::new(Vec2::new(100., 100.)),
            transform,
            ..Default::default()
        })
        .insert(Monster);
}

pub fn spawn_monster_left(mut commands: Commands, materials: Res<crate::Materials>) {
    let transform = Transform::from_translation(Vec3::new( -700.,0. , 1.));
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.monster_material.clone(),
            sprite: Sprite::new(Vec2::new(100., 100.)),
            transform,
            ..Default::default()
        })
        .insert(Monster);
}

pub fn spawn_monster_right(mut commands: Commands, materials: Res<crate::Materials>) {
    let transform = Transform::from_translation(Vec3::new( 700.,0. , 1.));
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.monster_material.clone(),
            sprite: Sprite::new(Vec2::new(100., 100.)),
            transform,
            ..Default::default()
        })
        .insert(Monster);
}

pub fn move_monster(time: Res<ControlledTime>, mut query: Query<(Entity, &Monster)>){
    
}
