use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::ui::*;
use crate::{
    base::*,
    setup::{do_setup_map, setup_items, setup_people},
    tiled::{Map, TileSet},
    world::{Affordance, Area, Character},
};
use bevy::prelude::*;
use ron::de::from_str;
use ron::ser::to_string;
use serde::{Deserialize, Serialize};

pub struct MenuPlugin;

pub const MAIN: &str = "main";
pub const JOURNAL: &str = "journal";
pub const INVENTORY: &str = "inventory";
pub const TALENTS: &str = "talents";
pub const SPELLS: &str = "spells";
pub const SYSTEM: &str = "system";
pub const HELP: &str = "help";
pub const SAVE: &str = "save";
pub const LOAD: &str = "load";

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Resource)]
struct Menus {
    menus: Vec<Menu>,
    pub journal_index: Option<usize>,
}

impl Menus {
    pub fn push(&mut self, m: Menu) -> &mut Self {
        self.menus.push(m);
        self
    }

    pub fn pop(&mut self) -> Option<Menu> {
        self.menus.pop()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.menus.clear();
        self.journal_index = None;
        self
    }

    pub fn current(&self) -> &String {
        &self.menus.iter().last().unwrap().code
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Menu {
    code: String,
    title: String,
    navigation: Option<(bool, bool)>,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        code: S1,
        title: S2,
        items: Vec<MenuItem>,
    ) -> Self {
        Menu {
            code: code.into(),
            title: title.into(),
            navigation: None,
            items,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MenuItem {
    code: String,
    text: String,
    extra: Option<String>,
}

impl MenuItem {
    pub fn new<S1: Into<String>, S2: Into<String>>(code: S1, text: S2) -> Self {
        MenuItem {
            code: code.into(),
            text: text.into(),
            extra: None,
        }
    }

    pub fn new_table<S1: Into<String>, S2: Into<String>>(text: S1, extra: S2) -> Self {
        MenuItem {
            code: String::new(),
            text: text.into(),
            extra: Some(extra.into()),
        }
    }
}

fn help_item() -> MenuItem {
    MenuItem::new(HELP, "Help")
}

fn save_item() -> MenuItem {
    MenuItem::new(SAVE, "Save")
}

fn load_item() -> MenuItem {
    MenuItem::new(LOAD, "Load")
}

pub fn system_menu() -> Menu {
    Menu::new(
        SYSTEM,
        "System",
        vec![help_item(), save_item(), load_item()],
    )
}

fn journal_item() -> MenuItem {
    MenuItem::new(JOURNAL, "Journal")
}

fn inventory_item() -> MenuItem {
    MenuItem::new(INVENTORY, "Inventory")
}

fn talents_item() -> MenuItem {
    MenuItem::new(TALENTS, "Talents")
}

fn spells_item() -> MenuItem {
    MenuItem::new(SPELLS, "Spells")
}

pub fn main_menu() -> Menu {
    Menu::new(
        MAIN,
        "Anthea",
        vec![
            journal_item(),
            inventory_item(),
            spells_item(),
            talents_item(),
        ],
    )
}

fn help_menu() -> Menu {
    Menu::new(HELP, "Help", vec![MenuItem::new("", "Click on your character in the middle of screen for journal, inventory, spells and talents.\nClick everywhere else to see a description.\nUse arrow keys to move.\nMove over an item to pick it up, move into characters and other things to interact.")])
}

fn journal_menu(journal: &Journal, menus: &Menus) -> Menu {
    let idx = menus.journal_index.unwrap_or(journal.entries.len() - 1);
    let e = journal.entries.get(idx).unwrap();
    let mut m = Menu::new(JOURNAL, "Journal", vec![MenuItem::new("", &e.text)]);
    m.navigation = Some((idx > 0, idx < journal.entries.len() - 1));
    m
}

fn inventory_menu(inventory: &Inventory) -> Menu {
    let mut msgs: Vec<MenuItem> = inventory
        .items
        .iter()
        .map(|i| MenuItem::new("", &i.description))
        .collect();
    if msgs.is_empty() {
        msgs.push(MenuItem::new("", "Empty hands!"));
    }
    Menu::new(INVENTORY, "Inventory", msgs)
}

fn talents_menu(talents: &Talents) -> Menu {
    Menu::new(
        TALENTS,
        "Talents",
        vec![
            MenuItem::new_table("Animals:", format!("{:>3}", talents.animals)),
            MenuItem::new_table("People:", format!("{:>3}", talents.people)),
            MenuItem::new_table("Weapons:", format!("{:>3}", talents.weapons)),
        ],
    )
}

fn spells_menu(spells: &Spells) -> Menu {
    let mut msgs: Vec<MenuItem> = spells
        .spells
        .iter()
        .map(|i| MenuItem::new("", &i.description))
        .collect();
    if msgs.is_empty() {
        msgs.push(MenuItem::new("", "Empty head!"));
    }
    Menu::new(SPELLS, "Spells", msgs)
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuEvent>()
            .add_event::<MenuItemEvent>()
            .add_event::<CloseMenuEvent>()
            .insert_resource(Menus::default())
            .add_system(menu_start)
            //.on_state_enter(STAGE, GameState::Menu,show_main_menu)
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_system))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_nav_system))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(journal_event))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(inventory_event))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(spells_event))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(talents_event))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(help_event))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(save_event))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(load_event))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_close))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(close_menu))
            .add_system_set(SystemSet::on_enter(GameState::Save).with_system(save))
            .add_system_set(SystemSet::on_enter(GameState::Clean).with_system(clean))
            .add_system_set(SystemSet::on_enter(GameState::Reset).with_system(reset))
            .add_system_set(SystemSet::on_enter(GameState::Reset).with_system(setup_items))
            .add_system_set(SystemSet::on_enter(GameState::Reset).with_system(setup_people))
            .add_system_set(SystemSet::on_enter( GameState::Load).with_system(load));
    }
}

