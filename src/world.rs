use crate::base::*;

use std::collections::HashMap;

pub struct Area {
    pub name: String,
    pub map_index: usize,
    pub start: Position,
    pub rooms: HashMap<String, Room>,
    pub affordances: HashMap<String, Affordance>,
    pub items: HashMap<String, Item>,
}


impl Area {
    pub fn new<S: Into<String>>(name: S, map_index: usize, start: Position) -> Self {
        Self{name:name.into(),map_index,start,rooms:HashMap::new(),affordances:HashMap::new(),items:HashMap::new()}
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


#[derive(Debug,Clone)]
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
pub struct Item {
    pub name: String,
    pub description: String,
    pub sprite: String,
    pub position: Position,
    pub dimension: Dimension,
}

impl Item {
    pub fn new<S1: Into<String>,S2: Into<String>,S3: Into<String>>(name: S1, description: S2,sprite: S3, x1: i32, y1: i32) -> Self {
        Self{name: name.into(),description:description.into(),sprite:sprite.into(),position: sprite_position(x1, y1),dimension: sprite_dimensions(x1, y1,x1,y1)}
    }

}


#[derive(Debug,Clone)]
pub struct ItemEvent(pub String);


pub fn sprite_position(x: i32, y: i32) -> Position {
    Position::new(x*SPRITE_SIZE, y*SPRITE_SIZE)
}

pub fn sprite_dimensions(x1: i32, y1: i32, x2: i32, y2: i32) -> Dimension {
    Dimension::new(Position::new(x1*SPRITE_SIZE-SPRITE_SIZE/2, y1*SPRITE_SIZE-SPRITE_SIZE/2),Position::new(x2*SPRITE_SIZE+SPRITE_SIZE/2, y2*SPRITE_SIZE+SPRITE_SIZE/2))
}


