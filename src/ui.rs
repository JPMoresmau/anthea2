use bevy::prelude::*;
use bevy::text::CalculatedSize;

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

pub struct MessageText;
pub struct ToFrame;

const DIMENSIONS: &[((f32,f32),(f32,f32))]= &[
    ((857.0,192.0),(879.0,212.0)),
    ((893.0,192.0),(965.0,212.0)),
    ((978.0,192.0),(1000.0,212.0)),
    ((857.0,226.0),(879.0,282.0)),
    ((978.0,226.0),(1000.0,282.0)),
    ((857.0,293.0),(879.0,316.0)),
    ((893.0,293.0),(965.0,316.0)),
    ((978.0,293.0),(1000.0,316.0)),
];

pub fn setup_ui( commands: &mut Commands,
    handles: Res<AntheaHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let mut atlas = TextureAtlas::new_empty(handles.ui_handle.clone(), Vec2::new(1024.0,666.0) );
    
    
    for ((x1,y1),(x2,y2)) in DIMENSIONS {
        atlas.add_texture(bevy::sprite::Rect{min:Vec2::new(*x1,*y1),max:Vec2::new(*x2,*y2)});
    }

    let texture_atlas_handle = texture_atlases.add(atlas);

    for (i,part) in MessageFramePart::iter().enumerate(){
        spawn_frame(commands, texture_atlas_handle.clone(), part, i);
    }

    commands.spawn(SpriteBundle {
        material: materials.add(handles.paper_handle.clone().into()),
        visible: Visible{is_transparent:true,is_visible:false},
        ..Default::default()
    })
    .with(Background);

    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
           style: Style {
               //border: bevy::prelude::Rect::all(Val::Px(10.0)),
                //align_self: AlignSelf::FlexStart,
                //position_type: PositionType::Absolute,
                /*position: bevy::prelude::Rect {
                    top: Val::Px(5.0),
                    //left: Val::Px(10.0),
                    ..Default::default()
                },*/
                //size: Size::new(Val::Percent(95.0), Val::Px(50.0)),
                //flex_grow: 1.0,
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                String::new(),
                TextStyle {
                    font: handles.font_handle.clone(),
                    font_size: 20.0,
                    color: Color::BLACK,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Top,
                },
            ),
            ..Default::default()
        })
        .with(MessageText);
    });

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

pub fn message_system(
    commands: &mut Commands,
    mut queue: ResMut<MessageQueue>,
    mut text_query: Query<(Entity, &MessageText, &mut Text)>){
    if let Some(msg) = queue.messages.pop(){
        println!("Message: {:?}",msg);
        for (e, _mt, mut text) in &mut text_query.iter_mut() {
            text.sections[0].value=msg.contents.clone();
            commands.insert_one(e,ToFrame);
        }
    }
}

pub fn message_decoration_system(
    commands: &mut Commands,
    text_query: Query<(Entity, &Text, &CalculatedSize, &ToFrame)>,
    mut bg_query: Query<(&Background, &mut Visible, &mut Transform)>,
    mut part_query: Query<(&MessageFramePart, &mut Visible, &mut Transform)>
){
    for (e, t,cs, _tf) in text_query.iter(){
        if t.sections[0].value.len()>0{
            let w = cs.size.width+20.0;
            let h = cs.size.height+20.0;
            for (_b,mut v,mut tr) in bg_query.iter_mut(){
                v.is_visible=true;
                tr.scale.x=(w+20.0)/512.0;
                tr.scale.y=(h+20.0)/512.0;
            }
            for (fp,mut v,mut tr) in part_query.iter_mut(){
                v.is_visible=true;
                match fp {
                    MessageFramePart::TopLeft => {
                        tr.translation.x=-w/2.0;
                        tr.translation.y=h/2.0;
                        tr.translation.z=1.0;
                    },
                    MessageFramePart::Top => {
                        tr.translation.x=0.0;
                        tr.translation.y=h/2.0;
                        tr.translation.z=1.0;
                        tr.scale.x=(w-20.0)/72.0;
                    },
                    MessageFramePart::TopRight => {
                        tr.translation.x=w/2.0;
                        tr.translation.y=h/2.0;
                        tr.translation.z=1.0;
                    },
                    MessageFramePart::Left => {
                        tr.translation.x=-w/2.0;
                        tr.translation.y=0.0;
                        tr.translation.z=1.0;
                        tr.scale.y=(h-18.0)/56.0;
                    },
                    MessageFramePart::Right => {
                        tr.translation.x=w/2.0;
                        tr.translation.y=0.0;
                        tr.translation.z=1.0;
                        tr.scale.y=(h-18.0)/56.0;
                    },
                    MessageFramePart::BottomLeft => {
                        tr.translation.x=-w/2.0;
                        tr.translation.y=-h/2.0;
                        tr.translation.z=1.0;
                    },
                    MessageFramePart::Bottom => {
                        tr.translation.x=0.0;
                        tr.translation.y=-h/2.0;
                        tr.translation.z=1.0;
                        tr.scale.x=(w-20.0)/72.0;
                    },
                    MessageFramePart::BottomRight => {
                        tr.translation.x=w/2.0;
                        tr.translation.y=-h/2.0;
                        tr.translation.z=1.0;
                    },
                }
                
            }
        }
        commands.remove_one::<ToFrame>(e);
    }
}