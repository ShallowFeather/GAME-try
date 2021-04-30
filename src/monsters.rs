use bevy::prelude::*;
use crate::time::ControlledTime;
use crate::types::*;
use crate::ScoreResource;
use bevy::prelude::*;


struct  MonsterMaterial {
    one_click: Handle<ColorMaterial>,
    more_click: Handle<ColorMaterial>,
}

impl FromWorld for MonsterMaterial {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut material = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let one_hit = asset_server.load("images/monster.png");
        let more_hit = asset_server.load("images/monster.png");
        MonsterMaterial {
            one_click: material.add(one_hit.into()),
            more_click: (material.add(more_hit.into())),
        }
    }
}

pub struct MonsterPosition {
    x: f32,
    y: f32,
}

struct Monster {
    click: Click,
}

fn spawn_monster(mut commands: Commands, mut song_config: ResMut<SongConfig>, materials: Res<MonsterMaterial>, time: Res<ControlledTime>,) {
    let secs = time.seconds_since_startup() - 2;
    let secs_last = secs - time.delta_seconds_f64();
    let mut remove_counter = 0;
    for monster in &song_config.monsters {
        if secs_last < monster.spawn_time && monster.spawn_time < secs {
            remove_counter += 1;
            let material = match monster.speed {
                Speed::OneClick => materials.one_click.clone(),
                Speed::MoreClick => materials.more_click.clone(),
            };

            let mut transform = Transform::from_translation(Vec3::new())
        }
    }

}


