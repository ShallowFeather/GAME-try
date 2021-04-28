use bevy::prelude::*;

struct SpriteSheets {
    map_tiles: Handle<TextureAtlas>,
}

fn use_sprites(
    handles: Res<SpriteSheets>,
    atlases: Res<Assets<TextureAtlas>>,
    textures: Res<Assets<Texture>>,
) {
    // Could be `None` if the asset isn't loaded yet
    if let Some(atlas) = atlases.get(&handles.map_tiles) {
        // do something with the texture atlas
    }

    // Can use a path instead of a handle
    if let Some(map_tex) = textures.get("map.png") {
        // if "map.png" was loaded, we can use it!
    }
}

pub struct MonsterPosition {
    x: f32,
    y: f32,
}

struct Monster;

struct Monstermer(Timer);

fn spawn_up(commands: &mut Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>） {
    let transform_UP = Transform::from_translation(Vec2::new(-400., 0.));
    let transform_DOWN = Transform::from_translation(Vec2::new(400., 0. ));
}


