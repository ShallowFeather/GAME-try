use bevy::prelude::*;
use core::time::*;
use bevy::ecs::system::Command;
use core::f32::consts::PI;
use bevy::ecs::schedule::IntoRunCriteria;

pub struct Monster;

pub struct SpawnTimer(pub Timer);

pub fn spawn_monster_up(mut commands: Commands,
                        materials: Res<crate::Materials>,
                        time: Res<Time>,
                        mut timer: ResMut<SpawnTimer>)
{
    if timer.0.tick(time.delta()).just_finished() {
        let transform = Transform::from_translation(Vec3::new(0., 450., 1.));
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.monster_material.clone(),
                sprite: Sprite::new(Vec2::new(100., 100.)),
                transform,
                ..Default::default()
            })
            .insert(Monster);
    }
}

pub fn spawn_monster_down(mut commands: Commands,
                          materials: Res<crate::Materials>,
                          time: Res<Time>,
                          mut timer: ResMut<SpawnTimer>)
{
    if timer.0.tick(time.delta()).just_finished() {
        let transform = Transform::from_translation(Vec3::new(0., -450., 1.));
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.monster_material.clone(),
                sprite: Sprite::new(Vec2::new(100., 100.)),
                transform,
                ..Default::default()
            })
            .insert(Monster);
    }
}

pub fn spawn_monster_left(mut commands: Commands,
                          materials: Res<crate::Materials>,
                          time: Res<Time>,
                          mut timer: ResMut<SpawnTimer>)
{
    if timer.0.tick(time.delta()).just_finished() {
        let transform = Transform::from_translation(Vec3::new(-700., 0., 1.));
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.monster_material.clone(),
                sprite: Sprite::new(Vec2::new(100., 100.)),
                transform,
                ..Default::default()
            })
            .insert(Monster);
    }
}

pub fn spawn_monster_right(mut commands: Commands,
                           materials: Res<crate::Materials>,
                           time: Res<Time>,
                           mut timer: ResMut<SpawnTimer>)
{
    if timer.0.tick(time.delta()).just_finished() {
        let transform = Transform::from_translation(Vec3::new(700., 0., 1.));
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.monster_material.clone(),
                sprite: Sprite::new(Vec2::new(100., 100.)),
                transform,
                ..Default::default()
            })
            .insert(Monster);
    }
}

pub fn move_monster_up(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.y > 100. {
            transform.translation.y -= time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_down(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.y < -100. {
            transform.translation.y += time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_right(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){ for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.x > 100. {
            transform.translation.x -= time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_left(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.x < (-100.) {
            transform.translation.x += time.delta_seconds() * 200.;
        }
    }
}

pub fn despawns_monster_right(mut commands: Commands,
                              keyboard_input: Res<Input<KeyCode>>,
                              mut monster_positions: Query<(Entity, &mut Transform, &Monster)>,
                              mut character: Query<&mut Transform, With<crate::character::character>>){
    let (a,mut b,  transform_a) = character.single().iter_mut();
    let mut a = transform_a.rotation.z;
    for (entity, transform, monster) in monster_positions.iter_mut() {
        let mut posx = transform_a;

        let posy = transform.translation.y;
        let mut path = transform.rotation.z;
        commands.entity(entity).despawn();
    }
}

