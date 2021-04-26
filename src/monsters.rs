use bevy::prelude::*;

pub struct Monster {
    body_texture: Handle<ColorMaterial>,
}

impl FromResources for Monster {
    fn from_resources(resources: &Resources) -> Self {
        let mut  materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get::<AssetServer>().unwrap();

        let monsters_body = asset_server.load("image/monster.jpg");

        Monster {
            body_texture: materials.add(monsters_body.into()),
        }
    }
}

struct Ghost;

pub struct SpawnTimer(Timer);

pub fn spawn_up(commands: &mut Commands, materials: Res<Monster>,
                time: Res<Time>, mut timer: Res<SpawnTimer>,) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(0., 400., 0.));
    commands
        .spawn(SpriteBundle {
            material: materials.red_texture.clone(),
            sprite: Sprite::new(Vec2::new(50., 50.)),//size
            transform,
            ..Default::default()
        })
        .with(Arrow);
}

pub fn spawn_down(commands: &mut Commands, materials: Res<Monster>,
                  time: Res<Time>, mut timer: Res<SpawnTimer>,) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    let transform = Transform::from_translation(Vec3::new(0., -400., 0.));
    commands
        .spawn(SpriteBundle {
            material: materials.red_texture.clone(),
            sprite: Sprite::new(Vec2::new(50., 50.)),//size
            transform,
            ..Default::default()
        })
        .with(Arrow);
}

pub fn spawn_left(commands: &mut Commands, materials: Res<Monster>,
                  time: Res<Time>, mut timer: Res<SpawnTimer>,) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(-400., 0., 0.));
    commands
        .spawn(SpriteBundle {
            material: materials.red_texture.clone(),
            sprite: Sprite::new(Vec2::new(50., 50.)),//size
            transform,
            ..Default::default()
        })
        .with(Arrow);
}

pub fn spawn_right(commands: &mut Commands, materials: Res<Monster>,
                   time: Res<Time>, mut timer: Res<SpawnTimer>,) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }
    let transform = Transform::from_translation(Vec3::new(400., 0., 0.));
    commands
        .spawn(SpriteBundle {
            material: materials.red_texture.clone(),
            sprite: Sprite::new(Vec2::new(50., 50.)),//size
            transform,
            ..Default::default()
        })
        .with(Arrow);
}

fn move_monster_up(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>){
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * BASE_SPEED;
    }
}

fn move_monster_down(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>){
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.y += time.delta_seconds() * BASE_SPEED;
    }
}

fn move_monster_left(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>){
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * BASE_SPEED;
    }
}

fn move_monster_right(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>){
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x -= time.delta_seconds() * BASE_SPEED;
    }
}

pub struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<Monster>()
            .add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            .add_system(spawn_up.system())
            .add_system(spawn_down.system())
            .add_system(spawn_left.system())
            .add_system(spawn_right.system())
            .add_system(move_monster_up.system())
            .add_system(move_monster_down.system())
            .add_system(move_monster_left.system())
            .add_system(move_monster_right.system());
    }
}