fn show_menu(mut queue: EventWriter<MessageEvent>, menu: &Menu) {
    //clearm.send(ClearMessage);
    let mut msgs = vec![Message::new(&menu.title, MessageStyle::MenuTitle)];
    if let Some((backward, forward)) = menu.navigation {
        msgs.push(Message::new(
            "",
            MessageStyle::Navigation(backward, forward),
        ));
    }
    for mi in menu.items.iter() {
        if let Some(extra) = &mi.extra {
            msgs.push(Message::new(
                &mi.text,
                MessageStyle::Table(vec![extra.clone()]),
            ));
        } else {
            msgs.push(Message::new(
                &mi.text,
                MessageStyle::Interaction(mi.code.clone()),
            ));
        }
    }
    queue.send(MessageEvent::new_multi(msgs));
}

fn push_menu(queue: EventWriter<MessageEvent>, mut menus: ResMut<Menus>, menu: Menu) {
    show_menu(queue, &menu);
    menus.push(menu);
}

/*fn show_main_menu(
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    menus: ResMut<Menus>,
){
    let m=main_menu();
    push_menu(clearm, queue, menus,m);
}*/

/*fn click_system(mouse_button_input: Res<Input<MouseButton>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    mut appstate: ResMut<State<GameState>>,
    ) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        clearm.send(ClearMessage);
        appstate.set_next(GameState::Running).unwrap();
    }
}*/

fn click_system(
    item_query: Query<(&Interaction, &Text, &InteractionItem), Changed<Interaction>>,
    mut clearm: EventWriter<ClearMessage>,
    queue: EventWriter<MessageEvent>,
    mut appstate: ResMut<State<GameState>>,
    mut menus: ResMut<Menus>,
    mut menuqueue: EventWriter<MenuItemEvent>,
) {
    if let Some((interaction, _txt, item)) = item_query.iter().next() {
        if *interaction == Interaction::Clicked {
            let msg = &item.0;
            if CLOSE == msg {
                menus.pop();
                if let Some(m) = menus.menus.last() {
                    show_menu(queue, m);
                } else {
                    clearm.send(ClearMessage);
                    appstate.set(GameState::Running).unwrap();
                }
            } else if let Some(m) = menus.menus.last() {
                menuqueue.send(MenuItemEvent {
                    menu: m.code.clone(),
                    item: msg.into(),
                });
            }
        }
    }
}

