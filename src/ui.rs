use bevy::text::CalculatedSize;
use bevy::{prelude::*, ui::FocusPolicy};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::base::*;

pub struct UIPlugin;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, StageLabel)]
pub struct AfterPostUpdate;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<ClearMessage>()
            .add_event::<MessageEvent>()
            .add_system(message_system.system())
            .add_stage_after(
                CoreStage::PostUpdate,
                AfterPostUpdate,
                SystemStage::parallel(),
            )
            .add_system_to_stage(
                AfterPostUpdate,
                message_decoration_system.system(),
            )
            //.add_system_to_stage(CoreStage::PostUpdate, message_decoration_system.system())
            //.add_system(message_decoration_system.system())
            .add_system_to_stage(CoreStage::PreUpdate, message_clear_system.system())
            //.add_system(message_clear_system.system())
        ;
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessageStyle {
    Title,
    MenuTitle,
    Info,
    Interaction(String),
    Navigation(bool, bool),
    Table(Vec<String>),
    Help,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MessageEvent {
    pub messages: Vec<Message>,
}

impl MessageEvent {
    pub fn new<S: Into<String>>(msg: S, style: MessageStyle) -> Self {
        MessageEvent {
            messages: vec![Message::new(msg, style)],
        }
    }

    pub fn new_multi(msgs: Vec<Message>) -> Self {
        MessageEvent { messages: msgs }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Message {
    pub contents: String,
    pub style: MessageStyle,
}

impl Message {
    pub fn new<S: Into<String>>(msg: S, style: MessageStyle) -> Self {
        Message {
            contents: msg.into(),
            style,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ClearMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum NavigationPart {
    Back,
    Forward,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Background;

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MessageText;
#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct InteractionItem(pub String);

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableItem;


const DIMENSIONS: &[((f32, f32), (f32, f32))] = &[
    // borders
    ((857.0, 192.0), (879.0, 212.0)),
    ((893.0, 192.0), (965.0, 212.0)),
    ((978.0, 192.0), (1000.0, 212.0)),
    ((857.0, 226.0), (879.0, 282.0)),
    ((978.0, 226.0), (1000.0, 282.0)),
    ((857.0, 293.0), (879.0, 316.0)),
    ((893.0, 293.0), (965.0, 316.0)),
    ((978.0, 293.0), (1000.0, 316.0)),
    // navigations
    ((560.0, 122.0), (590.0, 152.0)),
    ((598.0, 122.0), (628.0, 152.0)),
    ((560.0, 174.0), (590.0, 204.0)),
    ((598.0, 174.0), (628.0, 204.0)),
];

pub fn setup_ui(
    commands: &mut Commands,
    mut handles: ResMut<AntheaHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut state: ResMut<State<GameState>>,
) {
    let mut atlas = TextureAtlas::new_empty(handles.ui_handle.clone(), Vec2::new(1024.0, 666.0));

    for ((x1, y1), (x2, y2)) in DIMENSIONS {
        atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(*x1, *y1),
            max: Vec2::new(*x2, *y2),
        });
    }

    let texture_atlas_handle = texture_atlases.add(atlas);
    handles.ui_texture_atlas_handle = texture_atlas_handle.clone();
    for (i, part) in MessageFramePart::iter().enumerate() {
        spawn_frame(commands, texture_atlas_handle.clone(), part, i);
    }

    commands
        .spawn(SpriteBundle {
            material: materials.add(handles.paper_handle.clone().into()),
            visible: Visible {
                is_transparent: true,
                is_visible: false,
            },
            ..Default::default()
        })
        .with(Background);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                border: bevy::prelude::Rect::all(Val::Px(25.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        //border: bevy::prelude::Rect::all(Val::Px(50.0)),
                        align_self: AlignSelf::Center,
                        //flex_wrap: FlexWrap::Wrap,
                        //margin: Default::default(),
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
        Message {
            contents: "Anthea's Quest".to_owned(),
            style: MessageStyle::Title,
        },
        Message {
            contents: "Click to start".to_owned(),
            style: MessageStyle::Help,
        },
    ]));
    state.set_next(GameState::Background).unwrap();
}

fn spawn_frame(
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    part: MessageFramePart,
    tile_index: usize,
) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(tile_index as u32),
            texture_atlas: texture_atlas_handle,
            visible: Visible {
                is_transparent: true,
                is_visible: false,
            },
            ..Default::default()
        })
        .with(part);
}

/*fn spawn_navigation(commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    part: NavigationPart,
    tile_index: usize,
){
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(tile_index as u32),
        texture_atlas: texture_atlas_handle,
        visible: Visible{is_transparent:true,is_visible:false},
        ..Default::default()
    })
    .with(part);

}*/

fn build_section(msg: &Message, font: Handle<Font>, sep: &str) -> TextSection {
    let font_size = match msg.style {
        MessageStyle::Title => 72.0,
        MessageStyle::Help => 16.0,
        MessageStyle::MenuTitle => 32.0,
        _ => 20.0,
    };
    let style = TextStyle {
        font,
        font_size,
        color: Color::BLACK,
    };
    let txt = format!("{}{}", sep, msg.contents);
    TextSection { value: txt, style }
}

fn message_system(
    handles: Res<AntheaHandles>,
    commands: &mut Commands,
    mut event_reader: EventReader<MessageEvent>,
    mut text_query: Query<(&MessageText, &mut Text, &mut Style, &Parent)>,
    mut style_query: Query<&mut Style, Without<Text>>,
    bg_query: Query<(&Background, &mut Visible)>,
    part_query: Query<(&MessageFramePart, &mut Visible)>,
    nav_query: Query<(&Draw, &CalculatedSize, &Transform, &Parent), With<NavigationPart>>,
    menu_query: Query<Entity, With<InteractionItem>>,
    table_query: Query<Entity, With<TableItem>>,
) {
    if let Some(me) = event_reader.iter().next() {
        clear(
            commands,
            bg_query,
            part_query,
            &mut text_query,
            nav_query,
            menu_query,
            table_query
        );

        for (_mt, mut text, mut style, parent) in &mut text_query.iter_mut() {
            let mut ps = style_query.get_mut(parent.0).unwrap();
            ps.justify_content = JustifyContent::Center;
            style.align_self = Default::default();
            let mut sep = String::new();
            text.sections.clear();
           
           
            commands.set_current_entity(parent.0);
            commands.with_children(|parent| {
                let mut needs_close=false;
                for msg in me.messages.iter() {
                    //println!("Message: {:?}",msg);

                    if let MessageStyle::Navigation(backward, forward) = &msg.style {
                        build_navigation(parent, &handles, (*backward,*forward));
                    } else if let MessageStyle::Table(data) = &msg.style {
                        build_table(parent, &handles, &msg.contents, data);
                        needs_close = true;
                    } else {
                        let ts = build_section(&msg, handles.font_handle.clone(), &sep);
                        if let MessageStyle::Interaction(code) = &msg.style {
                           build_interaction(parent,ts,code);
                           needs_close = true;
                        } else {
                            text.sections.push(ts);
                            if msg.style == MessageStyle::Info {
                                ps.justify_content = JustifyContent::FlexEnd;
                            }
                            if sep.is_empty() {
                                sep = "\n".into();
                            }
                        }
                    }
                }
                if needs_close{
                    build_close(parent, &handles);
                }
            });
          
        }
    }
}

fn build_interaction<S1:Into<String>>(parent: &mut ChildBuilder, ts: TextSection, code: S1){
    parent
        .spawn(TextBundle {
            style: Style {
                //align_self: AlignSelf::Center,
                margin: bevy::prelude::Rect::all(Val::Px(5.0)),
                max_size: Size::new(
                    Val::Px(SCREEN_WIDTH as f32 * 0.8),
                    Val::Px(SCREEN_HEIGHT as f32 * 0.8),
                ),
                ..Default::default()
            },
            focus_policy: bevy::ui::FocusPolicy::Block,
            text: Text {
                sections: vec![ts],
                alignment: TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(Interaction::None)
        .with(InteractionItem(code.into()));
}


fn build_table<S1: Into<String>>(parent: &mut ChildBuilder,handles: &Res<AntheaHandles>,fst: S1, data:&Vec<String>){
    parent.spawn(NodeBundle {
        style: Style {
            margin: bevy::prelude::Rect::all(Val::Px(5.0)),
            max_size: Size::new(
                Val::Px(SCREEN_WIDTH as f32 * 0.4),
                Val::Px(16.0),
            ),
            size: Size::new(
                Val::Px(90.0),
                Val::Px(16.0),
            ),
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        visible: Visible {
            is_visible: false,
            is_transparent: true,
        },
        ..Default::default()
    })
    .with(TableItem)
    .with_children(|np| {
        for txt in std::iter::once(&fst.into()).chain(data.iter()){
            np.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![build_section(
                        &Message::new(txt, MessageStyle::Help),
                        handles.font_handle.clone(),
                        "",
                    )],
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    });
}


fn build_close(parent: &mut ChildBuilder,handles: &Res<AntheaHandles>,){
    parent
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            focus_policy: bevy::ui::FocusPolicy::Block,
            text: Text {
                sections: vec![build_section(
                    &Message::new(CLOSE, MessageStyle::Help),
                    handles.font_handle.clone(),
                    "",
                )],
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Interaction::None)
        .with(InteractionItem(CLOSE.into()));
}

fn build_navigation(parent: &mut ChildBuilder,handles: &Res<AntheaHandles>,(backward,forward):(bool,bool)){
    let l = MessageFramePart::iter().len();
    let btile = if backward { l } else { l + 2 };
    let ftile = if forward { l + 1 } else { l + 3 };
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            ..Default::default()
        })
        .with_children(|np| {
            np.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(btile as u32),
                texture_atlas: handles.ui_texture_atlas_handle.clone(),
                ..Default::default()
            })
            .with(Style::default())
            .with(CalculatedSize {
                size: Size::new(30.0, 30.0),
            })
            .with(Node::default())
            .with(NavigationPart::Back);
            if backward {
                np.with(FocusPolicy::Block).with(Interaction::None);
            }
            np.spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(ftile as u32),
                texture_atlas: handles.ui_texture_atlas_handle.clone(),
                ..Default::default()
            })
            .with(Style::default())
            .with(CalculatedSize {
                size: Size::new(30.0, 30.0),
            })
            .with(Node::default())
            .with(NavigationPart::Forward);
            if forward {
                np.with(FocusPolicy::Block).with(Interaction::None);
            }
        });
}

