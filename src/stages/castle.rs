use crate::base::*;
use crate::menu::*;
use crate::ui::*;
use crate::world::*;
use bevy::prelude::*;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(castle_area())
            .add_system(affordance_mirror.system())
            .add_system(affordance_fountain.system())
            .add_system(action_fountain.system())
            .add_system(action_mirror.system())
            .add_system(character_peleus.system())
            .add_system(character_nerita.system())
            .add_system(action_nerita.system())
            .add_system(character_cretien.system())
            .add_system(character_scopas.system())
            .add_system(consume_sword.system())
        ;
    }
}

const MIRROR: &str = "mirror";
const FOUNTAIN: &str = "fountain";
const SCISSORS: &str = "scissors";
const SCROLL: &str = "scroll";
const SWORD: &str = "sword";

const CUT: &str = "cut";
const FIX: &str = "fix";

const HAIR_CUT: &str = "hair_cut";
const HAIR_CUT_SELF: &str = "hair_cut_self";
const PELEUS_FORBIDDEN: &str = "peleus_forbidden";
const ALLOWED_TO_LEAVE: &str = "allowed_to_leave";
const TRAINED_BY_SCOPAS: &str = "trained_by_scopas";

const PELEUS: &str = "Peleus";
const NERITA: &str = "Nerita";
const CRETIEN: &str = "Cretien";
const SCOPAS: &str = "Scopas";

const CAT: &str = "cat";

fn castle_area() -> Area {
    let mut stage = Area::new("Selaion Palace", 0, sprite_position(-20, 4));
    let bedroom = Room::new("bedroom", "Your bedroom", 6, 3, 9, 6);
    let throne = Room::new("throne", "Selaion throne room", 11, 2, 26, 6);
    let garden =
        Room::new("garden", "The royal garden", 7, 8, 15, 12).add_dimensions(16, 9, 16, 11);
    let study = Room::new("study", "The study", 28, 2, 32, 5);
    let courtyard = Room::new("courtyard", "The courtyard", 17, 8, 25, 28)
        .add_dimensions(26, 8, 26, 26)
        .add_dimensions(27, 19, 35, 26);
    let kitchen = Room::new("kitchen", "The kitchen", 9, 19, 15, 24);
    let cellar = Room::new("cellar", "The cellar", 2, 20, 4, 24);
    let corridor = Room::new("corridor", "A dark corridor", 5, 22, 8, 22);
    let armory = Room::new("armory", "The armory", 31, 15, 35, 17);
    let gates = Room::new("gates", "The palace gates", 20, 29, 22, 29);

    stage
        .add_room(bedroom)
        .add_room(throne)
        .add_room(garden)
        .add_room(study)
        .add_room(courtyard)
        .add_room(kitchen)
        .add_room(cellar)
        .add_room(corridor)
        .add_room(armory)
        .add_room(gates);

    let mirror = Affordance::new(MIRROR, "Your bedside mirror", 9, 3);
    let fountain = Affordance::new(FOUNTAIN, "The garden fountain", 11, 10);
    stage.add_affordance(mirror).add_affordance(fountain);

    let scissors = Item::new(
        SCISSORS,
        "Sharpish scissors",
        "sprites/items/double_sword.png",
        14,
        12,
    );
    stage.add_item(scissors);
    let scroll = Item::new(
        SCROLL,
        "Undecipherable scroll",
        "sprites/items/scroll-brown.png",
        4,
        20,
    );
    stage.add_item(scroll);
    let sword = Item::new_consumable(
        SWORD,
        "Small sword",
        "sprites/items/long_sword1.png",
        34,
        15,
    );
    stage.add_item(sword);

    let peleus = Character::new(PELEUS,"Peleus, your brother", "sprites/people/peleus.png",19,2);
    let nerita = Character::new(NERITA,"Nerita, your maid", "sprites/people/nerita.png",6,4);
    let cretien = Character::new(CRETIEN,"Cretien, your old teacher", "sprites/people/cretien.png",30,5);
    let scopas = Character::new(SCOPAS,"Scopas, the weapons master", "sprites/people/scopas.png",22,19);

    stage.add_character(peleus).add_character(nerita).add_character(cretien).add_character(scopas);

    stage
}

fn affordance_mirror(
    inventory: Res<Inventory>,
    flags: Res<QuestFlags>,
    mut event_reader: EventReader<AffordanceEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut menu: ResMut<Events<MenuEvent>>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == MIRROR) {
        if inventory.contains_item(SCISSORS) {
            let mi = MenuItem::new(CUT, "Cut your hair with the scissors?");
            let m = Menu::new(MIRROR, "Mirror", vec![mi]);
            menu.send(MenuEvent::new(m));
        } else if flags.has_flag(QUEST_MAIN, HAIR_CUT) {
            queue.send(MessageEvent::new(
                "Your look at yourself and your short hair...",
                MessageStyle::Info,
            ));
        } else {
            queue.send(MessageEvent::new(
                "You look at yourself in the mirror",
                MessageStyle::Info,
            ));
        }
    }
}

