use bevy::prelude::*;
use std::{ collections::{HashMap, HashSet}};

use strum_macros::EnumIter; 

use crate::tiled::*;

pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 480;

pub const SPRITE_SIZE: i32 = 32;


pub const START_MAP_POSITION: Position = Position{x:-4*SPRITE_SIZE,y:4*SPRITE_SIZE};

pub const VISIBILITY_DISTANCE: f32 = 4.0 * SPRITE_SIZE as f32;

pub const MOVE_DELAY: u128 = 200;



#[derive(Default)]
pub struct AntheaHandles {
    pub people_handles: Vec<HandleUntyped>,
    pub tiles_handles: Vec<HandleUntyped>,
    pub tileset_handle: Handle<TileSet>,
    pub map_handles: Vec<Handle<Map>>,
    pub ui_handle: Handle<Texture>,
    pub paper_handle: Handle<Texture>,
    pub font_handle: Handle<Font>,
}


pub struct AntheaState {
    //player_position: Position,
    pub map_position: Position,
    pub positions: HashMap<Position,TileEntityState>,
    pub last_move: u128,
    pub game_state: GameState,
}

impl Default for AntheaState {
    fn default() -> Self {
        Self {
           //player_position: Position::default(),
           map_position: START_MAP_POSITION,
           positions: HashMap::new(),
           last_move: 0,
           game_state: GameState::Start,
        }
    }
}


#[derive(Debug,Clone,Copy,PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum GameState {
    Start,
    Running,
    Pause,
    End,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct MouseLocation {
    pub x: f32,
    pub y: f32,
}

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

pub struct PlayerPart {
    pub part: Part,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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
pub struct MapTile;

pub enum Part {
    BODY,
    PANTS,
    TOP,
    HAIR,
}