fn message_decoration_system(
    text_query: Query<(&Text, &CalculatedSize, &Transform, &MessageText), Mutated<CalculatedSize>>,
    item_query: Query<(&Text, &CalculatedSize, &Transform), With<InteractionItem>>,
    nav_query: Query<(&Draw, &CalculatedSize, &Transform), With<NavigationPart>>,
    table_query: Query<(&Node,&Children), With<TableItem>>,
    cs_query: Query<(&Draw, &CalculatedSize, &Transform)>,
    mut bg_query: Query<(
        &Background,
        &mut Visible,
        &mut Transform,
        &mut GlobalTransform,
    )>,
    mut part_query: Query<(
        &MessageFramePart,
        &mut Visible,
        &mut Transform,
        &mut GlobalTransform,
    )>,
) {
    for (t, cs, ttr,_mt) in text_query.iter() {
        if t.sections.len() > 0 && t.sections[0].value.len() > 0 {
            
            // println!("CalculatedSize: {:?}",cs);
            let mut max_w: f32 = 0.0;
            let mut add_y = 0.0;
            for (_t, cs, _tr) in item_query.iter() {
                max_w = max_w.max(cs.size.width);
                add_y += cs.size.height + 10.0;
            }
            let mut horiz_w: f32=0.0;
            let mut max_y: f32 = 0.0;
            for (_d, cs, _tr) in nav_query.iter() {
                max_y = max_y.max(cs.size.height);
                horiz_w +=cs.size.width;
            }
            max_w = max_w.max(horiz_w);
            add_y+=max_y;
            
            for (_n, chs) in table_query.iter(){
                horiz_w=0.0;
                max_y = 0.0;
                for e in chs.iter() {
                    if let Ok((_d,cs,_tr))=cs_query.get(*e){
                        //println!("Table query: {:?}",cs);
                        max_y = max_y.max(cs.size.height);
                        horiz_w +=cs.size.width;
                    }
                }
                max_w = max_w.max(horiz_w);
                add_y+=max_y+10.0;
                //println!("CalculatedSize.add_y: {:?}",add_y);
            }

            //println!("CalculatedSize.add_y: {:?}",add_y);
            //println!("text transform: {:?}",ttr.translation);
            let w = max_w.max(cs.size.width) + 20.0;
            let h = cs.size.height + 20.0 + add_y;

            let mut z = 0.5;
            for (_b, mut v, mut tr, mut gtr) in bg_query.iter_mut() {
                v.is_visible = true;
                tr.scale.x = (w + 20.0) / 512.0;
                tr.scale.y = (h + 20.0) / 512.0;
                tr.translation.x = ttr.translation.x;
                tr.translation.y = ttr.translation.y - add_y / 2.0;
                tr.translation.z = z;
                gtr.scale = tr.scale;
                gtr.translation = tr.translation;
            }
            z = 0.6;
            //ttr.translation.z=z+1.0;
            for (fp, mut v, mut tr, mut gtr) in part_query.iter_mut() {
                v.is_visible = true;

                match fp {
                    MessageFramePart::TopLeft => {
                        tr.translation.x = -w / 2.0;
                        tr.translation.y = h / 2.0;
                        tr.translation.z = z;
                    }
                    MessageFramePart::Top => {
                        tr.translation.x = 0.0;
                        tr.translation.y = h / 2.0;
                        tr.translation.z = z;
                        tr.scale.x = (w - 20.0) / 72.0;
                    }
                    MessageFramePart::TopRight => {
                        tr.translation.x = w / 2.0;
                        tr.translation.y = h / 2.0;
                        tr.translation.z = z;
                    }
                    MessageFramePart::Left => {
                        tr.translation.x = -w / 2.0;
                        tr.translation.y = 0.0;
                        tr.translation.z = z;
                        tr.scale.y = (h - 16.0) / 56.0;
                    }
                    MessageFramePart::Right => {
                        tr.translation.x = w / 2.0;
                        tr.translation.y = 0.0;
                        tr.translation.z = z;
                        tr.scale.y = (h - 16.0) / 56.0;
                    }
                    MessageFramePart::BottomLeft => {
                        tr.translation.x = -w / 2.0;
                        tr.translation.y = -h / 2.0;
                        tr.translation.z = z;
                    }
                    MessageFramePart::Bottom => {
                        tr.translation.x = 0.0;
                        tr.translation.y = -h / 2.0;
                        tr.translation.z = z;
                        tr.scale.x = (w - 18.0) / 72.0;
                    }
                    MessageFramePart::BottomRight => {
                        tr.translation.x = w / 2.0;
                        tr.translation.y = -h / 2.0;
                        tr.translation.z = z;
                    }
                }
                tr.translation.x += ttr.translation.x;
                tr.translation.y += ttr.translation.y;
                tr.translation.y -= add_y / 2.0;
                gtr.translation = tr.translation;
                gtr.scale = tr.scale;
            }
           
        }
    }
}

