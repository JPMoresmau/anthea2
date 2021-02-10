use crate::base::*;

use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Stage {
    pub name: String,
    pub map_index: usize,
    pub start: Position,
    pub rooms: HashMap<String, Room>,
}

impl Stage {
    pub fn new<S: Into<String>>(name: S, map_index: usize, start: Position) -> Self {
        Self{name:name.into(),map_index,start,rooms:HashMap::new()}
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
}

#[derive(Debug,Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub dimensions: (Position,Position),
}

impl Room {
    pub fn new<S1: Into<String>,S2: Into<String>>(name: S1, description: S2, x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self{name: name.into(),description:description.into(), dimensions:sprite_dimensions(x1, y1,x2,y2)}
    }


    pub fn contains(&self, pos: &Position) -> bool {
        pos.x>=self.dimensions.0.x && pos.x<=self.dimensions.1.x
        && pos.y>=self.dimensions.0.y && pos.y<=self.dimensions.1.y

    }
}

fn sprite_position(x: i32, y: i32) -> Position {
    Position::new(x*SPRITE_SIZE, y*SPRITE_SIZE)
}

fn sprite_dimensions(x1: i32, y1: i32, x2: i32, y2: i32) -> (Position,Position) {
    (Position::new(x1*SPRITE_SIZE-SPRITE_SIZE/2, y1*SPRITE_SIZE-SPRITE_SIZE/2),Position::new(x2*SPRITE_SIZE+SPRITE_SIZE/2, y2*SPRITE_SIZE+SPRITE_SIZE/2))
}

pub fn stage1() -> Stage {
    let mut stage = Stage::new("Selaion Palace",0, sprite_position(-4,4));
    let bedroom = Room::new("bedroom", "Your bedroom",3,2,9,6);
    let throne = Room::new("throne", "Selaion throne room", 11, 2, 26, 6);
   
    stage.add_room(bedroom)
        .add_room(throne);


    stage
}