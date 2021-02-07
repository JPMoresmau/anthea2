use bevy::{app::startup_stage, asset::{LoadState, SourceInfo}, diagnostic::EntityCountDiagnosticsPlugin, prelude::*, 
    render::pass, sprite::TextureAtlasBuilder,};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use std::{cmp::max, collections::{HashMap, HashSet}, intrinsics::transmute, time::{Instant}};

pub mod base;
use base::*;
pub mod tiled;
use tiled::*;
pub mod ui;
use ui::*;


fn main() {
    App::build()
        .init_resource::<AntheaHandles>()
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
        //.add_plugin(LogDiagnosticsPlugin::default())                                                                                                                                                                                                                    
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())                                                                                                                                                                                                                                                                                                                                                                                                                                     
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
            .insert_resource(MouseLocation::default())
            .insert_resource(MessageQueue::default())
            .insert_resource(State::new(AppState::Setup))
            .add_asset::<Map>()
            .init_asset_loader::<MapAssetLoader>()
            .add_asset::<TileSet>()
            .init_asset_loader::<TileSetAssetLoader>()
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .on_state_enter(STAGE, AppState::Setup, load_textures.system())
            .on_state_update(STAGE, AppState::Setup, check_textures.system())
            .on_state_enter(STAGE, AppState::Finished, setup_camera.system())
            .on_state_enter(STAGE, AppState::Finished, setup_map.system())
            .on_state_enter(STAGE, AppState::Finished, setup_people.system())
            .on_state_enter(STAGE, AppState::Finished, setup_ui.system())
            .add_system(player_movement_system.system())
            .add_system(cursor_system.system())
            .add_system(click_system.system())
            .add_system(message_system.system())
            //.add_system(visibility_system.system())
            ;
    }
}

fn load_textures(mut rpg_sprite_handles: ResMut<AntheaHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.people_handles = asset_server.load_folder("sprites/people").unwrap();
    rpg_sprite_handles.tiles_handles = asset_server.load_folder("sprites/tiles").unwrap();
    rpg_sprite_handles.tileset_handle = asset_server.load("anthea_tileset.tsx");
    rpg_sprite_handles.map_handles=vec![asset_server.load("castle1.tmx")];
    rpg_sprite_handles.ui_handle = asset_server.load("RPG_GUI_v1.png");
    rpg_sprite_handles.paper_handle = asset_server.load("paper background.png");
    rpg_sprite_handles.font_handle = asset_server.load("Breath Fire.otf");
}


fn check_textures(
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: ResMut<AntheaHandles>,
    asset_server: Res<AssetServer>,
) {
    let ls = asset_server.get_group_load_state(rpg_sprite_handles.people_handles.iter()
        .chain(rpg_sprite_handles.tiles_handles.iter())
        .map(|handle| handle.id)
        .chain(rpg_sprite_handles.map_handles.iter().map(|h| h.id))
        .chain(std::iter::once(rpg_sprite_handles.tileset_handle.id))
        .chain(std::iter::once(rpg_sprite_handles.ui_handle.id))
        .chain(std::iter::once(rpg_sprite_handles.paper_handle.id))
        .chain(std::iter::once(rpg_sprite_handles.font_handle.id))
    );
    if let LoadState::Loaded = ls {
        state.set_next(AppState::Finished).unwrap();
    }
}


struct MainCamera;

fn setup_camera(commands: &mut Commands) {
    commands.spawn(OrthographicCameraBundle::new_2d())
        .with(MainCamera)
        .spawn(UiCameraBundle::default());

}



fn setup_map( commands: &mut Commands,
    sprite_handles: Res<AntheaHandles>,
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

                let rel_pos=START_MAP_POSITION.to_relative(&pos);
                let e = state.positions.entry(rel_pos).or_default();
                e.entities.push(commands.current_entity().unwrap());
                let pass =  is_tile_passable(path);
                e.passable=e.passable&&pass;
                if ix==0 && !pass {
                    e.transparent=false;
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
}

fn setup_people( commands: &mut Commands,
    sprite_handles: Res<AntheaHandles>,
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
            state.map_position=new_pos;
            
            for (_tile, mut transform, mut vis) in &mut map_query.iter_mut() {
                transform.translation.x+=dif_x;
                transform.translation.y+=dif_y;
                if !vis.is_visible && is_visible(&transform.translation,Some(&state)){
                    vis.is_visible=true;
                }
            }
            
        }
    }
}

fn is_visible(pos: &Vec3, ostate: Option<&AntheaState>) -> bool {
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

fn cursor_system(
    // events to get cursor position
    mut cursor_moved_events: EventReader<CursorMoved>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
    mut location: ResMut<MouseLocation>,
) {
   
    // assuming there is exactly one main camera entity, so this is OK
    if let Some(camera_transform) = q_camera.iter().next(){

        for ev in cursor_moved_events.iter() {
            // get the size of the window that the event is for
            let wnd = wnds.get(ev.id).unwrap();
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = ev.position - size / 2.0;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            //println!("World coords: {}/{}", pos_wld.x, pos_wld.y);
            location.x=pos_wld.x;
            location.y=pos_wld.y;
        }

    }
    
}

fn click_system(mouse_button_input: Res<Input<MouseButton>>,
    location: Res<MouseLocation>,
    mut queue: ResMut<MessageQueue>,
    ) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("left mouse currently pressed as: {} {}",location.x,location.y);
        queue.messages.push(Message{contents:format!("{} {}",location.x,location.y), location:location.clone()});
    }
}


