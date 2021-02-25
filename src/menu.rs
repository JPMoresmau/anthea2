use crate::base::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct MenuPlugin;

pub const MAIN: &str = "main";
pub const JOURNAL: &str = "journal";
pub const INVENTORY: &str = "inventory";
pub const TALENTS: &str = "talents";

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Menus {
    menus: Vec<Menu>,
    pub journal_index: Option<usize>,
}

impl Menus {
    pub fn push<'a>(&'a mut self, m: Menu) -> &'a mut Self {
        self.menus.push(m);
        self
    }

    pub fn pop<'a>(&'a mut self) -> Option<Menu> {
        self.menus.pop()
    }

    pub fn clear<'a>(&'a mut self) -> &'a mut Self {
        self.menus.clear();
        self.journal_index = None;
        self
    }

    pub fn current<'a>(&'a self) -> &'a String {
        &self.menus.iter().last().unwrap().code
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Menu {
    code: String,
    title: String,
    navigation: Option<(bool, bool)>,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        code: S1,
        title: S2,
        items: Vec<MenuItem>,
    ) -> Self {
        Menu {
            code: code.into(),
            title: title.into(),
            navigation: None,
            items,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MenuItem {
    code: String,
    text: String,
    extra: Option<String>,
}

impl MenuItem {
    pub fn new<S1: Into<String>, S2: Into<String>>(code: S1, text: S2) -> Self {
        MenuItem {
            code: code.into(),
            text: text.into(),
            extra: None,
        }
    }

    pub fn new_table<S1: Into<String>, S2: Into<String>>(text: S1, extra: S2) -> Self {
        MenuItem {
            code: String::new(),
            text: text.into(),
            extra: Some(extra.into()),
        }
    }
}

fn journal_item() -> MenuItem {
    MenuItem::new(JOURNAL, "Journal")
}

fn inventory_item() -> MenuItem {
    MenuItem::new(INVENTORY, "Inventory")
}

fn talents_item() -> MenuItem {
    MenuItem::new(TALENTS, "Talents")
}

pub fn main_menu() -> Menu {
    Menu::new(
        MAIN,
        "Anthea",
        vec![journal_item(), inventory_item(), talents_item()],
    )
}

fn journal_menu(journal: &Journal, menus: &Menus) -> Menu {
    let idx = menus.journal_index.unwrap_or(journal.entries.len() - 1);
    let e = journal.entries.get(idx).unwrap();
    let mut m = Menu::new(JOURNAL, "Journal", vec![MenuItem::new("", &e.text)]);
    m.navigation = Some((idx > 0, idx < journal.entries.len() - 1));
    m
}

fn inventory_menu(inventory: &Inventory) -> Menu {
    let mut msgs: Vec<MenuItem> = inventory
        .items
        .iter()
        .map(|i| MenuItem::new("", &i.description))
        .collect();
    if msgs.is_empty() {
        msgs.push(MenuItem::new("", "Empty hands!"));
    }
    Menu::new(INVENTORY, "Inventory", msgs)
}

fn talents_menu(talents: &Talents) -> Menu {
    Menu::new(
        TALENTS,
        "Talents",
        vec![
            MenuItem::new_table("Animals:", format!("{:>3}", talents.animals)),
            MenuItem::new_table("People:", format!("{:>3}", talents.people)),
            MenuItem::new_table("Weapons:", format!("{:>3}", talents.weapons)),
        ],
    )
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MenuEvent>()
            .add_event::<MenuItemEvent>()
            .add_event::<CloseMenuEvent>()
            .insert_resource(Menus::default())
            .add_system(menu_start.system())
            //.on_state_enter(STAGE, GameState::Menu,show_main_menu.system())
            .on_state_update(STAGE, GameState::Menu, click_system.system())
            .on_state_update(STAGE, GameState::Menu, click_nav_system.system())
            .on_state_update(STAGE, GameState::Menu, journal_event.system())
            .on_state_update(STAGE, GameState::Menu, inventory_event.system())
            .on_state_update(STAGE, GameState::Menu, talents_event.system())
            .on_state_update(STAGE, GameState::Menu, menu_close.system())
            .on_state_update(STAGE, GameState::Menu, close_menu.system());
    }
}

fn show_menu(mut queue: ResMut<Events<MessageEvent>>, menu: &Menu) {
    //clearm.send(ClearMessage);
    let mut msgs = vec![Message::new(&menu.title, MessageStyle::MenuTitle)];
    if let Some((backward, forward)) = menu.navigation {
        msgs.push(Message::new(
            "",
            MessageStyle::Navigation(backward, forward),
        ));
    }
    for mi in menu.items.iter() {
        if let Some(extra) = &mi.extra{
            msgs.push(Message::new(
                &mi.text,
                MessageStyle::Table(vec![extra.clone()]),
            ));
        } else {
            msgs.push(Message::new(
                &mi.text,
                MessageStyle::Interaction(mi.code.clone()),
            ));
        }
    }
    queue.send(MessageEvent::new_multi(msgs));
}

fn push_menu(queue: ResMut<Events<MessageEvent>>, mut menus: ResMut<Menus>, menu: Menu) {
    show_menu(queue, &menu);
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
    item_query: Query<(&Interaction, &Text, &InteractionItem), Mutated<Interaction>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    mut appstate: ResMut<State<GameState>>,
    mut menus: ResMut<Menus>,
    mut menuqueue: ResMut<Events<MenuItemEvent>>,
) {
    if let Some((interaction, _txt, item)) = item_query.iter().next() {
        if *interaction == Interaction::Clicked {
            let msg = &item.0;
            if CLOSE == msg {
                menus.pop();
                if let Some(m) = menus.menus.last() {
                    show_menu(queue, m);
                } else {
                    clearm.send(ClearMessage);
                    appstate.set_next(GameState::Running).unwrap();
                }
            } else if let Some(m) = menus.menus.last() {
                menuqueue.send(MenuItemEvent {
                    menu: m.code.clone(),
                    item: msg.into(),
                });
            }
        }
    }
}

fn click_nav_system(
    item_query: Query<(&Interaction, &TextureAtlasSprite, &NavigationPart), Mutated<Interaction>>,
    mut menus: ResMut<Menus>,
    queue: ResMut<Events<MessageEvent>>,
    journal: Res<Journal>,
) {
    if let Some((interaction, _txt, item)) = item_query.iter().next() {
        if *interaction == Interaction::Clicked {
            if menus.current() == JOURNAL {
                let idx = menus.journal_index.unwrap_or(journal.entries.len() - 1);
                match item {
                    NavigationPart::Back => menus.journal_index = Some(idx - 1),
                    NavigationPart::Forward => menus.journal_index = Some(idx + 1),
                }
                menus.pop();
                let m = journal_menu(&journal, &menus);
                push_menu(queue, menus, m);
            }
        }
    }
}

fn close_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut clearm: ResMut<Events<ClearMessage>>,
    queue: ResMut<Events<MessageEvent>>,
    mut appstate: ResMut<State<GameState>>,
    mut menus: ResMut<Menus>,
) {
    //for event in keyboard_input_events.iter() {
    if keyboard_input.just_released(KeyCode::Escape) {
        println!("Escape");
        menus.pop();
        if let Some(m) = menus.menus.last() {
            show_menu(queue, m);
        } else {
            clearm.send(ClearMessage);
            appstate.set_next(GameState::Running).unwrap();
        }
    }
    //}
}

