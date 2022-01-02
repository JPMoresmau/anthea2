use bevy::{asset::LoadState, prelude::*};

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
            .insert_resource(Journal::default())
            .insert_resource(Inventory::default())
            .insert_resource(Talents::default())
            .insert_resource(QuestFlags::default())
            .insert_resource(Spells::default())
            .insert_resource(EventMemory::default())
            .add_event::<AffordanceEvent>()
            .add_event::<CharacterEvent>()
            .add_event::<ItemEvent>()
            .add_event::<BodyChangeEvent>()
            .add_event::<JournalEvent>()
            .add_event::<RemoveTileEvent>()
            .add_plugin(CastlePlugin)
            .add_asset::<Map>()
            .init_asset_loader::<MapAssetLoader>()
            .add_asset::<TileSet>()
            .init_asset_loader::<TileSetAssetLoader>()
            .add_state(GameState::Setup)
            .add_system_set(SystemSet::on_enter(GameState::Setup).with_system(load_assets.system()))
            .add_system_set(SystemSet::on_update(GameState::Setup).with_system(check_assets.system()))
            .add_system_set(SystemSet::on_enter(GameState::Background).with_system(setup_camera.system()))
            .add_system_set(SystemSet::on_update(GameState::Title).with_system(setup_ui.system()))
            .add_system_set(SystemSet::on_enter(GameState::Background).with_system(setup_map.system()))
            .add_system_set(SystemSet::on_enter(GameState::Start).with_system(setup_items.system()))
            .add_system_set(SystemSet::on_enter(GameState::Start).with_system(setup_body.system()))
            .add_system_set(SystemSet::on_enter(GameState::Start).with_system(setup_people.system()))
            .add_system_set(SystemSet::on_update(GameState::Start).with_system(start_system.system()))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(player_movement_system.system()))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(cursor_system.system()))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(click_system.system()))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(pickup_item.system()))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(body_change.system()))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(journal.system()))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system( remove_tile.system()))
            .add_plugin(MenuPlugin)
            .add_plugin(UIPlugin)
            //.add_system(visibility_system.system())
            ;
    }
}

fn load_assets(mut rpg_sprite_handles: ResMut<AntheaHandles>, asset_server: Res<AssetServer>) {
    println!("Loading assets...");
    rpg_sprite_handles.people_handles = asset_server.load_folder("sprites/people").unwrap();
    rpg_sprite_handles.tile_handles = asset_server.load_folder("sprites/tiles").unwrap();
    rpg_sprite_handles.item_handles = asset_server.load_folder("sprites/items").unwrap();
    rpg_sprite_handles.tileset_handle = asset_server.load("anthea_tileset.tsx");
    rpg_sprite_handles.map_handles = vec![asset_server.load("castle1.tmx")];
    rpg_sprite_handles.ui_handle = asset_server.load("RPG_GUI_v1.png");
    rpg_sprite_handles.paper_handle = asset_server.load("paper background.png");
    rpg_sprite_handles.font_handle = asset_server.load("GRECOromanLubedWrestling.ttf");
    rpg_sprite_handles.sound_handles = asset_server.load_folder("sounds").unwrap();
}

