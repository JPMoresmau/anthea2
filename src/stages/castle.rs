use crate::world::*;
use crate::base::*;
use crate::menu::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(castle_area())
            .add_system(affordance_mirror.system())
            .add_system(affordance_fountain.system())
            .add_system(action_fountain.system())
            //.add_system(message_clear_system.system())
        ;
    }
}

const MIRROR: &str= "mirror";
const FOUNTAIN: &str= "fountain";
const SCISSORS: &str= "scissors";
const CUT: &str= "cut";

fn castle_area() -> Area {
    let mut stage = Area::new("Selaion Palace",0, sprite_position(-7,4));
    let bedroom = Room::new("bedroom", "Your bedroom",6,3,9,6);
    let throne = Room::new("throne", "Selaion throne room", 11, 2, 26, 6);
    let garden = Room::new("garden", "The royal garden", 7, 8, 15, 12)
        .add_dimensions(16, 9, 16,11);
    let study=Room::new("study", "The study", 28, 2, 32, 5);
    let courtyard=Room::new("courtyard","The courtyard", 17,8,25,28)
        .add_dimensions(26,8,26,26)
        .add_dimensions(27,19, 35,26);
    let kitchen = Room::new("kitchen","The kitchen",9,19,15,24);
    let cellar= Room::new("cellar", "The cellar", 2, 20, 4, 24);
    let corridor= Room::new("corridor", "A dark corridor", 5, 22, 8, 22);
    let armory=Room::new("armory", "The armory", 31, 15, 35, 17);
    let gates=Room::new("gates","The palace gates", 20,29,22,29);

    stage.add_room(bedroom)
        .add_room(throne)
        .add_room(garden)
        .add_room(study)
        .add_room(courtyard)
        .add_room(kitchen)
        .add_room(cellar)
        .add_room(corridor)
        .add_room(armory)
        .add_room(gates);

    let mirror=Affordance::new(MIRROR,"Your bedside mirror",  9, 3);
    let fountain=Affordance::new(FOUNTAIN, "The garden fountain",  11, 10);
    stage.add_affordance(mirror).add_affordance(fountain);

    let scissors=Item::new(SCISSORS, "Sharpish scissors","sprites/items/double_sword.png",14,12);
    stage.add_item(scissors);
    stage
}

fn affordance_mirror(
    commands: &mut Commands,
    player_query: Query<(Entity, &Player)>,
    mut event_reader: EventReader<AffordanceEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
){
    for _e in event_reader.iter().filter(|e| e.0==MIRROR) {
        queue.send(MessageEvent::new("You look at yourself in the mirror", MessageStyle::Info));
    }
}

fn affordance_fountain(
    inventory: Res<Inventory>,
    mut event_reader: EventReader<AffordanceEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut menu: ResMut<Events<MenuEvent>>,
){
    for _e in event_reader.iter().filter(|e| e.0==FOUNTAIN) {
        if inventory.contains_item(SCISSORS){
            let mi=MenuItem::new(CUT,"Cut your hair with the scissors, using the fountain as a mirror");
            let m=Menu::new(FOUNTAIN, "Fountain", vec![mi]);
            menu.send(MenuEvent::new(m));
        } else {
            queue.send(MessageEvent::new("The water is refreshing.", MessageStyle::Info));
        }
    }
}

fn action_fountain(
    mut event_reader: EventReader<MenuItemEvent>,
    mut inventory: ResMut<Inventory>,
){
    if let Some(_e) = event_reader.iter().filter(|e| e.menu==FOUNTAIN && e.item==CUT).next() {
        inventory.remove_item(SCISSORS);
    }
}