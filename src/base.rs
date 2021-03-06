use bevy::prelude::*;
use std::{ collections::{HashMap, HashSet}};

use strum_macros::EnumIter; 

use crate::tiled::*;

pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 480;

pub const SPRITE_SIZE: i32 = 32;


pub const VISIBILITY_DISTANCE: f32 = 4.0 * SPRITE_SIZE as f32;

pub const MOVE_DELAY: u128 = 200;

pub const STAGE: &str = "app_state";
pub const CLOSE: &str = "Close";

pub const QUEST_MAIN: &str = "main";

pub const QUEST_STARTED: &str = "started";
pub const QUEST_COMPLETED: &str = "completed";

#[derive(Default)]
pub struct AntheaHandles {
    pub people_handles: Vec<HandleUntyped>,
    pub tile_handles: Vec<HandleUntyped>,
    pub item_handles: Vec<HandleUntyped>,
    pub tileset_handle: Handle<TileSet>,
    pub map_handles: Vec<Handle<Map>>,
    pub ui_handle: Handle<Texture>,
    pub paper_handle: Handle<Texture>,
    pub font_handle: Handle<Font>,
    pub ui_texture_atlas_handle: Handle<TextureAtlas>,
}


#[derive(Debug,Clone)]
pub struct AntheaState {
    //player_position: Position,
    pub map_position: Position,
    pub positions: HashMap<Position,TileEntityState>,
    pub revealed: HashSet<Position>,
    pub last_move: u128,
}

impl Default for AntheaState {
    fn default() -> Self {
        Self {
           //player_position: Position::default(),
           map_position: Position::default(),
           positions: HashMap::new(),
           revealed: HashSet::new(),
           last_move: 0,
        }
    }
}


#[derive(Debug,Clone,Copy,PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum GameState {
    Setup,
    Title,
    Background,
    Start,
    Running,
    Menu,
    Pause,
    End,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct MouseLocation {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug,Clone)]
pub struct TileEntityState {
    pub entities:Vec<Entity>,
    pub passable: bool,
    pub transparent: bool,
}

impl Default for TileEntityState {
    fn default() -> Self {
        Self{entities:vec![], passable:true, transparent:true}
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x:i32, y:i32) -> Self {
        Self{x,y}   
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32,self.y as f32, 0.0)
    }

    pub fn to_vec3_z(&self, z: f32) -> Vec3 {
        Vec3::new(self.x as f32,self.y as f32, z)
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

    pub fn inverse_x(&self) ->Position {
        Position{x:-self.x,y:self.y}
    }

    pub fn distance(&self, pos: &Position) -> i32 {
        (self.x-pos.x).abs().max((self.y-pos.y).abs())
    }

    
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dimension {
    topleft: Position,
    bottomright: Position,
}

impl Dimension {
    pub fn new(topleft: Position, bottomright:Position) -> Self{
        Self{topleft,bottomright}
    }

    pub fn contains(&self, pos: &Position) -> bool {
        pos.x>=self.topleft.x && pos.x<=self.bottomright.x
            && pos.y>=self.topleft.y && pos.y<=self.bottomright.y
    }

}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MapTile;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, EnumIter)]
pub enum PlayerPart {
    Body,
    Pants,
    Top,
    Hair,
    RightHand,
}


#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Player ;

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MainCamera;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Journal {
    pub quests: HashMap<String,Quest>,
    pub entries: Vec<JournalEntry>,
}


impl Default for Journal {
    fn default() -> Self {
        let q=Quest::new(QUEST_MAIN,"Main Quest");
        let mut j = Journal {quests:HashMap::new(), entries:vec![]};
        j.add_quest(q)
            .add_entry(QUEST_MAIN,"I have decided it, and nothing will alter my resolve. I will set up in search for Father. Peleus cannot stop me.");

        j
    }

}

impl Journal {
    pub fn add_quest<'a>(&'a mut self, quest:Quest) -> &'a mut Self {
        self.quests.insert(quest.code.clone(),quest);
        self
    } 

    pub fn add_entry<'a,S1: Into<String>, S2: Into<String>>(&'a mut self, quest: S1, text: S2) -> &'a mut Self {
        self.entries.push(JournalEntry::new(quest,text));
        self
    } 
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct JournalEntry {
    pub quest: String,
    pub text: String,
}

impl JournalEntry {
    pub fn new<S1: Into<String>, S2: Into<String>>(quest: S1, text: S2) -> Self {
        JournalEntry{quest:quest.into(),text:text.into()}
    }
}