fn check_assets(
    mut state: ResMut<State<GameState>>,
    rpg_sprite_handles: ResMut<AntheaHandles>,
    asset_server: Res<AssetServer>,
) {
    let ls = asset_server.get_group_load_state(
        rpg_sprite_handles
            .people_handles
            .iter()
            .chain(rpg_sprite_handles.tile_handles.iter())
            .chain(rpg_sprite_handles.item_handles.iter())
            .chain(rpg_sprite_handles.sound_handles.iter())
            .map(|handle| handle.id)
            .chain(rpg_sprite_handles.map_handles.iter().map(|h| h.id))
            .chain(std::iter::once(rpg_sprite_handles.tileset_handle.id))
            .chain(std::iter::once(rpg_sprite_handles.ui_handle.id))
            .chain(std::iter::once(rpg_sprite_handles.paper_handle.id))
            .chain(std::iter::once(rpg_sprite_handles.font_handle.id)),
    );
 
    if let LoadState::Loaded = ls {
        println!("Assets loaded");
        state.set(GameState::Title).unwrap();
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut state: ResMut<AntheaState>,
    stage: ResMut<Area>,
    mut sprite_query: Query<
        (&mut Transform, &mut Visible),
        Or<(With<MapTile>, With<Item>, With<Character>)>,
    >,
    mut msg: EventWriter<ClearMessage>,
    mut ev_affordance: EventWriter<AffordanceEvent>,
    mut ev_character: EventWriter<CharacterEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    
    //let (mut pos,mut map) = (&mut (state.player_position),&mut state.map_position);
    for i in keyboard_input.get_pressed() {
        state.last_move += time.delta().as_millis();
        if state.last_move < MOVE_DELAY {
            return;
        }
    
        let mut new_pos = state.map_position.clone();
        match i {
            KeyCode::Right => new_pos.x -= SPRITE_SIZE,
            KeyCode::Left => new_pos.x += SPRITE_SIZE,
            KeyCode::Up => new_pos.y -= SPRITE_SIZE,
            KeyCode::Down => new_pos.y += SPRITE_SIZE,
            _ => (),
        }
        if new_pos != state.map_position {
            if let Some(tes) = state.positions.get(&new_pos) {
                if !tes.passable {
                    new_pos.copy(&state.map_position);
                }
            }
            msg.send(ClearMessage);
        }
        if new_pos != state.map_position {
            state.last_move = 0;

            let rel_x = -(new_pos.x as f32);
            let rel_y = new_pos.y as f32;

            let sprite_position=SpritePosition::from_coords(rel_x,rel_y);
            if let Some(a) = stage.affordance_from_position(&sprite_position) {
                // println!("Affordance: {}",a.name);
                ev_affordance.send(AffordanceEvent(a.name.clone()));
            } else if let Some(c) = stage.character_from_position(&sprite_position) {
                //println!("Character: {}",c.name);
                ev_character.send(CharacterEvent(c.name.clone()));
            } else {
                audio.play(asset_server.get_handle("sounds/steps.ogg"));
                let dif_x = (new_pos.x - state.map_position.x) as f32;
                let dif_y = (new_pos.y - state.map_position.y) as f32;
                state.map_position = new_pos;

                for (mut transform, mut vis) in &mut sprite_query.iter_mut() {
                    transform.translation.x += dif_x;
                    transform.translation.y += dif_y;
                    if !vis.is_visible && is_visible(&transform.translation, Some(&state)) {
                        vis.is_visible = true;
                        let pos = state.map_position.to_relative(&Position::new(
                            transform.translation.x as i32,
                            transform.translation.y as i32,
                        ));
                        //println!("Revealing: {:?}",pos);
                        state.revealed.insert(pos);
                    }
                }
            }
        }
    }
}

fn pickup_item(
    mut commands: Commands,
    state: Res<AntheaState>,
    mut inventory: ResMut<Inventory>,
    item_query: Query<(Entity, &Item)>,
    mut stage: ResMut<Area>,
    mut queue: EventWriter<MessageEvent>,
    mut item_queue: EventWriter<ItemEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some(i) = stage.items.remove(&state.map_position.inverse_x().into()) {
        //println!("Item: {}",i.name);
        for (e, _i2) in item_query.iter().filter(|(_e, i2)| i.name == i2.name) {
            commands.entity(e).despawn_recursive();
        }
        audio.play(asset_server.get_handle("sounds/pickup.ogg"));
        if i.consumable {
            //queue.send(MessageEvent::new(format!("{} consumed",i.description), MessageStyle::Info));
            item_queue.send(ItemEvent(i.name));
        } else {
            queue.send(MessageEvent::new(
                format!("{} picked up", i.description),
                MessageStyle::Info,
            ));
            inventory.add_item(i);
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
    if let Some(camera_transform) = q_camera.iter().next() {
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
            location.coords=Some((pos_wld.x,pos_wld.y));
        }
    }
}

fn start_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut clearm: EventWriter<ClearMessage>,
    mut appstate: ResMut<State<GameState>>,
    mut state: ResMut<AntheaState>,
    mut sprite_query: Query<
        (&Transform, &mut Visible),
        (
            Without<Help>,
            Or<(With<MapTile>, With<Item>, With<Character>)>,
        ),
    >,
    mut help_query: Query<&mut Visible, With<Help>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        clearm.send(ClearMessage);
        appstate.set(GameState::Running).unwrap();
        for (transform, mut vis) in &mut sprite_query.iter_mut() {
            if !vis.is_visible && is_visible(&transform.translation, Some(&state)) {
                vis.is_visible = true;
                let pos = state.map_position.to_relative(&Position::new(
                    transform.translation.x as i32,
                    transform.translation.y as i32,
                ));
                //println!("Revealing: {:?}",pos);
                state.revealed.insert(pos);
            }
        }
        for mut vis in &mut help_query.iter_mut() {
            vis.is_visible = true;
        }
    }
}

