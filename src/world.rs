use crate::base::*;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct Area {
    pub name: String,
    pub map_index: usize,
    pub start: Position,
    pub rooms: HashMap<String, Room>,
    pub affordances: HashMap<String, Affordance>,
    pub items: HashMap<String, Item>,
    pub characters: HashMap<String, Character>,
}


impl Area {
    pub fn new<S: Into<String>>(name: S, map_index: usize, start: Position) -> Self {
        Self{name:name.into(),map_index,start,rooms:HashMap::new(),affordances:HashMap::new(),items:HashMap::new(),characters:HashMap::new()}
    }

    pub fn add_room<'a>(&'a mut self, room: Room) -> &'a mut Self {
        self.rooms.insert(room.name.clone(),room);
        self
    }

    pub fn room_from_position<'a>(&'a self, pos: &Position) -> Option<&'a Room> {
        self.rooms.values().filter(|r| r.contains(pos)).next()
    }

    pub fn room_from_coords<'a>(&'a self, x: f32, y:f32) -> Option<&'a Room> {
        self.room_from_position(&Position::new(x as i32, y as i32))
    }

    pub fn add_affordance<'a>(&'a mut self, aff: Affordance) -> &'a mut Self {
        self.affordances.insert(aff.name.clone(),aff);
        self
    }

    pub fn affordance_from_position<'a>(&'a self, pos: &Position) -> Option<&'a Affordance> {
        self.affordances.values().filter(|r| r.position.contains(pos)).next()
    }

    pub fn affordance_from_coords<'a>(&'a self, x: f32, y:f32) -> Option<&'a Affordance> {
        self.affordance_from_position(&Position::new(x as i32, y as i32))
    }

    pub fn add_character<'a>(&'a mut self, chr: Character) -> &'a mut Self {
        self.characters.insert(chr.name.clone(),chr);
        self
    }

    pub fn character_from_position<'a>(&'a self, pos: &Position) -> Option<&'a Character> {
        self.characters.values().filter(|r| r.dimension.contains(pos)).next()
    }

    pub fn character_from_coords<'a>(&'a self, x: f32, y:f32) -> Option<&'a Character> {
        self.character_from_position(&Position::new(x as i32, y as i32))
    }

    pub fn add_item<'a>(&'a mut self, item: Item) -> &'a mut Self {
        self.items.insert(item.name.clone(),item);
        self
    }

    pub fn item_from_position<'a>(&'a self, pos: &Position) -> Option<&'a Item> {
        self.items.values().filter(|r| r.dimension.contains(pos)).next()
    }

    pub fn item_from_coords<'a>(&'a self, x: f32, y:f32) -> Option<&'a Item> {
        self.item_from_position(&Position::new(x as i32, y as i32))
    }

    pub fn remove_item_from_pos<'a>(&'a mut self, pos: &Position) -> Option<Item> {
       if let Some(n)=self.item_from_position(pos).map(|i| i.name.clone()){
            self.items.remove(&n)
       } else {
           None
       }
    }
}

#[derive(Debug,Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub dimensions: Vec<Dimension>,

}

impl Room {
    pub fn new<S1: Into<String>,S2: Into<String>>(name: S1, description: S2, x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self{name: name.into(),description:description.into(), dimensions: vec![sprite_dimensions(x1, y1,x2,y2)]}
    }

    pub fn add_dimensions(mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Self{
        self.dimensions.push(sprite_dimensions(x1, y1,x2,y2));
        self
    }

    pub fn contains(&self, pos: &Position) -> bool {
        self.dimensions.iter().any(|d| d.contains(pos) )

    }
}


#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Affordance {
    pub name: String,
    pub description: String,
    pub position: Dimension,
}

impl Affordance {
    pub fn new<S1: Into<String>,S2: Into<String>>(name: S1, description: S2, x1: i32, y1: i32) -> Self {
        Self{name: name.into(),description:description.into(), position: sprite_dimensions(x1, y1,x1,y1)}
    }

}

#[derive(Debug,Clone)]
pub struct AffordanceEvent(pub String);


#[derive(Debug,Clone)]
pub struct ItemEvent(pub String);

#[derive(Debug,Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub sprite: String,
    pub position: Position,
    pub dimension: Dimension,
}


impl Character {
    pub fn new<S1: Into<String>,S2: Into<String>,S3: Into<String>>(name: S1, description: S2,sprite: S3, x1: i32, y1: i32) -> Self {
        Self{name: name.into(),description:description.into(),sprite:sprite.into(),position: sprite_position(x1, y1),dimension: sprite_dimensions(x1, y1,x1,y1)}
    }

}

#[derive(Debug,Clone)]
pub struct CharacterEvent(pub String);