fn click_nav_system(
    item_query: Query<(&Interaction, &TextureAtlasSprite, &NavigationPart), Changed<Interaction>>,
    mut menus: ResMut<Menus>,
    queue: EventWriter<MessageEvent>,
    journal: Res<Journal>,
) {
    if let Some((interaction, _txt, item)) = item_query.iter().next() {
        if *interaction == Interaction::Clicked && menus.current() == JOURNAL {
            let idx = menus.journal_index.unwrap_or(journal.entries.len() - 1);
            match item {
                NavigationPart::Back => menus.journal_index = Some(idx - 1),
                NavigationPart::Forward => menus.journal_index = Some(idx + 1),
            }
            menus.pop();
            let m = journal_menu(&journal, &menus);
            push_menu(queue, menus, m);
        }
    }
}

fn close_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut clearm: EventWriter<ClearMessage>,
    queue: EventWriter<MessageEvent>,
    mut appstate: ResMut<State<GameState>>,
    mut menus: ResMut<Menus>,
) {
    //for event in keyboard_input_events.iter() {
    if keyboard_input.just_released(KeyCode::Escape) {
        menus.pop();
        if let Some(m) = menus.menus.last() {
            show_menu(queue, m);
        } else {
            clearm.send(ClearMessage);
            appstate.set(GameState::Running).unwrap();
        }
    }
    //}
}

#[derive(Debug, Clone)]
pub struct MenuEvent {
    pub menu: Menu,
}

impl MenuEvent {
    pub fn new(m: Menu) -> Self {
        MenuEvent { menu: m }
    }
}

#[derive(Debug, Clone)]
pub struct CloseMenuEvent;

fn menu_start(
    mut appstate: ResMut<State<GameState>>,
    mut event_reader: EventReader<MenuEvent>,
    queue: EventWriter<MessageEvent>,
    mut menus: ResMut<Menus>,
) {
    if let Some(me) = event_reader.iter().next() {
        appstate.set(GameState::Menu).unwrap();
        menus.clear();
        push_menu(queue, menus, me.menu.clone());
    }
}

fn menu_close(
    mut appstate: ResMut<State<GameState>>,
    mut event_reader: EventReader<CloseMenuEvent>,
    mut menus: ResMut<Menus>,
) {
    if let Some(_me) = event_reader.iter().next() {
        appstate.set(GameState::Running).unwrap();
        menus.clear();
        //clearm.send(ClearMessage);
    }
}

#[derive(Debug, Clone)]
pub struct MenuItemEvent {
    pub menu: String,
    pub item: String,
}

fn journal_event(
    mut event_reader: EventReader<MenuItemEvent>,
    journal: Res<Journal>,
    menus: ResMut<Menus>,
    queue: EventWriter<MessageEvent>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == MAIN && e.item == JOURNAL)
    {
        let m = journal_menu(&journal, &menus);
        push_menu(queue, menus, m);
    }
}

fn inventory_event(
    mut event_reader: EventReader<MenuItemEvent>,
    inventory: Res<Inventory>,
    menus: ResMut<Menus>,
    queue: EventWriter<MessageEvent>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == MAIN && e.item == INVENTORY)
    {
        let m = inventory_menu(&inventory);
        push_menu(queue, menus, m);
    }
}

fn spells_event(
    mut event_reader: EventReader<MenuItemEvent>,
    spells: Res<Spells>,
    menus: ResMut<Menus>,
    queue: EventWriter<MessageEvent>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == MAIN && e.item == SPELLS)
    {
        let m = spells_menu(&spells);
        push_menu(queue, menus, m);
    }
}

fn talents_event(
    mut event_reader: EventReader<MenuItemEvent>,
    talents: Res<Talents>,
    menus: ResMut<Menus>,
    queue: EventWriter<MessageEvent>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == MAIN && e.item == TALENTS)
    {
        let m = talents_menu(&talents);
        push_menu(queue, menus, m);
    }
}