fn click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut location: ResMut<MouseLocation>,
    mut queue: EventWriter<MessageEvent>,
    mut clearm: EventWriter<ClearMessage>,
    state: Res<AntheaState>,
    stage: Res<Area>,
    mut menu: EventWriter<MenuEvent>,
) {
    let pressed= mouse_button_input.just_pressed(MouseButton::Left);
    if !pressed {
        return;
    }
    if let Some((x,y)) = location.coords {
        //println!("left mouse currently pressed as: {} {}",x,y);
        if x < -SCREEN_WIDTH as f32 / 2.0 + SPRITE_SIZE as f32
            && y > SCREEN_HEIGHT as f32 / 2.0 - SPRITE_SIZE as f32
        {
            location.coords=None;
            if pressed {
                menu.send(MenuEvent::new(system_menu()));
            } else {
                queue.send(MessageEvent::new("System menu", MessageStyle::Info));
            }
            return;
        }

        let rel_x = x - (state.map_position.x as f32);
        let rel_y = -(y - (state.map_position.y as f32));
        //println!("relative: {:?},{:?}",rel_x,rel_y);

        let rel_pos = Position::new(-rel_x as i32, rel_y as i32);
        let mut revealed = false;
        for rp in state.revealed.iter() {
            if rp.distance(&rel_pos) <= SPRITE_SIZE / 2 {
                revealed = true;
                break;
            }
        }

        let sprite_position=SpritePosition::from_coords(rel_x,rel_y);
        //println!("sprite pos: {:?},{:?}",(rel_x/SPRITE_SIZE as f32).round() as i32 ,(rel_y/SPRITE_SIZE as f32).round() as i32);

        if revealed {
            if x.abs() <= SPRITE_SIZE as f32 / 2.0
                && y.abs() <= SPRITE_SIZE as f32 / 2.0
            {
                location.coords=None;
                //println!("click on center");
                //appstate.set_next(GameState::Menu).unwrap();
                if pressed {
                    menu.send(MenuEvent::new(main_menu()));
                } else {
                    queue.send(MessageEvent::new("Anthea (click for layer menu)", MessageStyle::Info));
                }
                /*queue.send(MessageEvent::new_multi(vec![
                    Message::new("Journal",MessageStyle::Interaction),
                    Message::new("Inventory",MessageStyle::Interaction),
                    Message::new("Talents",MessageStyle::Interaction),
                ]));*/
            } else if let Some(c) = stage.character_from_position(&sprite_position) {
                queue.send(MessageEvent::new(&c.description, MessageStyle::Info));
            } else if let Some(a) = stage.affordance_from_position(&sprite_position) {
                queue.send(MessageEvent::new(&a.description, MessageStyle::Info));
            } else if let Some(i) = stage.item_from_position(&sprite_position)  {
                queue.send(MessageEvent::new(&i.description, MessageStyle::Info));
            } else if let Some(r) = stage.room_from_position(&sprite_position) {
                queue.send(MessageEvent::new(&r.description, MessageStyle::Info));
            } else {
                clearm.send(ClearMessage);
            }
        } else {
            if pressed {
                clearm.send(ClearMessage);
            }
        }
    }
    
}

fn body_change(
    mut event_reader: EventReader<BodyChangeEvent>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut event_memory: ResMut<EventMemory>,
    mut sprite_query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>, &PlayerPart)>,
) {
    if let Some(e) = event_reader.iter().next() {
        for (mut sprite, atlas_handle, part) in sprite_query.iter_mut() {
            if part == &e.part {
                if let Some(texture_atlas) = texture_atlases.get(atlas_handle) {
                    let hair_handle = asset_server.get_handle(e.sprite.as_str());
                    if let Some(hair_index) = texture_atlas.get_texture_index(&hair_handle) {
                        sprite.index = hair_index as u32;
                        event_memory.body.push(e.clone());
                    } else {
                        eprintln!("Could not find handle for {}", e.sprite);
                    }
                } else {
                    eprintln!("Could not find handle for body parts");
                }
            }
        }
    }
}

fn journal(
    mut event_reader: EventReader<JournalEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut journal: ResMut<Journal>,
) {
    for je in event_reader.iter() {
        audio.play(asset_server.get_handle("sounds/journal.ogg"));
        journal.add_entry(&je.quest, &je.text);
    }
}

fn remove_tile(
    mut commands: Commands,
    mut event_reader: EventReader<RemoveTileEvent>,
    mut event_memory: ResMut<EventMemory>,
    mut state: ResMut<AntheaState>,
    maptile_query: Query<&MapTile>,
) {
    for rte in event_reader.iter() {
        if let Some(tes) = state.positions.get_mut(&rte.position) {
            tes.passable = true;
            for e in tes.entities.iter() {
                if let Ok(MapTile(layer)) = maptile_query.get(*e) {
                    if *layer == rte.layer {
                        commands.entity(*e).despawn_recursive();
                        event_memory.removed_tiles.push(rte.clone());
                    }
                }
            }
        }
    }
}
