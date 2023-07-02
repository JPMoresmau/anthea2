use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_asset_loader::prelude::*;

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

use pathfinding::prelude::astar;
use std::collections::{HashMap, HashSet};
use std::env;

fn main() {
    let mut builder = App::new();

    builder
        .init_resource::<AntheaHandles>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Anthea's Quest".to_string(),
                resolution: (SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AntheaPlugin);
    if let Ok(var) = env::var("BEVY_DIAGNOSTICS") {
        if &var == "1" {
            builder
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default());
        }
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
    }
    builder.run();
}

pub struct AntheaPlugin;

impl Plugin for AntheaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AntheaState::default())
            .insert_resource(MouseLocation::default())
            .insert_resource(Journal::default())
            .insert_resource(Inventory::default())
            .insert_resource(Talents::default())
            .insert_resource(QuestFlags::default())
            .insert_resource(Spells::default())
            .insert_resource(EventMemory::default())
            .insert_resource(MovementPlan::default())
            .add_event::<AffordanceEvent>()
            .add_event::<CharacterEvent>()
            .add_event::<ItemEvent>()
            .add_event::<BodyChangeEvent>()
            .add_event::<JournalEvent>()
            .add_event::<RemoveTileEvent>()
            .add_event::<MoveEvent>()
            .add_plugin(CastlePlugin)
            .add_asset::<Map>()
            .init_asset_loader::<MapAssetLoader>()
            .add_asset::<TileSet>()
            .init_asset_loader::<TileSetAssetLoader>()
            .add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Setup).continue_to_state(GameState::Title),
            )
            .add_collection_to_loading_state::<_, AntheaHandles>(GameState::Setup)
            .add_system(setup_camera.in_schedule(OnEnter(GameState::Title)))
            .add_system(setup_map.in_schedule(OnEnter(GameState::Background)))
            .add_systems(
                (setup_items, setup_body, setup_people)
                    .chain()
                    .in_schedule(OnEnter(GameState::Start)),
            )
            .add_system(start_system.in_set(OnUpdate(GameState::Start)))
            .add_systems(
                (
                    player_movement_system,
                    automatic_movement_system,
                    move_system,
                    click_system,
                    pickup_item,
                    body_change,
                    journal,
                    remove_tile,
                )
                    .in_set(OnUpdate(GameState::Running)),
            )
            .add_plugin(MenuPlugin)
            .add_plugin(UIPlugin);
    }
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut state: ResMut<AntheaState>,
    mut msg: EventWriter<MoveEvent>,
) {
    state.last_move += time.delta().as_millis();
    if state.last_move < MOVE_DELAY {
        return;
    }

    //let (mut pos,mut map) = (&mut (state.player_position),&mut state.map_position);
    for i in keyboard_input.get_pressed() {
        let mut new_pos = state.map_position.clone();
        match i {
            KeyCode::Right => new_pos.x += 1,
            KeyCode::Left => new_pos.x -= 1,
            KeyCode::Up => new_pos.y -= 1,
            KeyCode::Down => new_pos.y += 1,
            _ => (),
        }
        msg.send(MoveEvent(new_pos));
    }
}

fn move_system(
    mut move_events: EventReader<MoveEvent>,
    mut state: ResMut<AntheaState>,
    stage: ResMut<Area>,
    mut sprite_query: Query<
        (&mut Transform, &mut Visibility, &ComputedVisibility),
        Or<(With<MapTile>, With<Item>, With<Character>)>,
    >,
    mut msg: EventWriter<ClearMessage>,
    mut ev_affordance: EventWriter<AffordanceEvent>,
    mut ev_character: EventWriter<CharacterEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some(e) = move_events.iter().next() {
        let mut new_pos = e.0.clone();
        if new_pos != state.map_position {
            if let Some(tes) = state.positions.get(&new_pos) {
                if !tes.passable {
                    new_pos.copy(&state.map_position);
                }
            }
        }
        if new_pos != state.map_position {
            state.last_move = 0;

            //let sprite_position=new_pos.inverse_x();
            if let Some(a) = stage.affordance_from_position(&new_pos) {
                // println!("Affordance: {}",a.name);
                ev_affordance.send(AffordanceEvent(a.name.clone()));
            } else if let Some(c) = stage.character_from_position(&new_pos) {
                //println!("Character: {}",c.name);
                ev_character.send(CharacterEvent(c.name.clone()));
            } else {
                msg.send(ClearMessage);
                audio.play(asset_server.get_handle("sounds/steps.ogg"));
                let dif_x = ((new_pos.x - state.map_position.x) * SPRITE_SIZE) as f32;
                let dif_y = ((new_pos.y - state.map_position.y) * SPRITE_SIZE) as f32;
                state.map_position = new_pos;

                for (mut transform, mut vis, cvis) in &mut sprite_query.iter_mut() {
                    transform.translation.x -= dif_x;
                    transform.translation.y += dif_y;
                    if !cvis.is_visible() && is_visible(&transform.translation, Some(&state)) {
                        *vis = Visibility::Visible;
                        let pos = state.map_position.add(&SpritePosition::from_coords(
                            transform.translation.x,
                            -transform.translation.y,
                        ));
                        //println!("Revealing: {:?}",pos);
                        state.revealed.insert(pos);
                    }
                }
            }
        }
    }
}

