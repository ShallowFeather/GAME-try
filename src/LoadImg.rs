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

        let up_handle = asset_server.load("images/");
        let down_handle = asset_server.load("images/arrow_red.png");
        let right_handle = asset_server.load("images/");
        let left_handle = asset_server.load("images/");

        MainCharator {
            up_texture: materials.add(up_handle.into()),
            down_texture: materials.add(down_handle.into()),
            right_texture: materials.add(right_handle.into()),
            left_texture: materials.add(left_handle.into())
        }
    }
}

//space arrow
struct Arrow;

pub struct SpawnTimer(Timer);

pub fn spawn_arrows (commands: &mut Commands, materials: Res<MainCharator>, time: Res<Time>, mut timer: ResMut<SpawnTimer>,) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(-400., 0., 0., ));
    commands
        .spawn(SpriteBundle {
            material: materials.right_texture.clone(),
            sprite: Sprite::new(Vec2::new(140., 140.)),
            transform,
            ..Default::default()
        })
        .with(Arrow);
}

//move arrow
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * 200.;
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<MainCharator>()
            .add_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
            .add_system(spawn_arrows.system())
            .add_system(move_arrows.system());
    }
}