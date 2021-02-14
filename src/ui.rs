use bevy::prelude::*;
use bevy::text::CalculatedSize;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::base::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<ClearMessage>()
            .add_event::<MessageEvent>()
            .add_system(message_system.system())
            .add_system(message_decoration_system.system())
            .add_system(message_clear_system.system())
        ;
    }
}
#[derive(Debug,Clone,Copy,PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum MessageStyle {
    Title,
    Info,
    Interaction,
    Help,
}


#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct MessageEvent {
    pub messages: Vec<Message>
}

impl MessageEvent {
    pub fn new<S: Into<String>>(msg: S, style: MessageStyle) -> Self {
        MessageEvent{messages:vec![Message{contents:msg.into(),style}]}
    }

    pub fn new_multi(msgs: Vec<Message>) -> Self {
        MessageEvent{messages:msgs}
    }
}

#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct Message {
    pub contents:String,
    pub style: MessageStyle,
}

#[derive(Debug,Clone,Copy,PartialEq, Eq, PartialOrd, Ord,Default)]
pub struct ClearMessage;

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
    mut queue: ResMut<Events<MessageEvent>>,
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
            border: bevy::prelude::Rect::all(Val::Px(25.0)),
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
                //border: bevy::prelude::Rect::all(Val::Px(50.0)),
                align_self: AlignSelf::FlexStart,

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
            text: Text{sections:vec![], alignment:
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Top,
                }
            },
            ..Default::default()
        })
        .with(MessageText);
    });
    queue.send(MessageEvent::new_multi(vec![
        Message{contents:"Anthea's Quest".to_owned(), style: MessageStyle::Title},
        Message{contents:"Click to start".to_owned(), style: MessageStyle::Help}]));
        
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

fn build_section(msg: &Message, font: Handle<Font>, sep: &str) -> TextSection {
    let font_size = match msg.style {
        MessageStyle::Title=> 72.0,
        MessageStyle::Help => 12.0,
        _ => 20.0,
    };
    let style=TextStyle {
        font,
        font_size,
        color: Color::BLACK,
    }; 

    TextSection {
        value: format!("{}{}",sep,msg.contents),
        style,
    }
}

fn message_system(
    handles: Res<AntheaHandles>,
    commands: &mut Commands,
    mut event_reader: EventReader<MessageEvent>,
    mut text_query: Query<(Entity, &MessageText, &mut Text, &mut Style)>){
        for me in event_reader.iter() {
            for (e, _mt, mut text, mut style) in &mut text_query.iter_mut() {
                style.align_self=Default::default();
                let mut sep=String::new();
                text.sections.clear();
                for msg in me.messages.iter(){
                    //println!("Message: {:?}",msg);
                    text.sections.push(build_section(&msg,handles.font_handle.clone(),&sep));
                    if msg.style==MessageStyle::Info {
                        style.align_self= AlignSelf::FlexStart;
                    }
                    if sep.is_empty(){
                        sep="\n".into();
                    }
                }
                commands.insert_one(e,ToFrame);
            }
           
        }
        
}

fn message_decoration_system(
    commands: &mut Commands,
    text_query: Query<(Entity, &Text, &CalculatedSize, &Transform),With<ToFrame>>,
    mut bg_query: Query<(&Background, &mut Visible, &mut Transform)>,
    mut part_query: Query<(&MessageFramePart, &mut Visible, &mut Transform)>
){
    for (e, t,cs,ttr) in text_query.iter(){
        if t.sections.len()>0 && t.sections[0].value.len()>0{
            //println!("text transform: {:?}",ttr.translation);
            let w = cs.size.width+20.0;
            let h = cs.size.height+20.0;
            let mut z = 0.5;
            for (_b,mut v,mut tr) in bg_query.iter_mut(){
                v.is_visible=true;
                tr.scale.x=(w+20.0)/512.0;
                tr.scale.y=(h+20.0)/512.0;
                tr.translation.x=ttr.translation.x;
                tr.translation.y=ttr.translation.y;
                tr.translation.z=z;
            }
            z = 0.6;
            //ttr.translation.z=z+1.0;
            for (fp,mut v,mut tr) in part_query.iter_mut(){
                v.is_visible=true;
                
                match fp {
                    MessageFramePart::TopLeft => {
                        tr.translation.x=-w/2.0;
                        tr.translation.y=h/2.0;
                        tr.translation.z=z;
                    },
                    MessageFramePart::Top => {
                        tr.translation.x=0.0;
                        tr.translation.y=h/2.0;
                        tr.translation.z=z;
                        tr.scale.x=(w-20.0)/72.0;
                    },
                    MessageFramePart::TopRight => {
                        tr.translation.x=w/2.0;
                        tr.translation.y=h/2.0;
                        tr.translation.z=z;
                    },
                    MessageFramePart::Left => {
                        tr.translation.x=-w/2.0;
                        tr.translation.y=0.0;
                        tr.translation.z=z;
                        tr.scale.y=(h-16.0)/56.0;
                    },
                    MessageFramePart::Right => {
                        tr.translation.x=w/2.0;
                        tr.translation.y=0.0;
                        tr.translation.z=z;
                        tr.scale.y=(h-16.0)/56.0;
                    },
                    MessageFramePart::BottomLeft => {
                        tr.translation.x=-w/2.0;
                        tr.translation.y=-h/2.0;
                        tr.translation.z=z;
                    },
                    MessageFramePart::Bottom => {
                        tr.translation.x=0.0;
                        tr.translation.y=-h/2.0;
                        tr.translation.z=z;
                        tr.scale.x=(w-18.0)/72.0;
                    },
                    MessageFramePart::BottomRight => {
                        tr.translation.x=w/2.0;
                        tr.translation.y=-h/2.0;
                        tr.translation.z=z;
                    },
                }
                tr.translation.x+=ttr.translation.x;
                tr.translation.y+=ttr.translation.y;
                
            }
            commands.remove_one::<ToFrame>(e);
        }
       
    }
}

fn message_clear_system(    
    mut event_reader: EventReader<ClearMessage>,
    mut bg_query: Query<(&Background, &mut Visible)>,
    mut part_query: Query<(&MessageFramePart, &mut Visible)>,
    mut text_query: Query<(&MessageText, &mut Text)>,
    ){
        for _ev in event_reader.iter() {
            //println!("clear");
            for (_b,mut v) in bg_query.iter_mut() {
                v.is_visible=false;
            }
            for (_p, mut v) in part_query.iter_mut() {
                v.is_visible=false;
            }
            for (_p, mut t) in text_query.iter_mut() {
                t.sections.clear()
            }
        }
}
