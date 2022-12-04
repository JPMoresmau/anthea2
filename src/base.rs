use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::tiled::*;

pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 480;

pub const SPRITE_SIZE: i32 = 32;

pub const VISIBILITY_DISTANCE: f32 = 4.0 * SPRITE_SIZE as f32;

pub const MOVE_DELAY: u128 = 200;

pub const DOUBLE_CLICK_DELAY: u128 = 500;

pub const STAGE: &str = "app_state";
pub const CLOSE: &str = "Close";

pub const QUEST_MAIN: &str = "main";

pub const QUEST_STARTED: &str = "started";
pub const QUEST_COMPLETED: &str = "completed";

#[derive(Default, Resource, AssetCollection)]
pub struct AntheaHandles {
    #[asset(path = "sprites/people", collection)]
    pub people_handles: Vec<HandleUntyped>,
    #[asset(path = "sprites/tiles", collection)]
    pub tile_handles: Vec<HandleUntyped>,
    #[asset(path = "sprites/items", collection)]
    pub item_handles:Vec<HandleUntyped>,
    #[asset(path = "anthea_tileset.tsx")]
    pub tileset_handle: Handle<TileSet>,
    #[asset(path = "castle1.tmx")]
    pub map_handle: Handle<Map>,
    #[asset(path = "RPG_GUI_v1.png")]
    pub ui_handle: Handle<Image>,
    #[asset(path = "paper background.png")]
    pub paper_handle: Handle<Image>,
    #[asset(path = "GRECOromanLubedWrestling.ttf")]
    pub font_handle: Handle<Font>,
    pub ui_texture_atlas_handle: Handle<TextureAtlas>,
    #[asset(path = "sounds", collection)]
    pub sound_handles: Vec<HandleUntyped>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct AntheaState {
    //player_position: Position,
    pub map_position: SpritePosition,
    pub positions: HashMap<SpritePosition, TileEntityState>,
    pub revealed: HashSet<SpritePosition>,
    pub last_move: u128,
    //pub last_hover: Option<SpritePosition>,
}

impl Default for AntheaState {
    fn default() -> Self {
        Self {
            //player_position: Position::default(),
            map_position: SpritePosition::default(),
            positions: HashMap::new(),
            revealed: HashSet::new(),
            last_move: 0,
            //last_hover: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter,Hash)]
pub enum GameState {
    Setup,
    Title,
    Background,
    Start,
    Running,
    Menu,
    Pause,
    Save,
    Clean,
    Reset,
    Load,
    End,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Resource)]
pub struct MouseLocation {
    pub coords: Option<SpritePosition>,
    pub last_click: Option<SpritePosition>,
    pub last_click_time: u128
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileEntityState {
    pub entities: Vec<Entity>,
    pub passable: bool,
    pub transparent: bool,
}

impl Default for TileEntityState {
    fn default() -> Self {
        Self {
            entities: vec![],
            passable: true,
            transparent: true,
        }
    }
}


#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SpritePosition {
    pub x: i32,
    pub y: i32,
}

impl SpritePosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_coords(x: f32, y: f32) -> Self {
        Self { 
            x: (x/SPRITE_SIZE as f32).round() as i32,
            y: (y/SPRITE_SIZE as f32).round() as i32, 
         }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Self::to_vec3_z(self,0.0)
    }

    pub fn to_vec3_z(&self, z: f32) -> Vec3 {
        Vec3::new((self.x * SPRITE_SIZE) as f32, (self.y * SPRITE_SIZE) as f32, z)
    }

    pub fn from_vec3(v: &Vec3) -> SpritePosition {
        Self::from_coords(v.x,v.y)
    }

    pub fn copy(&mut self, pos: &SpritePosition) {
        self.x = pos.x;
        self.y = pos.y;
    }

    pub fn to_relative(&self, pos: &SpritePosition) -> SpritePosition {
        SpritePosition {
            x: self.x - pos.x,
            y: self.y - pos.y,
        }
    }

    pub fn add(&self, pos: &SpritePosition) -> SpritePosition {
        SpritePosition {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }

    pub fn distance(&self, other: &SpritePosition) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
    
}



#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SpriteDimension {
    topleft: SpritePosition,
    bottomright: SpritePosition,
}

impl SpriteDimension {
    pub fn new(topleft: SpritePosition, bottomright: SpritePosition) -> Self {
        Self {
            topleft,
            bottomright,
        }
    }

    pub fn contains(&self, pos: &SpritePosition) -> bool {
        pos.x >= self.topleft.x
            && pos.x <= self.bottomright.x
            && pos.y >= self.topleft.y
            && pos.y <= self.bottomright.y
    }

    pub fn positions(&self) -> Vec<SpritePosition> {
        let mut v= vec![];
        for x in self.topleft.x ..= self.bottomright.x {
            for y in self.topleft.y ..= self.bottomright.y {
                v.push(SpritePosition::new(x,y));
            }
        }
        v
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Resource)]
pub struct MovementPlan(pub Vec<SpritePosition>);

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Component)]
pub struct MapTile(pub usize);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, EnumIter, Serialize, Deserialize, Component)]
pub enum PlayerPart {
    Body,
    Pants,
    Top,
    Hair,
    RightHand,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Component)]