fn affordance_fountain(
    inventory: Res<Inventory>,
    flags: Res<QuestFlags>,
    mut event_reader: EventReader<AffordanceEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut menu: ResMut<Events<MenuEvent>>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == FOUNTAIN) {
        if inventory.contains_item(SCISSORS) {
            let mi = MenuItem::new(
                CUT,
                "Cut your hair with the scissors, using the fountain as a mirror?",
            );
            let m = Menu::new(FOUNTAIN, "Fountain", vec![mi]);
            menu.send(MenuEvent::new(m));
        } else if flags.has_flag(QUEST_MAIN, HAIR_CUT) {
            queue.send(MessageEvent::new(
                "Your reflection in the water looks like a grinning boy...",
                MessageStyle::Info,
            ));
        } else {
            queue.send(MessageEvent::new(
                "The water is refreshing.",
                MessageStyle::Info,
            ));
        }
    }
}

fn action_fountain(
    mut event_reader: EventReader<MenuItemEvent>,
    mut inventory: ResMut<Inventory>,
    mut talents: ResMut<Talents>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut journal: ResMut<Journal>,
    mut flags: ResMut<QuestFlags>,
    mut close_menu: ResMut<Events<CloseMenuEvent>>,
    mut body_change: ResMut<Events<BodyChangeEvent>>,
) {
    if let Some(_e) = event_reader
        .iter()
        .filter(|e| e.menu == FOUNTAIN && e.item == CUT)
        .next()
    {
        inventory.remove_item(SCISSORS);
        talents.people += 1;
        close_menu.send(CloseMenuEvent);
        journal.add_entry(
            QUEST_MAIN,
            "I cut my hair short using the fountain as a mirror. Not sure I did a great job.",
        );
        flags.set_flag(QUEST_MAIN, HAIR_CUT);
        flags.set_flag(QUEST_MAIN, HAIR_CUT_SELF);

        body_change.send(BodyChangeEvent::new(PlayerPart::Hair,"sprites/people/hair_short.png"));
        
        queue.send(MessageEvent::new(
            "You feel you've made a mess, but you cut your hair short (People +1).",
            MessageStyle::Info,
        ));
    }
}

fn action_mirror(
    mut event_reader: EventReader<MenuItemEvent>,
    mut inventory: ResMut<Inventory>,
    mut talents: ResMut<Talents>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut journal: ResMut<Journal>,
    mut flags: ResMut<QuestFlags>,
    mut close_menu: ResMut<Events<CloseMenuEvent>>,
    mut body_change: ResMut<Events<BodyChangeEvent>>,
) {
    if let Some(_e) = event_reader
        .iter()
        .filter(|e| e.menu == MIRROR && e.item == CUT)
        .next()
    {
        inventory.remove_item(SCISSORS);
        talents.people += 2;
        body_change.send(BodyChangeEvent::new(PlayerPart::Hair,"sprites/people/hair_short.png"));
        close_menu.send(CloseMenuEvent);
        journal.add_entry(
            QUEST_MAIN,
            "I cut my hair short using the bedroom mirror.",
        );
        flags.set_flag(QUEST_MAIN, HAIR_CUT);
        queue.send(MessageEvent::new(
            "You carefully cut your hair short (People +2).",
            MessageStyle::Info,
        ));
    }
}

fn character_peleus(
    mut flags: ResMut<QuestFlags>,
    mut event_reader: EventReader<CharacterEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut journal: ResMut<Journal>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == PELEUS) {
        if flags.has_flag(QUEST_MAIN, HAIR_CUT){
            if flags.has_flag(QUEST_MAIN, ALLOWED_TO_LEAVE){
                queue.send(MessageEvent::new(
                    "You haven't left yet?",
                    MessageStyle::Info,
                ));
            } else {
                flags.set_flag(QUEST_MAIN, ALLOWED_TO_LEAVE);
                queue.send(MessageEvent::new(
                    "I see you're determined enough get rid of the hair you were so proud of.\nAllright, I will give orders that you're allowed to leave.",
                    MessageStyle::Info,
                ));
                
                journal.add_entry(QUEST_MAIN,"Peleus has allowed me to leave on my quest for Father!");
            }
        } else if flags.has_flag(QUEST_MAIN, PELEUS_FORBIDDEN) {
            queue.send(MessageEvent::new(
                "Once again, I am NOT going to let a girl go chasing a ghost.\nYour duty is to stay here and marry to strenghten my kingdom.\nDon't insist!",
                MessageStyle::Info,
            ));
        } else {
            flags.set_flag(QUEST_MAIN, PELEUS_FORBIDDEN);
            journal.add_entry(
                QUEST_MAIN,
                "Peleus forbids me to leave. He'll see!",
            );
            queue.send(MessageEvent::new(
                "I am NOT going to let a girl go chasing a ghost.\nYour duty is to stay here and marry to strenghten my kingdom.",
                MessageStyle::Info,
            ));
        }
    }
}


