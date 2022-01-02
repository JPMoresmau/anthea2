use crate::base::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Area {
    pub name: String,
    pub map_index: usize,
    pub start: Position,
    pub rooms: HashMap<String, Room>,
    pub affordances: HashMap<SpritePosition, Affordance>,
    pub items: HashMap<SpritePosition, Item>,
    pub characters: HashMap<SpritePosition, Character>,
}

impl Area {
    pub fn new<S: Into<String>>(name: S, map_index: usize, start: Position) -> Self {
        Self {
            name: name.into(),
            map_index,
            start,
            rooms: HashMap::new(),
            affordances: HashMap::new(),
            items: HashMap::new(),
            characters: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> &mut Self {
        self.rooms.insert(room.name.clone(), room);
        self
    }

    pub fn room_from_position<'a>(&'a self, pos: &SpritePosition) -> Option<&'a Room> {
        self.rooms.values().find(|r| r.contains(pos))
    }

    /*pub fn room_from_coords(&self, x: f32, y: f32) -> Option<&Room> {
        self.room_from_position(&Position::new(x as i32, y as i32))
    }*/

    pub fn add_affordance(&mut self, aff: Affordance) -> &mut Self {
        for pos in aff.dimension.positions().into_iter(){
            self.affordances.insert(pos, aff.clone());
        }
        self
    }

    pub fn affordance_from_position<'a>(&'a self, pos: &SpritePosition) -> Option<&'a Affordance> {
        self.affordances.get(pos)
    }

    /*
    pub fn affordance_from_coords(&self, x: f32, y: f32) -> Option<&Affordance> {
        self.affordance_from_position(&Position::new(x as i32, y as i32))
    }*/

    pub fn add_character(&mut self, chr: Character) -> &mut Self {
        self.characters.insert(chr.position.clone(), chr);
        self
    }

    pub fn character_from_position<'a>(&'a self, pos: &SpritePosition) -> Option<&'a Character> {
        self.characters.get(pos)
    }

    /*pub fn character_from_coords(&self, x: f32, y: f32) -> Option<&Character> {
        self.character_from_position(&Position::new(x as i32, y as i32))
    }*/

    pub fn add_item(&mut self, item: Item) -> &mut Self {
        self.items.insert(item.position.clone(), item);
        self
    }

    pub fn item_from_position<'a>(&'a self, pos: &SpritePosition) -> Option<&'a Item> {
        self.items.get(pos)
    }

    /*pub fn item_from_coords(&self, x: f32, y: f32) -> Option<&Item> {
        self.item_from_position(&Position::new(x as i32, y as i32))
    }

    pub fn remove_item_from_pos(&mut self, pos: &Position) -> Option<Item> {
        if let Some(n) = self.item_from_position(pos).map(|i| i.name.clone()) {
            self.items.remove(&n)
        } else {
            None
        }
    }*/
}

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub dimensions: Vec<SpriteDimension>,
}

impl Room {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        description: S2,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            dimensions: vec![SpriteDimension::new(SpritePosition::new(x1, y1), SpritePosition::new(x2, y2))],
        }
    }

    pub fn add_dimensions(mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        self.dimensions.push(SpriteDimension::new(SpritePosition::new(x1, y1), SpritePosition::new(x2, y2)));
        self
    }

    pub fn contains(&self, pos: &SpritePosition) -> bool {
        self.dimensions.iter().any(|d| d.contains(pos))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affordance {
    pub name: String,
    pub description: String,
    pub dimension: SpriteDimension,
}

impl Affordance {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        description: S2,
        x1: i32,
        y1: i32,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            dimension: SpriteDimension::new(SpritePosition::new(x1, y1), SpritePosition::new(x1, y1)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AffordanceEvent(pub String);

#[derive(Debug, Clone)]
pub struct ItemEvent(pub String);

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub sprite: String,
    pub position: SpritePosition,
}

impl Character {
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterEvent(pub String);
