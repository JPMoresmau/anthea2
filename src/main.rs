use bevy::{asset::{LoadState}, prelude::*, };

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};


pub mod base;
use base::*;
pub mod menu;
use menu::*;
pub mod setup;
use setup::*;
pub mod tiled;
use tiled::*;

pub mod ui;
use ui::*;
pub mod world;
use world::*;

pub mod stages;
use stages::castle::*;


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



pub struct AntheaPlugin;

impl Plugin for AntheaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(AntheaState::default())
            .insert_resource(MouseLocation::default())
            .insert_resource(State::new(GameState::Setup))
            .add_event::<AffordanceEvent>()
            .add_plugin(CastlePlugin)
            .add_asset::<Map>()
            .init_asset_loader::<MapAssetLoader>()
            .add_asset::<TileSet>()
            .init_asset_loader::<TileSetAssetLoader>()
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<GameState>::default())
            .on_state_enter(STAGE, GameState::Setup, load_textures.system())
            .on_state_update(STAGE, GameState::Setup, check_textures.system())
            .on_state_enter(STAGE, GameState::Background, setup_camera.system())
            .on_state_enter(STAGE, GameState::Background, setup_map.system())
            .on_state_enter(STAGE, GameState::Start, setup_items.system())
            .on_state_enter(STAGE, GameState::Start, setup_people.system())
            .on_state_enter(STAGE, GameState::Start, setup_ui.system())
            .on_state_update(STAGE, GameState::Start, start_system.system())
            .on_state_update(STAGE, GameState::Running, player_movement_system.system())
            .on_state_update(STAGE, GameState::Running, cursor_system.system())
            .on_state_update(STAGE, GameState::Running, click_system.system())
            .add_plugin(MenuPlugin)
            .add_plugin(UIPlugin)
            //.add_system(visibility_system.system())
            ;
    }
}

fn load_textures(mut rpg_sprite_handles: ResMut<AntheaHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.people_handles = asset_server.load_folder("sprites/people").unwrap();
    rpg_sprite_handles.tile_handles = asset_server.load_folder("sprites/tiles").unwrap();
    rpg_sprite_handles.item_handles = asset_server.load_folder("sprites/items").unwrap();
    rpg_sprite_handles.tileset_handle = asset_server.load("anthea_tileset.tsx");
    rpg_sprite_handles.map_handles=vec![asset_server.load("castle1.tmx")];
    rpg_sprite_handles.ui_handle = asset_server.load("RPG_GUI_v1.png");
    rpg_sprite_handles.paper_handle = asset_server.load("paper background.png");
    rpg_sprite_handles.font_handle = asset_server.load("GRECOromanLubedWrestling.ttf");
}


fn check_textures(
    mut state: ResMut<State<GameState>>,
    rpg_sprite_handles: ResMut<AntheaHandles>,
    asset_server: Res<AssetServer>,
) {
    let ls = asset_server.get_group_load_state(rpg_sprite_handles.people_handles.iter()
        .chain(rpg_sprite_handles.tile_handles.iter())
        .chain(rpg_sprite_handles.item_handles.iter())
        .map(|handle| handle.id)
        .chain(rpg_sprite_handles.map_handles.iter().map(|h| h.id))
        .chain(std::iter::once(rpg_sprite_handles.tileset_handle.id))
        .chain(std::iter::once(rpg_sprite_handles.ui_handle.id))
        .chain(std::iter::once(rpg_sprite_handles.paper_handle.id))
        .chain(std::iter::once(rpg_sprite_handles.font_handle.id))
    );
    if let LoadState::Loaded = ls {
        state.set_next(GameState::Background).unwrap();
    }
}


fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut state: ResMut<AntheaState>,
    stage: Res<Area>,
    mut sprite_query: Query<(&mut Transform, &mut Visible),Or<(With<MapTile>,With<Item>)>>,
    mut msg: ResMut<Events<ClearMessage>>,
    mut ev_affordance: ResMut<Events<AffordanceEvent>>,
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
            msg.send(ClearMessage);
        }
        if new_pos!=state.map_position {

            state.last_move=0;

            let rel_x = -(new_pos.x as f32);
            let rel_y = new_pos.y as f32 ;

            if let Some(a) = stage.affordance_from_coords(rel_x,rel_y){
                // println!("Affordance: {}",a.name);
                ev_affordance.send(AffordanceEvent(a.name.clone()));
            } else {
                if let Some(i) = stage.item_from_coords(rel_x,rel_y){
                    println!("Item: {}",i.name);
                }
                let dif_x = (new_pos.x-state.map_position.x) as f32;
                let dif_y = (new_pos.y-state.map_position.y) as f32;
                state.map_position=new_pos;
                
                for ( mut transform, mut vis) in &mut sprite_query.iter_mut() {
                    transform.translation.x+=dif_x;
                    transform.translation.y+=dif_y;
                    if !vis.is_visible && is_visible(&transform.translation,Some(&state)){
                        vis.is_visible=true;
                        let pos = state.map_position.to_relative(&Position::new(transform.translation.x as i32, transform.translation.y as i32));
                        //println!("Revealing: {:?}",pos);
                        state.revealed.insert(pos);
                        
                    }
                }
               
            }
            
        }
    }
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

fn start_system(mouse_button_input: Res<Input<MouseButton>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    mut appstate: ResMut<State<GameState>>,
    ) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        clearm.send(ClearMessage); 
        appstate.set_next(GameState::Running).unwrap();
    }
}

fn click_system(mouse_button_input: Res<Input<MouseButton>>,
    location: Res<MouseLocation>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    state: Res<AntheaState>,
    stage: Res<Area>,
    mut appstate: ResMut<State<GameState>>,
    ) {
    if mouse_button_input.just_pressed(MouseButton::Left) {

        //println!("left mouse currently pressed as: {} {}",location.x,location.y);

        let rel_x = location.x-(state.map_position.x as f32);
        let rel_y = -(location.y-(state.map_position.y as f32)) ;
        //println!("relative: {:?},{:?}",rel_x,rel_y);

        let rel_pos=Position::new(-rel_x as i32,rel_y as i32);
        let mut revealed = false;
        for rp in state.revealed.iter() {
            if rp.distance(&rel_pos)<=SPRITE_SIZE/2{
                revealed=true;
                break;
            }
        }
        if revealed {
            if location.x.abs()<=SPRITE_SIZE as f32/2.0 && location.y.abs()<=SPRITE_SIZE as f32/2.0 {
                println!("click on center");
                appstate.set_next(GameState::Menu).unwrap();
                /*queue.send(MessageEvent::new_multi(vec![
                    Message::new("Journal",MessageStyle::Interaction),
                    Message::new("Inventory",MessageStyle::Interaction),
                    Message::new("Talents",MessageStyle::Interaction),
                ]));*/
            } else if let Some(a) = stage.affordance_from_coords(rel_x,rel_y){
                queue.send(MessageEvent::new(&a.description, MessageStyle::Info));
            } else if let Some(i) = stage.item_from_coords(rel_x,rel_y){
                queue.send(MessageEvent::new(&i.description, MessageStyle::Info));
            } else if let Some(r) = stage.room_from_coords(rel_x,rel_y){
                queue.send(MessageEvent::new(&r.description, MessageStyle::Info));
            } else {
                clearm.send(ClearMessage);
            }
        } else {
            clearm.send(ClearMessage);
        }

         
     }
}