fn character_nerita(
    flags: Res<QuestFlags>,
    inventory: Res<Inventory>,
    mut event_reader: EventReader<CharacterEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut menu: ResMut<Events<MenuEvent>>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == NERITA) {
        if inventory.contains_item(SCISSORS) {
            let mi = MenuItem::new(
                CUT,
                "You really want me to cut your hair with these scissors?",
            );
            let m = Menu::new(NERITA, "Nerita, your maid", vec![mi]);
            menu.send(MenuEvent::new(m));
        } else if flags.has_flag(QUEST_MAIN, HAIR_CUT){
            if flags.has_flag(QUEST_MAIN, HAIR_CUT_SELF) {
                let mi = MenuItem::new(
                    FIX,
                    "What have you done to your hair? Shall I fix it for you?",
                );
                let m = Menu::new(NERITA, "Nerita, your maid", vec![mi]);
                menu.send(MenuEvent::new(m));
            } else {
                queue.send(MessageEvent::new(
                    "You look like a boy now! A pretty boy!",
                    MessageStyle::Info,
                ));
            }
        } else {

            queue.send(MessageEvent::new(
                "You'll always be a little girl to me. Let me comb your hair!",
                MessageStyle::Info,
            ));
        }
    }
}

fn action_nerita(
    mut event_reader: EventReader<MenuItemEvent>,
    mut inventory: ResMut<Inventory>,
    mut talents: ResMut<Talents>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut journal: ResMut<Journal>,
    mut flags: ResMut<QuestFlags>,
    mut close_menu: ResMut<Events<CloseMenuEvent>>,
    mut body_change: ResMut<Events<BodyChangeEvent>>,
) {
    if let Some(e) = event_reader
        .iter()
        .filter(|e| e.menu == NERITA)
        .next()
    {
        if e.item==CUT {
            inventory.remove_item(SCISSORS);
            talents.people += 2;
            body_change.send(BodyChangeEvent::new(PlayerPart::Hair,"sprites/people/hair_short.png"));
            close_menu.send(CloseMenuEvent);
            journal.add_entry(
                QUEST_MAIN,
                "Nerita cut my hair so I don't look too much like a girl now. I think it suits me.",
            );
            flags.set_flag(QUEST_MAIN, HAIR_CUT);
            queue.send(MessageEvent::new(
                "Really a shame to cut such beautiful hair (People +2)!",
                MessageStyle::Info,
            ));
        } else if e.item==FIX {
            talents.people += 1;
            close_menu.send(CloseMenuEvent);
            journal.add_entry(
                QUEST_MAIN,
                "Nerita fixed my hair so it doesn't look as bad as it used to.",
            );
            flags.unset_flag(QUEST_MAIN, HAIR_CUT_SELF);
            queue.send(MessageEvent::new(
                "Now, you look a bit better now (People +1)!",
                MessageStyle::Info,
            ));
        }
    }
}

fn character_cretien(
    mut event_reader: EventReader<CharacterEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut inventory: ResMut<Inventory>,
    mut spells: ResMut<Spells>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == CRETIEN) {
        if inventory.contains_item(SCROLL) {
            queue.send(MessageEvent::new(
                "Ooohh, this scroll is a magic spell! Let me see if I can teach you the incantation (Spell gained)...",
                MessageStyle::Info,
            ));
            inventory.remove_item(SCROLL);
            let spell=Spell::new(CAT,"Create the illusion of a cat!");
            spells.add_spell(spell);
        } else {
            queue.send(MessageEvent::new(
                "I'm always on the lookout for new knowledge!",
                MessageStyle::Info,
            ));
        }
    }
}

fn character_scopas(
    mut event_reader: EventReader<CharacterEvent>,
    mut queue: ResMut<Events<MessageEvent>>,
    mut talents: ResMut<Talents>,
    mut flags: ResMut<QuestFlags>,
    mut journal: ResMut<Journal>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == SCOPAS) {
        if talents.weapons>0 {
            if flags.has_flag(QUEST_MAIN, TRAINED_BY_SCOPAS){
                queue.send(MessageEvent::new(
                    "Don't tire yourself out!",
                    MessageStyle::Info,
                ));
            } else {
                flags.set_flag(QUEST_MAIN, TRAINED_BY_SCOPAS);
                journal.add_entry(QUEST_MAIN, "Scopas gave me a hard fighting lesson.");
                
                queue.send(MessageEvent::new(
                    "You're getting better with a weapon, but you still need to practise (Weapons +1)!",
                    MessageStyle::Info,
                ));
                talents.weapons+=1;
            }
        } else {
            queue.send(MessageEvent::new(
                "Get a weapon and come back to me to train.",
                MessageStyle::Info,
            ));
        }
    }
}


fn consume_sword(
    mut event_reader: EventReader<ItemEvent>,
    mut talents: ResMut<Talents>,
    mut body_change: ResMut<Events<BodyChangeEvent>>,
    mut queue: ResMut<Events<MessageEvent>>,
){
    for _e in event_reader.iter().filter(|e| e.0 == SWORD) {
        body_change.send(BodyChangeEvent::new(PlayerPart::RightHand,"sprites/people/short_sword.png"));
        talents.weapons+=1;
        queue.send(MessageEvent::new("You now have a weapon (Weapons +1)!", MessageStyle::Info));
    }
}