use bevy::prelude::*;
use core::time::*;
use bevy::ecs::system::Command;
use bevy::ecs::schedule::{IntoRunCriteria, SingleThreadedExecutor};
use core::f32::consts::PI;
use crate::score::ScoreResource;

pub struct Monster;

pub struct SpawnTimer(pub Timer);

pub fn spawn_monster_up(mut commands: Commands,
                        materials: Res<crate::Materials>,
                        time: Res<Time>,
                        mut timer: ResMut<SpawnTimer>
)
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
        if transform.translation.y > 30. {
            transform.translation.y -= time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_down(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.y < -30. {
            transform.translation.y += time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_right(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.x > 30. {
            transform.translation.x -= time.delta_seconds() * 200.;
        }
    }
}

pub fn move_monster_left(time: Res<Time>, mut query: Query<(&mut Transform, &Monster)>){
    for (mut transform, _monster) in query.iter_mut() {
        if transform.translation.x < (-30.) {
            transform.translation.x += time.delta_seconds() * 200.;
        }
    }
}

pub fn despawns_monster_up(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut monster_positions: Query<(Entity, &mut Transform, &Monster), Without<crate::character::Character>>,
    character: Query<(Entity, &Transform, &crate::character::Character), Without<Monster>>,
    mut score: ResMut<crate::score::ScoreResource>)
{

    let (_entity, transform, _character) = character.single().expect("");
    let path = transform.rotation.z;
    for (entity, transform, _monster) in monster_positions.iter_mut() {
        let posx = transform.translation.x;
        let posy = transform.translation.y;
        if posx == 0. && 150. >= posy && posy >= 100.
            && path == (-0.70710677)
            && keyboard_input.just_pressed(KeyCode::O)
        {
            score.combo += 1;
            commands.entity(entity).despawn();
        }
        else if posy <= 50. && posy >= 0. && posx == 0. {
            score.combo = 0;
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawns_monster_down(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut monster_positions: Query<(Entity, &mut Transform, &Monster), Without<crate::character::Character>>,
    character: Query<(Entity, &Transform, &crate::character::Character), Without<Monster>>,
    mut score: ResMut<ScoreResource>,
){
    let (_entity, transform, _character) = character.single().expect("");
    let path = transform.rotation.z;
    for (entity, transform, _monster) in monster_positions.iter_mut() {

        let posx = transform.translation.x;
        let posy = transform.translation.y;
        if posx == 0. && -175. <= posy && posy <= -150. && path == (0.70710677)
            && keyboard_input.just_pressed(KeyCode::O) {
            score.combo += 1;
            commands.entity(entity).despawn();
        }
        else if posy >= -50. && posy <= 0. && posx == 0. {
            score.combo = 0;
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawns_monster_left(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut monster_positions: Query<(Entity, &mut Transform, &Monster), Without<crate::character::Character>>,
    character: Query<(Entity, &Transform, &crate::character::Character), Without<Monster>>
){
    let (_entity, transform, _character) = character.single().expect("");
    let path = transform.rotation.z;
    for (entity, transform, _monster) in monster_positions.iter_mut() {
        let posx = transform.translation.x;
        let posy = transform.translation.y;
        if posy == 0. && -175. <= posx && posx <= -150. && path == (1.)
            && keyboard_input.just_pressed(KeyCode::O) {
            score.combo += 1;
            commands.entity(entity).despawn();
        }
        else if posx >= -50. && posx <= 0. && posy == 0. {
            score.combo = 0;
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawns_monster_right(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut monster_positions: Query<(Entity, &mut Transform, &Monster), Without<crate::character::Character>>,
    character: Query<(Entity, &Transform, &crate::character::Character), Without<Monster>>
){
    let (_entity, transform, _character) = character.single().expect("");
    let path = transform.rotation.z;
    for (entity, transform, _monster) in monster_positions.iter_mut() {
        let posx = transform.translation.x;
        let posy = transform.translation.y;
        if posy == 0. && 175. >= posx && posx >= 150. && path == (0.)
            && keyboard_input.just_pressed(KeyCode::O) {
            score.combo += 1;
            commands.entity(entity).despawn();
        }
        else if posx <= 50. && posx >= 0. && posy == 0. {
                score.combo = 0;
                commands.entity(entity).despawn();
            }
        }
}
