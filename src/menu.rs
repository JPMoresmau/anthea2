use bevy::prelude::*;

use crate::base::*;
use crate::ui::*;

pub struct MenuPlugin;

pub const MAIN: &str = "main";
pub const JOURNAL: &str = "journal";
pub const INVENTORY: &str = "inventory";
pub const TALENTS: &str = "talents";

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Menus {
    menus: Vec<Menu>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Menu {
    code: String,
    title: String,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new<S1: Into<String>, S2: Into<String>>(code:S1, title:S2,items: Vec<MenuItem>) -> Self{
        Menu{code:code.into(),title:title.into(), items}
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct MenuItem {
    code: String,
    text: String,
}

impl MenuItem {
    pub fn new<S1: Into<String>, S2: Into<String>>(code:S1, text:S2) -> Self{
        MenuItem{code:code.into(),text:text.into()}
    }
}

fn journal_item() -> MenuItem {
    MenuItem::new(JOURNAL,"Journal")
}

fn inventory_item() -> MenuItem {
    MenuItem::new(INVENTORY,"Inventory")
}

fn talents_item() -> MenuItem {
    MenuItem::new(TALENTS,"Talents")
}

fn main_menu() -> Menu {
    Menu::new(MAIN, "Anthea", vec![journal_item(), inventory_item(), talents_item()])
}

fn journal_menu(journal: &Journal) -> Menu {
    let e=journal.entries.last().unwrap();

    Menu::new(MAIN, "Journal",vec![MenuItem::new("",&e.text)])
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<MenuEvent>()
            .insert_resource(Menus::default())
            .on_state_enter(STAGE, GameState::Menu,show_main_menu.system())
            .on_state_update(STAGE, GameState::Menu,click_system.system())
            .add_system(journal_event.system())
        ;
    }
}

fn show_menu(
    mut clearm: ResMut<Events<ClearMessage>>,
    mut queue: ResMut<Events<MessageEvent>>,
    menu: &Menu,
){
    clearm.send(ClearMessage); 
    let mut msgs = vec![
        Message::new(&menu.title,MessageStyle::MenuTitle),
    ];
    for mi in menu.items.iter(){
        msgs.push(Message::new(&mi.text,MessageStyle::Interaction(mi.code.clone())));
    }
    queue.send(MessageEvent::new_multi(msgs));
}

fn show_main_menu(
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    mut menus: ResMut<Menus>,
){
    let m=main_menu();
    show_menu(clearm, queue, &m);
    menus.menus.push(m);
}

/*fn click_system(mouse_button_input: Res<Input<MouseButton>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    mut appstate: ResMut<State<GameState>>,
    ) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        clearm.send(ClearMessage); 
        appstate.set_next(GameState::Running).unwrap();
    }
}*/

fn click_system(
    item_query: Query<(&Interaction,&Text,&InteractionItem),Mutated<Interaction>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    mut appstate: ResMut<State<GameState>>,
    mut menus: ResMut<Menus>,
    mut menuqueue: ResMut<Events<MenuEvent>>,
){
    if let Some((interaction, _txt, item)) = item_query.iter().next() {
        if *interaction==Interaction::Clicked {
            let msg = &item.0;
            if CLOSE==msg {
                menus.menus.pop();
                if let Some( m) = menus.menus.last(){
                    show_menu(clearm, queue, m);
                } else {
                    clearm.send(ClearMessage); 
                    appstate.set_next(GameState::Running).unwrap();
                }
            } else if let Some( m) = menus.menus.last(){
                menuqueue.send(MenuEvent{menu:m.code.clone(),item:msg.into()});
            }
            
        }
    }
}

#[derive(Debug,Clone)]
pub struct MenuEvent{
    pub menu:String,
    pub item:String,
}

fn journal_event(    
    mut event_reader: EventReader<MenuEvent>,
    journal: Res<Journal>,
    mut menus: ResMut<Menus>,
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,) {
        if let Some(_e) = event_reader.iter().filter(|e| e.menu==MAIN && e.item==JOURNAL).next() {
            let m=journal_menu(&journal);
            show_menu(clearm, queue,&m);
            menus.menus.push(m);
        }
}