fn message_clear_system(
    commands: &mut Commands,
    mut event_reader: EventReader<ClearMessage>,
    bg_query: Query<(&Background, &mut Visible)>,
    part_query: Query<(&MessageFramePart, &mut Visible)>,
    mut text_query: Query<(&MessageText, &mut Text, &mut Style, &Parent)>,
    nav_query: Query<(&Draw, &CalculatedSize, &Transform, &Parent), With<NavigationPart>>,
    menu_query: Query<Entity, With<InteractionItem>>,
    table_query: Query<Entity, With<TableItem>>,
) {
    if let Some(_ev) = event_reader.iter().next() {
        //println!("clear");
        clear(
            commands,
            bg_query,
            part_query,
            &mut text_query,
            nav_query,
            menu_query,
            table_query,
        );
    }
}

fn clear(
    commands: &mut Commands,
    mut bg_query: Query<(&Background, &mut Visible)>,
    mut part_query: Query<(&MessageFramePart, &mut Visible)>,
    text_query: &mut Query<(&MessageText, &mut Text, &mut Style, &Parent)>,
    nav_query: Query<(&Draw, &CalculatedSize, &Transform, &Parent), With<NavigationPart>>,
    menu_query: Query<Entity, With<InteractionItem>>,
    table_query: Query<Entity, With<TableItem>>,
) {
    for (_b, mut v) in bg_query.iter_mut() {
        v.is_visible = false;
    }
    for (_p, mut v) in part_query.iter_mut() {
        v.is_visible = false;
    }
    for (_m, mut t, _s, _p) in text_query.iter_mut() {
        t.sections.clear()
    }
    for (_m, _cs, _t, p) in nav_query.iter().take(1) {
        commands.despawn_recursive(p.0);
    }
    for e in menu_query.iter() {
        commands.despawn_recursive(e);
    }
    for e in table_query.iter() {
        commands.despawn_recursive(e);
    }
}
