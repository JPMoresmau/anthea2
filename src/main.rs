use bevy::{app::startup_stage, asset::{LoadState, SourceInfo}, diagnostic::EntityCountDiagnosticsPlugin, prelude::*, render::pass, sprite::TextureAtlasBuilder};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod tiled;
use tiled::*;
use std::{cmp::max, collections::{HashMap, HashSet}, time::{Instant}};

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

const SPRITE_SIZE: i32 = 32;


const START_MAP_POSITION: Position = Position{x:-4*SPRITE_SIZE,y:4*SPRITE_SIZE};

const VISIBILITY_DISTANCE: f32 = 4.0 * SPRITE_SIZE as f32;

const MOVE_DELAY: u128 = 200;


fn main() {
    App::build()
        .init_resource::<SpriteHandles>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Anthea's Quest".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            vsync: true,
            resizable: false,
            //mode: WindowMode::Fullscreen { use_size: false },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AntheaPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())                                                                                                                                                                                                                    
        .add_plugin(FrameTimeDiagnosticsPlugin::default())                                                                                                                                                                                                                                                                                                                                                                                                                                     
        .run();
}

const STAGE: &str = "app_state";

#[derive(Clone)]
enum AppState {
    Setup,
    Finished,
}

pub struct AntheaPlugin;

impl Plugin for AntheaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(AntheaState::default())
            .insert_resource(State::new(AppState::Setup))
            .add_asset::<Map>()
            .init_asset_loader::<MapAssetLoader>()
            .add_asset::<TileSet>()
            .init_asset_loader::<TileSetAssetLoader>()
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .on_state_enter(STAGE, AppState::Setup, load_textures.system())
            .on_state_update(STAGE, AppState::Setup, check_textures.system())
            .on_state_enter(STAGE, AppState::Finished, setup_map.system())
            .on_state_enter(STAGE, AppState::Finished, setup_people.system())
            .add_system(player_movement_system.system())
            //.add_system(visibility_system.system())
            ;
    }
}

#[derive(Default)]
struct SpriteHandles {
    people_handles: Vec<HandleUntyped>,
    tiles_handles: Vec<HandleUntyped>,
    tileset_handle: Handle<TileSet>,
    map_handles: Vec<Handle<Map>>,
}

fn load_textures(mut rpg_sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.people_handles = asset_server.load_folder("sprites/people").unwrap();
    rpg_sprite_handles.tiles_handles = asset_server.load_folder("sprites/tiles").unwrap();
    rpg_sprite_handles.tileset_handle = asset_server.load("anthea_tileset.tsx");
    rpg_sprite_handles.map_handles=vec![asset_server.load("castle1.tmx")];
}


fn check_textures(
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let ls = asset_server.get_group_load_state(rpg_sprite_handles.people_handles.iter()
        .chain(rpg_sprite_handles.tiles_handles.iter())
        .map(|handle| handle.id)
        .chain(rpg_sprite_handles.map_handles.iter().map(|h| h.id))
        .chain(std::iter::once(rpg_sprite_handles.tileset_handle.id))
    );
    if let LoadState::Loaded = ls {
        state.set_next(AppState::Finished).unwrap();
    }
}

struct AntheaState {
    //player_position: Position,
    map_position: Position,
    positions: HashMap<Position,TileEntityState>,
    last_move: u128,
}

impl Default for AntheaState {
    fn default() -> Self {
        Self {
           //player_position: Position::default(),
           map_position: START_MAP_POSITION,
           positions: HashMap::new(),
           last_move: 0,
        }
    }
}


struct TileEntityState {
    entities:Vec<Entity>,
    passable: bool,
}

impl Default for TileEntityState {
    fn default() -> Self {
        Self{entities:vec![], passable:true}
    }
}