#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Quest {
    pub code: String,
    pub text: String,
}

impl Quest {
    pub fn new<S1: Into<String>, S2: Into<String>>(code: S1, text: S2) -> Self {
        Quest{code:code.into(),text:text.into()}
    }
}

#[derive(Debug,Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub sprite: String,
    pub position: Position,
    pub dimension: Dimension,
    pub consumable: bool,
}

impl Item {
    pub fn new<S1: Into<String>,S2: Into<String>,S3: Into<String>>(name: S1, description: S2,sprite: S3, x1: i32, y1: i32) -> Self {
        Self{name: name.into(),description:description.into(),sprite:sprite.into(),position: sprite_position(x1, y1),dimension: sprite_dimensions(x1, y1,x1,y1), consumable: false}
    }

    pub fn new_consumable<S1: Into<String>,S2: Into<String>,S3: Into<String>>(name: S1, description: S2,sprite: S3, x1: i32, y1: i32) -> Self {
        let mut s=Item::new(name,description,sprite,x1,y1);
        s.consumable=true;
        s
    }
}

#[derive(Debug,Clone, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn add_item<'a>(&'a mut self, item: Item) -> &'a mut Self {
        self.items.push(item);
        self.items.sort_by_key(|i| i.description.clone());
        self
    }

    pub fn contains_item(&self, item: &str) -> bool {
        self.items.iter().any(|i| i.name==item)
    }

    pub fn remove_item<'a>(&'a mut self, item: &str) -> &'a mut Self {
        if let Some((ix,_e)) = self.items.iter().enumerate().filter(|(_ix,i)| i.name==item).next(){
            self.items.remove(ix);
        }
        self
    }
}


pub fn sprite_position(x: i32, y: i32) -> Position {
    Position::new(x*SPRITE_SIZE, y*SPRITE_SIZE)
}

pub fn sprite_dimensions(x1: i32, y1: i32, x2: i32, y2: i32) -> Dimension {
    Dimension::new(Position::new(x1*SPRITE_SIZE-SPRITE_SIZE/2, y1*SPRITE_SIZE-SPRITE_SIZE/2),Position::new(x2*SPRITE_SIZE+SPRITE_SIZE/2, y2*SPRITE_SIZE+SPRITE_SIZE/2))
}

#[derive(Debug,Clone, Default)]
pub struct Talents{
    pub animals: u32,
    pub people: u32,
    pub weapons: u32,
}

#[derive(Debug, Clone, Default)]
pub struct QuestFlags{
    flags: HashSet<(String,String)>,
}

impl QuestFlags {
    pub fn set_flag<'a, S1: Into<String>, S2: Into<String>>(&'a mut self, quest: S1, flag: S2) -> &'a mut Self {
        self.flags.insert((quest.into(),flag.into()));
        self
    }

    pub fn unset_flag<'a, S1: Into<String>, S2: Into<String>>(&'a mut self, quest: S1, flag: S2) -> &'a mut Self {
        self.flags.remove(&(quest.into(),flag.into()));
        self
    }

    pub fn has_flag<S1: Into<String>, S2: Into<String>>(&self, quest: S1, flag: S2) -> bool {
        self.flags.contains(&(quest.into(),flag.into()))
    }
}

#[derive(Debug,Clone)]
pub struct Spell {
    pub name: String,
    pub description: String,
}

impl Spell {
    pub fn new<S1: Into<String>,S2: Into<String>>(name: S1, description: S2) -> Self {
        Self{name: name.into(),description:description.into()}
    }

}


#[derive(Debug,Clone, Default)]
pub struct Spells {
    pub spells: Vec<Spell>,
}

impl Spells {
    pub fn add_spell<'a>(&'a mut self, spell: Spell) -> &'a mut Self {
        self.spells.push(spell);
        self.spells.sort_by_key(|i| i.description.clone());
        self
    }

    pub fn contains_spell(&self, spell: &str) -> bool {
        self.spells.iter().any(|i| i.name==spell)
    }

    pub fn remove_spell<'a>(&'a mut self, spell: &str) -> &'a mut Self {
        if let Some((ix,_e)) = self.spells.iter().enumerate().filter(|(_ix,i)| i.name==spell).next(){
            self.spells.remove(ix);
        }
        self
    }
}

pub struct BodyChangeEvent {
    pub part: PlayerPart,
    pub sprite: String,
}

impl BodyChangeEvent {
    pub fn new<S1: Into<String>>(part: PlayerPart, sprite:S1) -> Self {
        Self{part:part,sprite:sprite.into()}
    }
}