fn help_event(
    mut event_reader: EventReader<MenuItemEvent>,
    menus: ResMut<Menus>,
    queue: EventWriter<MessageEvent>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == SYSTEM && e.item == HELP)
    {
        let m = help_menu();
        push_menu(queue, menus, m);
    }
}

fn save_event(
    mut event_reader: EventReader<MenuItemEvent>,
    mut appstate: ResMut<State<GameState>>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == SYSTEM && e.item == SAVE)
    {
        appstate.set(GameState::Save).unwrap();
    }
}

fn save(world: &mut World) {
    let ss = SaveState::from_world(world);
    let save_string = to_string(&ss).unwrap();
    write!(
        File::create(&Path::new("save.ron")).unwrap(),
        "{}",
        save_string
    )
    .unwrap();

    let mut appstate = world.get_resource_mut::<State<GameState>>().unwrap();
    appstate.set(GameState::Running).unwrap();
    let mut menus = world.get_resource_mut::<Menus>().unwrap();
    menus.clear();
    let mut clearm = world
        .get_resource_mut::<bevy::ecs::event::Events<ClearMessage>>()
        .unwrap();
    clearm.send(ClearMessage);
}

fn load_event(
    mut event_reader: EventReader<MenuItemEvent>,
    mut appstate: ResMut<State<GameState>>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == SYSTEM && e.item == LOAD)
    {
        appstate.set(GameState::Clean).unwrap();
    }
}

fn clean(world: &mut World) {
    let mut s = String::new();
    File::open(&Path::new("save.ron"))
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let ss: SaveState = from_str(&s).unwrap();
    ss.clean_world(world);
    world.insert_resource::<SaveState>(ss);
    let mut appstate = world.get_resource_mut::<State<GameState>>().unwrap();
    appstate.set(GameState::Reset).unwrap();
}

fn reset(
    commands: Commands,
    sprite_handles: Res<AntheaHandles>,
    asset_server: Res<AssetServer>,
    //stage: Res<Area>,
    state: ResMut<AntheaState>,
    map_assets: Res<Assets<Map>>,
    tileset_assets: Res<Assets<TileSet>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures: ResMut<Assets<Image>>,
    mut appstate: ResMut<State<GameState>>,
) {
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
    appstate.set(GameState::Load).unwrap();
}