fn automatic_movement_system(
    mut move_plan: ResMut<MovementPlan>,
    mut msg: EventWriter<MoveEvent>,
    state: Res<AntheaState>,
) {
    if state.last_move < MOVE_DELAY {
        return;
    }

    if let Some(new_pos) = move_plan.0.pop() {
        msg.send(MoveEvent(new_pos));
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
    if let Some(i) = stage.items.remove(&state.map_position) {
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

fn start_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut clearm: EventWriter<ClearMessage>,
    mut appstate: ResMut<NextState<GameState>>,
    mut state: ResMut<AntheaState>,
    mut sprite_query: Query<
        (&Transform, &mut Visibility, &ComputedVisibility),
        (
            Without<Help>,
            Or<(With<MapTile>, With<Item>, With<Character>)>,
        ),
    >,
    mut help_query: Query<&mut Visibility, With<Help>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        clearm.send(ClearMessage);
        appstate.set(GameState::Running);
        for (transform, mut vis, cvis) in &mut sprite_query.iter_mut() {
            if !cvis.is_visible() && is_visible(&transform.translation, Some(&state)) {
                *vis = Visibility::Visible;
                let pos = state.map_position.add(&SpritePosition::from_coords(
                    transform.translation.x,
                    -transform.translation.y,
                ));
                //println!("Revealing: {:?}",pos);
                state.revealed.insert(pos);
            }
        }
        for mut vis in &mut help_query.iter_mut() {
            *vis = Visibility::Visible;
        }
    }
}

fn click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut location: ResMut<MouseLocation>,
    mut queue: EventWriter<MessageEvent>,
    mut clearm: EventWriter<ClearMessage>,
    state: Res<AntheaState>,
    stage: Res<Area>,
    mut menu: EventWriter<MenuEvent>,
    time: Res<Time>,
    mut move_plan: ResMut<MovementPlan>,
) {
    let pressed = mouse_button_input.just_pressed(MouseButton::Left);
    if !pressed {
        return;
    }
    let window = window.get_single().unwrap();
    //if let Some(rel_pos) = location.coords.clone() {
    if let Some(camera_transform) = q_camera.iter().next() {
        if let Some(w_pos) = window.cursor_position() {
            let size = Vec2::new(window.width(), window.height());

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = w_pos - size / 2.0;

            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            //println!("World coords: {}/{}", pos_wld.x, pos_wld.y);
            let rel_pos = SpritePosition::from_coords(pos_wld.x, -pos_wld.y);

            let sprite_position = state.map_position.add(&rel_pos);
            //println!("relative: {:?},{:?}",rel_x,rel_y);
            let ms = time.elapsed().as_millis();

            let dbl_clicked = if let Some(old) = &location.last_click {
                old == &sprite_position && ms - location.last_click_time < DOUBLE_CLICK_DELAY
            } else {
                false
            };
            location.last_click = Some(sprite_position.clone());
            location.last_click_time = ms;

            //let rel_pos = Position::new(-rel_x as i32, rel_y as i32);
            //let sprite_position=SpritePosition::new(rel_x,rel_y);
            /*if !pressed {
                if let Some(p) = &state.last_hover {
                    if p==&sprite_position {
                        return;
                    }
                }
            }
            state.last_hover=Some(sprite_position.clone());
            */
            //println!("left mouse currently pressed as: {:?}",sprite_position);
            //println!("left mouse currently pressed relative: {:?}",rel_pos);
            //if x < -SCREEN_WIDTH as f32 / 2.0 + SPRITE_SIZE as f32
            //    && y > SCREEN_HEIGHT as f32 / 2.0 - SPRITE_SIZE as f32
            if rel_pos.x <= -9 && rel_pos.y == -7 {
                //if pressed {
                menu.send(MenuEvent::new(system_menu()));
                location.coords = None;
                /*} else {
                    queue.send(MessageEvent::new("System menu", MessageStyle::Info));
                }*/
                return;
            }

            let revealed = state.revealed.contains(&sprite_position);
            /*for rp in state.revealed.iter() {
                if rp.distance(&rel_pos) <= SPRITE_SIZE / 2 {
                    revealed = true;
                    break;
                }
            }*/

            //println!("sprite pos: {:?},{:?}",(rel_x/SPRITE_SIZE as f32).round() as i32 ,(rel_y/SPRITE_SIZE as f32).round() as i32);

            if revealed {
                if sprite_position == state.map_position {
                    //println!("click on center");
                    //appstate.set_next(GameState::Menu).unwrap();
                    //if pressed {
                    location.coords = None;
                    menu.send(MenuEvent::new(main_menu()));
                    /* } else {
                        queue.send(MessageEvent::new("Anthea (click for player menu)", MessageStyle::Info));
                    }*/
                    /*queue.send(MessageEvent::new_multi(vec![
                        Message::new("Journal",MessageStyle::Interaction),
                        Message::new("Inventory",MessageStyle::Interaction),
                        Message::new("Talents",MessageStyle::Interaction),
                    ]));*/
                } else {
                    if let Some(c) = stage.character_from_position(&sprite_position) {
                        queue.send(MessageEvent::new(&c.description, MessageStyle::Info));
                    } else if let Some(a) = stage.affordance_from_position(&sprite_position) {
                        queue.send(MessageEvent::new(&a.description, MessageStyle::Info));
                    } else if let Some(i) = stage.item_from_position(&sprite_position) {
                        queue.send(MessageEvent::new(&i.description, MessageStyle::Info));
                    } else if let Some(r) = stage.room_from_position(&sprite_position) {
                        queue.send(MessageEvent::new(&r.description, MessageStyle::Info));
                    } else {
                        clearm.send(ClearMessage);
                    }
                    if dbl_clicked {
                        //println!("Double click");
                        move_plan.0.clear();
                        if let Some(tes) = state.positions.get(&sprite_position) {
                            if tes.passable {
                                //println!("State positions: {:?}",&state.positions);
                                //println!("Stage characters: {:?}",&stage.characters);
                                //println!("Should go from {:?} to {:?}",&state.map_position,sprite_position);
                                move_plan.0.append(&mut path(state, &sprite_position));
                            }
                        }
                    }
                }
            } else {
                //if pressed {
                clearm.send(ClearMessage);
                //}
            }
        }
    }
}

fn successors(
    pos: &SpritePosition,
    positions: &HashMap<SpritePosition, TileEntityState>,
    revealed: &HashSet<SpritePosition>,
) -> Vec<(SpritePosition, u32)> {
    vec![
        SpritePosition::new(pos.x - 1, pos.y),
        SpritePosition::new(pos.x + 1, pos.y),
        SpritePosition::new(pos.x, pos.y - 1),
        SpritePosition::new(pos.x, pos.y + 1),
    ]
    .into_iter()
    .filter(|p| revealed.contains(p))
    .filter(|p| {
        if let Some(tes) = positions.get(p) {
            tes.passable
        } else {
            false
        }
    })
    .map(|p| (p, 1))
    .collect()
}

fn path(state: Res<AntheaState>, to: &SpritePosition) -> Vec<SpritePosition> {
    //println!("Should go from {:?} to {:?}",&state.map_position,to);
    let result = astar(
        &state.map_position,
        |p| successors(p, &state.positions, &state.revealed),
        |p| p.distance(to) / 3,
        |p| p == to,
    );
    let mut v = result.map(|t| t.0).unwrap_or_else(Vec::new);
    v.reverse();
    v
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
                        sprite.index = hair_index;
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