struct PlayerPart {
    part: Part,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32,self.y as f32, 0.0)
    }

    pub fn from_vec3(v: &Vec3)-> Position {
        Position{x:v.x as i32, y:v.y as i32}
    }

    pub fn copy(&mut self, pos: &Position)  {
       self.x=pos.x;
       self.y=pos.y;
    }

    pub fn to_relative(&self, pos: &Position) ->Position {
        Position{x:self.x-pos.x,y:self.y-pos.y}
    }

    pub fn add(&self, pos: &Position) ->Position {
        Position{x:self.x+pos.x,y:self.y+pos.y}
    }

    pub fn inverse(&self) ->Position {
        Position{x:-self.x,y:-self.y}
    }

    pub fn distance(&self, pos: &Position) -> i32 {
        (self.x-pos.x).abs().max((self.y-pos.y).abs())
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct MapTile;

enum Part {
    BODY,
    PANTS,
    TOP,
    HAIR,
}

fn setup_map( commands: &mut Commands,
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<AntheaState>,
    map_assets: Res<Assets<Map>>,
    tileset_assets: Res<Assets<TileSet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
){
    let map = map_assets.get(&sprite_handles.map_handles[0]).unwrap();
    let ts = tileset_assets.get(&sprite_handles.tileset_handle).unwrap();

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprite_handles.tiles_handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }
        
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    
    let atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas = texture_atlases.get(atlas_handle.clone()).unwrap();
    for l in map.layers.iter(){
        let mut pos = state.map_position.clone();
        let mut c=0;
        for t in &l.tiles {
            if *t>0{
                let path = &ts.tiles[t-1];
                let tile_handle = asset_server.get_handle(path.as_str());
                let tile_index = texture_atlas.get_texture_index(&tile_handle).unwrap();
                let vec3 = pos.to_vec3();
                let vis= is_visible(&vec3);
                commands.spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(tile_index as u32),
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(vec3),
                    visible: Visible{is_transparent:true,is_visible:vis},
                    ..Default::default()
                }).with(MapTile);

                let rel_pos=pos.to_relative(&START_MAP_POSITION).inverse();
                let e = state.positions.entry(rel_pos).or_default();
                e.entities.push(commands.current_entity().unwrap());
                e.passable=e.passable&& is_tile_passable(path);
                
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
}

fn setup_people( commands: &mut Commands,
    sprite_handles: Res<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
){
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

    let pos = Position::default().to_vec3();

    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(UiCameraBundle::default())
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
        .with(PlayerPart{part:Part::HAIR})
        ;
        
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut state: ResMut<AntheaState>,
    mut map_query: Query<(&MapTile, &mut Transform, &mut Visible)>,
){
    state.last_move+=time.delta().as_millis();
    if state.last_move<MOVE_DELAY{
        return;
    }
    //let (mut pos,mut map) = (&mut (state.player_position),&mut state.map_position);
    for i in keyboard_input.get_pressed(){
        let mut new_pos = state.map_position.clone();
        match i {
            KeyCode::Right => new_pos.x-=SPRITE_SIZE,
            KeyCode::Left => new_pos.x+=SPRITE_SIZE,
            KeyCode::Up => new_pos.y-=SPRITE_SIZE,
            KeyCode::Down => new_pos.y+=SPRITE_SIZE,
            _ => (),
        }
        if new_pos!=state.map_position {
            if let Some(tes) = state.positions.get(&new_pos){
                if !tes.passable {
                    new_pos.copy(&state.map_position);
                }
            }
        }

        if new_pos!=state.map_position {
            state.last_move=0;
            let dif_x = (new_pos.x-state.map_position.x) as f32;
            let dif_y = (new_pos.y-state.map_position.y) as f32;
            
            for (_tile, mut transform, mut vis) in &mut map_query.iter_mut() {
                transform.translation.x+=dif_x;
                transform.translation.y+=dif_y;
                if is_visible(&transform.translation){
                    vis.is_visible=true;
                }
            }
            state.map_position=new_pos;
        }
    }
}

fn is_visible(pos: &Vec3) -> bool {
    pos.x.abs()<VISIBILITY_DISTANCE && pos.y.abs()<VISIBILITY_DISTANCE as f32
}