pub struct Player;

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Component)]
pub struct MainCamera;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Resource)]
pub struct Journal {
    pub quests: HashMap<String, Quest>,
    pub entries: Vec<JournalEntry>,
}

impl Default for Journal {
    fn default() -> Self {
        let q = Quest::new(QUEST_MAIN, "Main Quest");
        let mut j = Journal {
            quests: HashMap::new(),
            entries: vec![],
        };
        j.add_quest(q)
            .add_entry(QUEST_MAIN,"I have decided it, and nothing will alter my resolve. I will set up in search for Father. Peleus cannot stop me.");

        j
    }
}

impl Journal {
    pub fn add_quest(&mut self, quest: Quest) -> &mut Self {
        self.quests.insert(quest.code.clone(), quest);
        self
    }

    pub fn add_entry<S1: Into<String>, S2: Into<String>>(
        &mut self,
        quest: S1,
        text: S2,
    ) -> &mut Self {
        self.entries.push(JournalEntry::new(quest, text));
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct JournalEntry {
    pub quest: String,
    pub text: String,
}

impl JournalEntry {
    pub fn new<S1: Into<String>, S2: Into<String>>(quest: S1, text: S2) -> Self {
        JournalEntry {
            quest: quest.into(),
            text: text.into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct JournalEvent {
    pub quest: String,
    pub text: String,
}

impl JournalEvent {
    pub fn new<S1: Into<String>, S2: Into<String>>(quest: S1, text: S2) -> Self {
        JournalEvent {
            quest: quest.into(),
            text: text.into(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Quest {
    pub code: String,
    pub text: String,
}

impl Quest {
    pub fn new<S1: Into<String>, S2: Into<String>>(code: S1, text: S2) -> Self {
        Quest {
            code: code.into(),
            text: text.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub sprite: String,
    pub position: SpritePosition,
    //pub dimension: SpriteDimension,
    pub consumable: bool,
}

impl Item {
    pub fn new<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        name: S1,
        description: S2,
        sprite: S3,
        x1: i32,
        y1: i32,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            sprite: sprite.into(),
            position: SpritePosition::new(x1, y1),
            //dimension: SpriteDimension::new(SpritePosition::new(x1, y1), SpritePosition::new(x1, y1)),
            consumable: false,
        }
    }

    pub fn new_consumable<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        name: S1,
        description: S2,
        sprite: S3,
        x1: i32,
        y1: i32,
    ) -> Self {
        let mut s = Item::new(name, description, sprite, x1, y1);
        s.consumable = true;
        s
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn add_item(&mut self, item: Item) -> &mut Self {
        self.items.push(item);
        self.items.sort_by_key(|i| i.description.clone());
        self
    }

    pub fn contains_item(&self, item: &str) -> bool {
        self.items.iter().any(|i| i.name == item)
    }

    pub fn remove_item(&mut self, item: &str) -> &mut Self {
        if let Some((ix, _e)) = self
            .items
            .iter()
            .enumerate()
            .find(|(_ix, i)| i.name == item)
        {
            self.items.remove(ix);
        }
        self
    }
}


#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct Talents {
    pub animals: u32,
    pub people: u32,
    pub weapons: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct QuestFlags {
    flags: HashSet<(String, String)>,
}

impl QuestFlags {
    pub fn set_flag<S1: Into<String>, S2: Into<String>>(
        &mut self,
        quest: S1,
        flag: S2,
    ) -> &mut Self {
        self.flags.insert((quest.into(), flag.into()));
        self
    }

    pub fn unset_flag<S1: Into<String>, S2: Into<String>>(
        &mut self,
        quest: S1,
        flag: S2,
    ) -> &mut Self {
        self.flags.remove(&(quest.into(), flag.into()));
        self
    }

    pub fn has_flag<S1: Into<String>, S2: Into<String>>(&self, quest: S1, flag: S2) -> bool {
        self.flags.contains(&(quest.into(), flag.into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub description: String,
}

impl Spell {
    pub fn new<S1: Into<String>, S2: Into<String>>(name: S1, description: S2) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct Spells {
    pub spells: Vec<Spell>,
}

impl Spells {
    pub fn add_spell(&mut self, spell: Spell) -> &mut Self {
        self.spells.push(spell);
        self.spells.sort_by_key(|i| i.description.clone());
        self
    }

    pub fn contains_spell(&self, spell: &str) -> bool {
        self.spells.iter().any(|i| i.name == spell)
    }

    pub fn remove_spell(&mut self, spell: &str) -> &mut Self {
        if let Some((ix, _e)) = self
            .spells
            .iter()
            .enumerate()
            .find(|(_ix, i)| i.name == spell)
        {
            self.spells.remove(ix);
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyChangeEvent {
    pub part: PlayerPart,
    pub sprite: String,
}

impl BodyChangeEvent {
    pub fn new<S1: Into<String>>(part: PlayerPart, sprite: S1) -> Self {
        Self {
            part,
            sprite: sprite.into(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Component)]
pub struct Help;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTileEvent {
    pub position: SpritePosition,
    pub layer: usize,
}

impl RemoveTileEvent {
    pub fn new(position: SpritePosition, layer: usize) -> Self {
        Self { position, layer }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Resource)]
pub struct EventMemory {
    pub body: Vec<BodyChangeEvent>,
    pub removed_tiles: Vec<RemoveTileEvent>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEvent(pub SpritePosition);