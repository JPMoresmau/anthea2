use bevy::prelude::*;
use bevy::sprite::*;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

use crate::base::*;

pub struct MessageQueue {
    pub messages: Vec<Message>,
}

impl Default for MessageQueue {
    fn default() -> Self {
        MessageQueue{messages:vec![]}
    }
}
#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct Message {
    pub contents:String,
    pub location:MouseLocation,
}

#[derive(Debug,Clone,Copy,PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum MessageFramePart {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

pub struct Background;

const DIMENSIONS: &[((f32,f32),(f32,f32))]= &[
    ((857.0,192.0),(879.0,212.0)),
    ((893.0,192.0),(965.0,212.0)),
    ((978.0,192.0),(1000.0,212.0)),
    ((857.0,226.0),(870.0,282.0)),
    ((986.0,226.0),(1000.0,282.0)),
    ((857.0,293.0),(879.0,316.0)),
    ((893.0,293.0),(965.0,316.0)),
    ((978.0,293.0),(1000.0,316.0)),
];

pub fn setup_ui( commands: &mut Commands,
    sprite_handles: Res<AntheaHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mut atlas = TextureAtlas::new_empty(sprite_handles.ui_handle.clone(), Vec2::new(1024.0,666.0) );
    
    
    for ((x1,y1),(x2,y2)) in DIMENSIONS {
        atlas.add_texture(bevy::sprite::Rect{min:Vec2::new(*x1,*y1),max:Vec2::new(*x2,*y2)});
    }

    let texture_atlas_handle = texture_atlases.add(atlas);

    for (i,part) in MessageFramePart::iter().enumerate(){
        spawn_frame(commands, texture_atlas_handle.clone(), part, i);
    }

    commands.spawn(SpriteBundle {
        material: materials.add(sprite_handles.paper_handle.clone().into()),
        visible: Visible{is_transparent:true,is_visible:false},
        ..Default::default()
    })
    .with(Background);
}

fn spawn_frame(commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    part: MessageFramePart,
    tile_index: usize,
){
    
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(tile_index as u32),
        texture_atlas: texture_atlas_handle,
        visible: Visible{is_transparent:true,is_visible:false},
        ..Default::default()
    })
    .with(part);
}

pub fn message_system(mut queue: ResMut<MessageQueue>,){
    if let Some(msg) = queue.messages.pop(){
        println!("Message: {:?}",msg);
    }
}