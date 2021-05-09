use bevy::prelude::*;
use core::time::*;

pub struct Monster;

pub struct SpawnTimer(Timer);

pub fn spawn_monster_up(mut commands: Commands,
                        materials: Res<crate::Materials>,
) {

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

pub fn move_monster_up(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.y > 200. {
            transform.translation.y -= time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_down(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.y < -200. {
            transform.translation.y += time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_left(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.x > 200. {
            transform.translation.x -= time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_right(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.x < (-200.) {
            transform.translation.x += time.delta_seconds() * 200.;
        }
    }
}