fn load(world: &mut World) {
    let ss: SaveState = world.remove_resource::<SaveState>().unwrap();
    ss.to_world(world);
    let mut appstate = world.get_resource_mut::<State<GameState>>().unwrap();
    appstate.set(GameState::Running).unwrap();
    let mut menus = world.get_resource_mut::<Menus>().unwrap();
    menus.clear();
    let mut clearm = world
        .get_resource_mut::<bevy::ecs::event::Events<ClearMessage>>()
        .unwrap();
    clearm.send(ClearMessage);
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct SaveState {
    state: AntheaState,
    journal: Journal,
    inventory: Inventory,
    talents: Talents,
    flags: QuestFlags,
    spells: Spells,
    event_memory: EventMemory,
    area_affordances: HashMap<SpritePosition, Affordance>,
    area_items: HashMap<SpritePosition, Item>,
}

impl SaveState {
    pub fn from_world(world: &World) -> SaveState {
        let mut ss = SaveState::default();
        ss.state = world.get_resource::<AntheaState>().unwrap().clone();
        ss.journal = world.get_resource::<Journal>().unwrap().clone();
        ss.inventory = world.get_resource::<Inventory>().unwrap().clone();
        ss.talents = world.get_resource::<Talents>().unwrap().clone();
        ss.flags = world.get_resource::<QuestFlags>().unwrap().clone();
        ss.spells = world.get_resource::<Spells>().unwrap().clone();
        ss.event_memory = world.get_resource::<EventMemory>().unwrap().clone();
        ss.area_affordances = world.get_resource::<Area>().unwrap().affordances.clone();
        ss.area_items = world.get_resource::<Area>().unwrap().items.clone();

        ss
    }

    pub fn clean_world(&self, world: &mut World) {
        world.insert_resource::<AntheaState>(self.state.clone());
        world.insert_resource::<Journal>(self.journal.clone());
        world.insert_resource::<Inventory>(self.inventory.clone());
        world.insert_resource::<Talents>(self.talents.clone());
        world.insert_resource::<QuestFlags>(self.flags.clone());
        world.insert_resource::<Spells>(self.spells.clone());
        world.insert_resource::<EventMemory>(self.event_memory.clone());

        let mut area = world.get_resource_mut::<Area>().unwrap();
        area.affordances = self.area_affordances.clone();
        area.items = self.area_items.clone();

        let mut todelete: Vec<Entity> = vec![];
        let mut del_query =
            world.query_filtered::<Entity, Or<(With<Item>, With<MapTile>, With<Character>)>>();
        for entity in del_query.iter(world) {
            todelete.push(entity);
        }
        for e in todelete.into_iter() {
            despawn_with_children_recursive(world, e);
        }
    }

    pub fn to_world(&self, world: &mut World) {
        //let old_pos = &world.get_resource::<AntheaState>().unwrap().map_position;
        //let new_pos=&self.state.map_position;
        //let dif_x = (new_pos.x-old_pos.x) as f32;
        //let dif_y = (new_pos.y-old_pos.y) as f32;
        /*
        let mut todelete:Vec<Entity>=vec![];
        let mut todelete_names:Vec<String>=vec![];
        let mut item_query=world.query::<(Entity, &Item)>();


        for (entity, item) in item_query.iter(world){
            if !self.area_items.contains(&item.name){
                todelete.push(entity);
                todelete_names.push(item.name.clone());
            }
        }
        for e in todelete.into_iter(){
            despawn_with_children_recursive(world,e);
        }

        let mut area= world.get_resource_mut::<Area>().unwrap();
        for name in todelete_names.into_iter(){
            area.items.remove(&name);
        }*/

        let mut part_query = world.query::<(Entity, &Handle<TextureAtlas>, &PlayerPart)>();

        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let texture_atlases = world.get_resource::<Assets<TextureAtlas>>().unwrap();

        let mut todo: Vec<(Entity, Handle<TextureAtlas>, String)> = vec![];

        for (entity, atlas_handle, part) in part_query.iter(world) {
            for bce in self.event_memory.body.iter() {
                if part == &bce.part {
                    todo.push((entity, atlas_handle.clone(), bce.sprite.clone()));
                }
            }
        }
        let mut todo2: Vec<(Entity, usize)> = vec![];
        for (entity, atlas_handle, sprite) in todo.into_iter() {
            if let Some(texture_atlas) = texture_atlases.get(&atlas_handle) {
                let hair_handle = asset_server.get_handle(sprite.as_str());
                if let Some(hair_index) = texture_atlas.get_texture_index(&hair_handle) {
                    //sprite.index=hair_index as u32;
                    todo2.push((entity, hair_index));
                }
            }
        }
        for (entity, hair_index) in todo2.into_iter() {
            world.get_mut::<TextureAtlasSprite>(entity).unwrap().index = hair_index;
        }

        let mut todelete: Vec<(Entity, usize)> = vec![];
        let mut state = world.get_resource_mut::<AntheaState>().unwrap();

        for rte in self.event_memory.removed_tiles.iter() {
            if let Some(tes) = state.positions.get_mut(&rte.position) {
                tes.passable = true;
                for e in tes.entities.iter() {
                    todelete.push((*e, rte.layer));
                }
            }
        }

        let mut maptile_query = world.query::<&MapTile>();
        for (e, l) in todelete.iter() {
            if let Ok(MapTile(layer)) = maptile_query.get(world, *e) {
                if layer == l {
                    despawn_with_children_recursive(world, *e);
                }
            }
        }

        let mut sprite_query = world.query_filtered::<(&Transform, &mut Visibility), (
            Without<Help>,
            Or<(With<MapTile>, With<Item>, With<Character>)>,
        )>();
        for (transform, mut vis) in sprite_query.iter_mut(world) {
            if !vis.is_visible {
                let pos = self.state.map_position.to_relative(&SpritePosition::from_coords(
                    transform.translation.x,
                    transform.translation.y,
                ));
                if self.state.revealed.contains(&pos) {
                    vis.is_visible = true;
                }
            }
        }
    }
}
