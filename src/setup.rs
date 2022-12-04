use crate::base::*;
use crate::tiled::*;
use crate::world::*;
use bevy::{prelude::*, sprite::TextureAtlasBuilder};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}

pub fn setup_map(
    commands: Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    stage: Res<Area>,
    mut state: ResMut<AntheaState>,
    map_assets: Res<Assets<Map>>,
    tileset_assets: Res<Assets<TileSet>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: ResMut<Assets<Image>>,
    mut appstate: ResMut<State<GameState>>,
) {
    state.map_position = stage.start.clone();
    do_setup_map(
        commands,
        sprite_handles,
        asset_server,
        //stage,
        state,
        map_assets,
        tileset_assets,
        texture_atlases,
        textures,
    );
    appstate.set(GameState::Start).unwrap();
}

pub fn do_setup_map(
    mut commands: Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    //stage: Res<Area>,
    mut state: ResMut<AntheaState>,
    map_assets: Res<Assets<Map>>,
    tileset_assets: Res<Assets<TileSet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let map = map_assets.get(&sprite_handles.map_handle).unwrap();
    let ts = tileset_assets.get(&sprite_handles.tileset_handle).unwrap();

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.tile_handles.iter() {
        let texture = textures.get(&handle.clone_weak().typed::<Image>()).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas = texture_atlases.get(&atlas_handle).unwrap();

    for (ix, l) in map.layers.iter().enumerate() {
        let mut pos = SpritePosition::new(0, 0);
        //let start = SpritePosition::new(state.map_position.x,state.map_position.y);
        let mut c = 0;
        for t in &l.tiles {
            if *t > 0 {
                let path = &ts.tiles[t - 1];
                let tile_handle = asset_server.get_handle(path.as_str());
                let tile_index = texture_atlas.get_texture_index(&tile_handle).unwrap();
                let mut rel_pos = pos.to_relative(&state.map_position);
                rel_pos.y = -rel_pos.y;
                let vec3 = rel_pos.to_vec3();
                let ec = commands
                    .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(tile_index),
                        texture_atlas: atlas_handle.clone(),
                        transform: Transform::from_translation(vec3),
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    })
                    .insert(MapTile(ix))
                    .id();

                let e = state.positions.entry(pos.clone()).or_default();
                e.entities.push(ec);
                let pass = is_tile_passable(path);
                e.passable = e.passable && pass;
                if ix == 0 && !pass {
                    e.transparent = false;
                }
            }
            c += 1;
            if c == l.width {
                c = 0;
                pos.x = 0;
                pos.y += 1;
            } else {
                pos.x += 1;
            }
        }
    }

    //println!("finished map");
    //println!("Revealed: {:?}",state.revealed);
}

pub fn setup_items(
    mut commands: Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    stage: Res<Area>,
    state: ResMut<AntheaState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.item_handles.iter() {
        let texture = textures.get(&handle.clone_weak().typed::<Image>()).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas = texture_atlases.get(&atlas_handle).unwrap();
    for item in stage.items.values() {
        let item_handle = asset_server.get_handle(item.sprite.as_str());
        let item_index = texture_atlas.get_texture_index(&item_handle).unwrap();
        let pos = SpritePosition {
            x: item.position.x - state.map_position.x,
            y: state.map_position.y - item.position.y,
        }
        .to_vec3_z(0.3);
        let vis = false; //is_visible(&pos,None);
        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(item_index),
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(pos),
                visibility: Visibility { is_visible: vis },
                ..Default::default()
            })
            .insert(item.clone());
    }

    let item_handle = asset_server.get_handle("sprites/items/help.png");
    let item_index = texture_atlas.get_texture_index(&item_handle).unwrap();
    let pos = Vec3::new(
        (-SCREEN_WIDTH / 2 + SPRITE_SIZE / 2) as f32,
        (SCREEN_HEIGHT / 2 - SPRITE_SIZE / 2) as f32,
        0.3,
    );
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(item_index),
            texture_atlas: atlas_handle,
            transform: Transform::from_translation(pos),
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(Help);
}

