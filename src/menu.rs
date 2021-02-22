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

impl Menus {
    pub fn push<'a>(&'a mut self, m: Menu) -> &'a mut Self {
        self.menus.push(m);
        self
    }

    pub fn pop<'a>(&'a mut self) -> Option<Menu> {
        self.menus.pop()
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Menu {
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
pub struct MenuItem {
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

pub fn main_menu() -> Menu {
    Menu::new(MAIN, "Anthea", vec![journal_item(), inventory_item(), talents_item()])
}

fn journal_menu(journal: &Journal) -> Menu {
    let e=journal.entries.last().unwrap();

    Menu::new(MAIN, "Journal",vec![MenuItem::new("",&e.text)])
}


fn inventory_menu(inventory: &Inventory) -> Menu {
    let mut msgs: Vec<MenuItem>=inventory.items.iter().map(|i|  MenuItem::new(&i.name,&i.description)).collect();
    if msgs.is_empty() {
        msgs.push(MenuItem::new("empty","Empty hands!"));
    }
    Menu::new(MAIN, "Inventory",msgs)
}


impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<MenuEvent>()
            .add_event::<MenuItemEvent>()
            .insert_resource(Menus::default())
            .add_system(menu_start.system())
            //.on_state_enter(STAGE, GameState::Menu,show_main_menu.system())
            .on_state_update(STAGE, GameState::Menu,click_system.system())
            .on_state_update(STAGE, GameState::Menu,journal_event.system())
            .on_state_update(STAGE, GameState::Menu,inventory_event.system())
            .on_state_update(STAGE, GameState::Menu,close_menu.system())
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

fn push_menu(
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    mut menus: ResMut<Menus>,
    menu: Menu,
){
    show_menu(clearm, queue, &menu);
    menus.push(menu);
}

/*fn show_main_menu(
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    menus: ResMut<Menus>,
){
    let m=main_menu();
    push_menu(clearm, queue, menus,m);
}*/

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
    mut menuqueue: ResMut<Events<MenuItemEvent>>,
){
    if let Some((interaction, _txt, item)) = item_query.iter().next() {
        if *interaction==Interaction::Clicked {
            let msg = &item.0;
            if CLOSE==msg {
                menus.pop();
                if let Some( m) = menus.menus.last(){
                    show_menu(clearm, queue, m);
                } else {
                    clearm.send(ClearMessage); 
                    appstate.set_next(GameState::Running).unwrap();
                }
            } else if let Some( m) = menus.menus.last(){
                menuqueue.send(MenuItemEvent{menu:m.code.clone(),item:msg.into()});
            }
            
        }
    }
}

fn close_menu(keyboard_input: Res<Input<KeyCode>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    mut appstate: ResMut<State<GameState>>,
    mut menus: ResMut<Menus>) {
    //for event in keyboard_input_events.iter() {
        if keyboard_input.just_released(KeyCode::Escape) {
            println!("Escape");
            menus.pop();
            if let Some( m) = menus.menus.last(){
                show_menu(clearm, queue, m);
            } else {
                clearm.send(ClearMessage); 
                appstate.set_next(GameState::Running).unwrap();
            }
        }
    //}
}

#[derive(Debug,Clone)]
pub struct MenuEvent{
    pub menu:Menu,
}

impl MenuEvent {
    pub fn new(m: Menu) -> Self {
        MenuEvent{menu:m}
    }
}

fn menu_start(    
    mut appstate: ResMut<State<GameState>>,
    mut event_reader: EventReader<MenuEvent>,
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    menus: ResMut<Menus>,
){
    if let Some(me) = event_reader.iter().next() {
        appstate.set_next(GameState::Menu).unwrap();
        push_menu(clearm, queue, menus,me.menu.clone());
    }
}

#[derive(Debug,Clone)]
pub struct MenuItemEvent{
    pub menu:String,
    pub item:String,
}

fn journal_event(    
    mut event_reader: EventReader<MenuItemEvent>,
    journal: Res<Journal>,
    menus: ResMut<Menus>,
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,) {
        if let Some(_e) = event_reader.iter().filter(|e| e.menu==MAIN && e.item==JOURNAL).next() {
            let m=journal_menu(&journal);
            push_menu(clearm, queue, menus,m);
        }
}


fn inventory_event(    
    mut event_reader: EventReader<MenuItemEvent>,
    inventory: Res<Inventory>,
    menus: ResMut<Menus>,
    clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,) {
        if let Some(_e) = event_reader.iter().filter(|e| e.menu==MAIN && e.item==INVENTORY).next() {
            let m=inventory_menu(&inventory);
            push_menu(clearm, queue, menus,m);
        }
}