#[derive(Debug, Clone)]
pub struct MenuEvent {
    pub menu: Menu,
}

impl MenuEvent {
    pub fn new(m: Menu) -> Self {
        MenuEvent { menu: m }
    }
}

#[derive(Debug, Clone)]
pub struct CloseMenuEvent;

fn menu_start(
    mut appstate: ResMut<State<GameState>>,
    mut event_reader: EventReader<MenuEvent>,
    queue: ResMut<Events<MessageEvent>>,
    mut menus: ResMut<Menus>,
) {
    if let Some(me) = event_reader.iter().next() {
        appstate.set_next(GameState::Menu).unwrap();
        menus.clear();
        push_menu(queue, menus, me.menu.clone());
    }
}

fn menu_close(
    mut appstate: ResMut<State<GameState>>,
    mut event_reader: EventReader<CloseMenuEvent>,
    mut menus: ResMut<Menus>,
) {
    if let Some(_me) = event_reader.iter().next() {
        appstate.set_next(GameState::Running).unwrap();
        menus.clear();
        //clearm.send(ClearMessage);
    }
}

#[derive(Debug, Clone)]
pub struct MenuItemEvent {
    pub menu: String,
    pub item: String,
}

fn journal_event(
    mut event_reader: EventReader<MenuItemEvent>,
    journal: Res<Journal>,
    menus: ResMut<Menus>,
    queue: ResMut<Events<MessageEvent>>,
) {
    if let Some(_e) = event_reader
        .iter()
        .filter(|e| e.menu == MAIN && e.item == JOURNAL)
        .next()
    {
        let m = journal_menu(&journal, &menus);
        push_menu(queue, menus, m);
    }
}

fn inventory_event(
    mut event_reader: EventReader<MenuItemEvent>,
    inventory: Res<Inventory>,
    menus: ResMut<Menus>,
    queue: ResMut<Events<MessageEvent>>,
) {
    if let Some(_e) = event_reader
        .iter()
        .filter(|e| e.menu == MAIN && e.item == INVENTORY)
        .next()
    {
        let m = inventory_menu(&inventory);
        push_menu(queue, menus, m);
    }
}

fn talents_event(
    mut event_reader: EventReader<MenuItemEvent>,
    talents: Res<Talents>,
    menus: ResMut<Menus>,
    queue: ResMut<Events<MessageEvent>>,
) {
    if let Some(_e) = event_reader
        .iter()
        .filter(|e| e.menu == MAIN && e.item == TALENTS)
        .next()
    {
        let m = talents_menu(&talents);
        push_menu(queue, menus, m);
    }
}