pub fn setup_body(
    mut commands: Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    //println!("starting people");
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.people_handles.iter() {
        let texture = textures.get(&handle.clone_weak().typed::<Image>()).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let body_handle = asset_server.get_handle("sprites/people/human_f.png");
    let body_index = texture_atlas.get_texture_index(&body_handle).unwrap();

    let hair_handle = asset_server.get_handle("sprites/people/fem_black.png");
    let hair_index = texture_atlas.get_texture_index(&hair_handle).unwrap();

    let pants_handle = asset_server.get_handle("sprites/people/pants_l_white.png");
    let pants_index = texture_atlas.get_texture_index(&pants_handle).unwrap();

    let top_handle = asset_server.get_handle("sprites/people/shirt_white1.png");
    let top_index = texture_atlas.get_texture_index(&top_handle).unwrap();

    let hand_handle = asset_server.get_handle("sprites/people/empty.png");
    let hand_index = texture_atlas.get_texture_index(&hand_handle).unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);

    let pos = Vec3::new(0.0, 0.0, 0.3);
    commands
        .spawn((Player, SpatialBundle::default()))
        .add_children(|p| {
            p.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(body_index),
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(PlayerPart::Body);
            let pos2 =  Vec3::new(0.0, 0.0, 0.4);
            p.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(pants_index),
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(pos2),
                ..Default::default()
            })
            .insert(PlayerPart::Pants);
            p.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(top_index),
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(pos2),
                ..Default::default()
            })
            .insert(PlayerPart::Top);
            p.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(hair_index),
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(pos2),
                ..Default::default()
            })
            .insert(PlayerPart::Hair);
            p.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(hand_index),
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(pos2),
                ..Default::default()
            })
            .insert(PlayerPart::RightHand);
        });
}

pub fn setup_people(
    mut commands: Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    stage: Res<Area>,
    state: Res<AntheaState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    //println!("starting people");
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.people_handles.iter() {
        let texture = textures.get(&handle.clone_weak().typed::<Image>()).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);

    let texture_atlas = texture_atlases.get(&atlas_handle).unwrap();
    for chr in stage.characters.values() {
        let chr_handle = asset_server.get_handle(chr.sprite.as_str());
        let chr_index = texture_atlas.get_texture_index(&chr_handle).unwrap();
        let pos = SpritePosition {
            x: chr.position.x - state.map_position.x,
            y: state.map_position.y - chr.position.y,
        }
        .to_vec3_z(0.3);
        let vis = false; //is_visible(&pos,None);
        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(chr_index),
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(pos),
                visibility: Visibility { is_visible: vis },
                ..Default::default()
            })
            .insert(chr.clone());
    }
}

pub fn is_visible(pos: &Vec3, ostate: Option<&AntheaState>) -> bool {
    if pos.x.abs() < VISIBILITY_DISTANCE && pos.y.abs() < VISIBILITY_DISTANCE as f32 {
        if let Some(state) = ostate {
            let mut d_x = pos.x;
            let mut d_y = pos.y;
            while d_x != 0.0 || d_y != 0.0 {
                let mut n_x = d_x;
                if d_x != 0.0 && d_x.abs() >= d_y.abs() {
                    n_x = if d_x < 0.0 {
                        d_x + SPRITE_SIZE as f32
                    } else {
                        d_x - SPRITE_SIZE as f32
                    };
                }
                if d_y != 0.0 && d_x.abs() <= d_y.abs() {
                    d_y = if d_y < 0.0 {
                        d_y + SPRITE_SIZE as f32
                    } else {
                        d_y - SPRITE_SIZE as f32
                    };
                }
                d_x = n_x;

                let pos = state
                    .map_position
                    .add(&SpritePosition::from_coords(d_x, -d_y));
                if let Some(t) = state.positions.get(&pos) {
                    if !t.transparent {
                        return false;
                    }
                }
            }
        }
        return true;
    }
    false
}
