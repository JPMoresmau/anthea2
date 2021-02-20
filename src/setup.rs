
use bevy::{prelude::*, sprite::TextureAtlasBuilder,};
use crate::base::*;
use crate::tiled::*;
use crate::world::*;



pub fn setup_camera(commands: &mut Commands) {
    commands.spawn(OrthographicCameraBundle::new_2d())
        .with(MainCamera)
        .spawn(UiCameraBundle::default());

}

pub fn setup_map( commands: &mut Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    stage: Res<Area>,
    mut state: ResMut<AntheaState>,
    map_assets: Res<Assets<Map>>,
    tileset_assets: Res<Assets<TileSet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut appstate: ResMut<State<GameState>>,
){
    let map = map_assets.get(&sprite_handles.map_handles[stage.map_index]).unwrap();
    let ts = tileset_assets.get(&sprite_handles.tileset_handle).unwrap();

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.tile_handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }
    state.map_position=stage.start.clone();
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    
    let atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas = texture_atlases.get(atlas_handle.clone()).unwrap();
    for (ix,l) in map.layers.iter().enumerate(){
        let mut pos = state.map_position.clone();
        let mut c=0;
        for t in &l.tiles {
            if *t>0{
                let path = &ts.tiles[t-1];
                let tile_handle = asset_server.get_handle(path.as_str());
                let tile_index = texture_atlas.get_texture_index(&tile_handle).unwrap();
                let vec3 = pos.to_vec3();
                let vis= is_visible(&vec3,None);
                commands.spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(tile_index as u32),
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(vec3),
                    visible: Visible{is_transparent:true,is_visible:vis},
                    ..Default::default()
                }).with(MapTile);

                let rel_pos=state.map_position.to_relative(&pos);
                let e = state.positions.entry(rel_pos.clone()).or_default();
                e.entities.push(commands.current_entity().unwrap());
                let pass =  is_tile_passable(path);
                e.passable=e.passable&&pass;
                if ix==0 && !pass {
                    e.transparent=false;
                }
                if vis {
                    state.revealed.insert(rel_pos);
                }
            }
            c+=1;
            if c==l.width{
                c=0;
                pos.x=state.map_position.x;
                pos.y=pos.y-SPRITE_SIZE;
            } else {
                pos.x=pos.x+SPRITE_SIZE;
            }
        }
    }

    
    appstate.set_next(GameState::Start).unwrap();
    //println!("finished map");
    //println!("Revealed: {:?}",state.revealed);
}

pub fn setup_items(commands: &mut Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    stage: Res<Area>,
    state: Res<AntheaState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    ){
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.item_handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    
    let atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas = texture_atlases.get(atlas_handle.clone()).unwrap();   
    for item in stage.items.values(){
        let item_handle = asset_server.get_handle(item.sprite.as_str());
        let item_index = texture_atlas.get_texture_index(&item_handle).unwrap();
        let pos=Position{x:state.map_position.x+item.position.x,y:state.map_position.y-item.position.y}.to_vec3_z(0.3);
        let vis= is_visible(&pos,None);
        commands.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(item_index as u32),
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_translation(pos),
            visible: Visible{is_transparent:true,is_visible:vis},
            ..Default::default()
        })
        .with(item.clone());
    }
    
}

pub fn setup_people( commands: &mut Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
){
    //println!("starting people");
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.people_handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
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

    let atlas_handle = texture_atlases.add(texture_atlas);

    let pos = Position::default().to_vec3_z(0.3);
    commands
        .spawn((Player,))
        .with_children(|p| {
            p
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(body_index as u32),
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                })
                .with(PlayerPart{part:Part::BODY})
                .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(pants_index as u32),
                        texture_atlas: atlas_handle.clone(),
                        transform: Transform::from_translation(pos),
                        ..Default::default()}
                )
                .with(PlayerPart{part:Part::PANTS})
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(top_index as u32),
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(pos),
                    ..Default::default()}
                )
                .with(PlayerPart{part:Part::TOP})
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(hair_index as u32),
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(pos),
                    ..Default::default()}
                )
                .with(PlayerPart{part:Part::HAIR});
            })
        ;
        
}

pub fn is_visible(pos: &Vec3, ostate: Option<&AntheaState>) -> bool {
    if pos.x.abs()<VISIBILITY_DISTANCE && pos.y.abs()<VISIBILITY_DISTANCE as f32{
        if let Some(state) = ostate {
            let mut d_x = pos.x;
            let mut d_y = pos.y;
            while d_x!=0.0 || d_y!=0.0 {
                let mut n_x=d_x;
                if d_x!=0.0 && d_x.abs()>=d_y.abs() {
                    n_x=if d_x<0.0 {
                        d_x+SPRITE_SIZE as f32
                    } else {
                        d_x-SPRITE_SIZE as f32
                    };
                } 
                if d_y!=0.0 && d_x.abs()<=d_y.abs(){
                    d_y=if d_y<0.0 {
                        d_y+SPRITE_SIZE as f32
                    } else {
                        d_y-SPRITE_SIZE as f32
                    };
                }
                d_x=n_x;
            
                let pos =state.map_position.add(&Position::from_vec3(&Vec3::new(d_x,d_y,0.0)).inverse());
                if let Some(t) = state.positions.get(&pos){
                    if !t.transparent{
                        return false;
                    }
                }
            }
            return true;
        } else {
            return true;
        }
    }
    false
}