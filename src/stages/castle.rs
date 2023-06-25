use crate::base::*;
use crate::menu::*;
use crate::ui::*;
use crate::world::*;
use bevy::prelude::*;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(castle_area())
            .add_system(affordance_mirror)
            .add_system(affordance_fountain)
            .add_system(action_fountain)
            .add_system(action_mirror)
            .add_system(character_peleus)
            .add_system(character_nerita)
            .add_system(action_nerita)
            .add_system(character_cretien)
            .add_system(character_scopas)
            .add_system(character_cherise)
            .add_system(character_rats)
            .add_system(action_rats)
            .add_system(character_theon)
            .add_system(consume_sword)
            .add_system(affordance_outside);
    }
}

const MIRROR: &str = "mirror";
const FOUNTAIN: &str = "fountain";
const SCISSORS: &str = "scissors";
const SCROLL: &str = "scroll";
const SWORD: &str = "sword";
const OUTSIDE: &str = "outside";
const CUT: &str = "cut";
const FIX: &str = "fix";
const FIGHT: &str = "fight";
const SCARE: &str = "scare";

const HAIR_CUT: &str = "hair_cut";
const HAIR_CUT_SELF: &str = "hair_cut_self";
const PELEUS_FORBIDDEN: &str = "peleus_forbidden";
const ALLOWED_TO_LEAVE: &str = "allowed_to_leave";
const OPENED_EXIT: &str = "opened_exit";
const TRAINED_BY_SCOPAS: &str = "trained_by_scopas";
const OBTAINED_FOOD: &str = "obtained_food";

const PELEUS: &str = "Peleus";
const NERITA: &str = "Nerita";
const CRETIEN: &str = "Cretien";
const SCOPAS: &str = "Scopas";
const CHERISE: &str = "Cherise";
const THEON: &str = "Theon";

const RATS: &str = "Rats";

const CAT: &str = "cat";

const QUEST_RATS: &str = "Rats";
const RATS_GONE: &str = "rats_gone";

fn castle_area() -> Area {
    let mut stage = Area::new("Selaion Palace", 0, SpritePosition::new(20, 4));
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

    let peleus = Character::new(
        PELEUS,
        "Peleus, your brother",
        "sprites/people/peleus.png",
        19,
        2,
    );
    let nerita = Character::new(
        NERITA,
        "Nerita, your maid",
        "sprites/people/nerita.png",
        6,
        4,
    );
    let cretien = Character::new(
        CRETIEN,
        "Cretien, your old teacher",
        "sprites/people/cretien.png",
        30,
        5,
    );
    let scopas = Character::new(
        SCOPAS,
        "Scopas, the weapons master",
        "sprites/people/scopas.png",
        22,
        19,
    );
    let cherise = Character::new(
        CHERISE,
        "Cherise, the cook",
        "sprites/people/cherise.png",
        12,
        21,
    );
    let theon = Character::new(
        THEON,
        "Theon, a palace guard",
        "sprites/people/theon.png",
        21,
        27,
    );

    let rats = Character::new(RATS, "Big rats", "sprites/people/rat.png", 2, 24);

    stage
        .add_character(peleus)
        .add_character(nerita)
        .add_character(cretien)
        .add_character(scopas)
        .add_character(cherise)
        .add_character(theon)
        .add_character(rats);

    stage
}

fn affordance_mirror(
    inventory: Res<Inventory>,
    flags: Res<QuestFlags>,
    mut event_reader: EventReader<AffordanceEvent>,
    mut queue: EventWriter<MessageEvent>,
    mut menu: EventWriter<MenuEvent>,
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
    mut queue: EventWriter<MessageEvent>,
    mut menu: EventWriter<MenuEvent>,
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
    mut queue: EventWriter<MessageEvent>,
    mut journal: EventWriter<JournalEvent>,
    mut flags: ResMut<QuestFlags>,
    mut close_menu: EventWriter<CloseMenuEvent>,
    mut body_change: EventWriter<BodyChangeEvent>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == FOUNTAIN && e.item == CUT)
    {
        inventory.remove_item(SCISSORS);
        talents.people += 1;
        close_menu.send(CloseMenuEvent);
        journal.send(JournalEvent::new(
            QUEST_MAIN,
            "I cut my hair short using the fountain as a mirror. Not sure I did a great job.",
        ));
        flags.set_flag(QUEST_MAIN, HAIR_CUT);
        flags.set_flag(QUEST_MAIN, HAIR_CUT_SELF);

        body_change.send(BodyChangeEvent::new(
            PlayerPart::Hair,
            "sprites/people/hair_short.png",
        ));

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
    mut queue: EventWriter<MessageEvent>,
    mut journal: EventWriter<JournalEvent>,
    mut flags: ResMut<QuestFlags>,
    mut close_menu: EventWriter<CloseMenuEvent>,
    mut body_change: EventWriter<BodyChangeEvent>,
) {
    if let Some(_e) = event_reader
        .iter()
        .find(|e| e.menu == MIRROR && e.item == CUT)
    {
        inventory.remove_item(SCISSORS);
        talents.people += 2;
        body_change.send(BodyChangeEvent::new(
            PlayerPart::Hair,
            "sprites/people/hair_short.png",
        ));
        close_menu.send(CloseMenuEvent);
        journal.send(JournalEvent::new(
            QUEST_MAIN,
            "I cut my hair short using the bedroom mirror.",
        ));
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
    mut queue: EventWriter<MessageEvent>,
    mut journal: EventWriter<JournalEvent>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == PELEUS) {
        if flags.has_flag(QUEST_MAIN, HAIR_CUT) {
            if flags.has_flag(QUEST_MAIN, ALLOWED_TO_LEAVE) {
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

                journal.send(JournalEvent::new(
                    QUEST_MAIN,
                    "Peleus has allowed me to leave on my quest for Father!",
                ));
            }
        } else if flags.has_flag(QUEST_MAIN, PELEUS_FORBIDDEN) {
            queue.send(MessageEvent::new(
                "Once again, I am NOT going to let a girl go chasing a ghost.\nYour duty is to stay here and marry to strenghten my kingdom.\nDon't insist!",
                MessageStyle::Info,
            ));
        } else {
            flags.set_flag(QUEST_MAIN, PELEUS_FORBIDDEN);
            journal.send(JournalEvent::new(
                QUEST_MAIN,
                "Peleus forbids me to leave. He'll see!",
            ));
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
    mut queue: EventWriter<MessageEvent>,
    mut menu: EventWriter<MenuEvent>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == NERITA) {
        if inventory.contains_item(SCISSORS) {
            let mi = MenuItem::new(
                CUT,
                "You really want me to cut your hair with these scissors?",
            );
            let m = Menu::new(NERITA, "Nerita, your maid", vec![mi]);
            menu.send(MenuEvent::new(m));
        } else if flags.has_flag(QUEST_MAIN, HAIR_CUT) {
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
    mut queue: EventWriter<MessageEvent>,
    mut journal: EventWriter<JournalEvent>,
    mut flags: ResMut<QuestFlags>,
    mut close_menu: EventWriter<CloseMenuEvent>,
    mut body_change: EventWriter<BodyChangeEvent>,
) {
    if let Some(e) = event_reader.iter().find(|e| e.menu == NERITA) {
        if e.item == CUT {
            inventory.remove_item(SCISSORS);
            talents.people += 2;
            body_change.send(BodyChangeEvent::new(
                PlayerPart::Hair,
                "sprites/people/hair_short.png",
            ));
            close_menu.send(CloseMenuEvent);
            journal.send(JournalEvent::new(
                QUEST_MAIN,
                "Nerita cut my hair so I don't look too much like a girl now. I think it suits me.",
            ));
            flags.set_flag(QUEST_MAIN, HAIR_CUT);
            queue.send(MessageEvent::new(
                "Really a shame to cut such beautiful hair (People +2)!",
                MessageStyle::Info,
            ));
        } else if e.item == FIX {
            talents.people += 1;
            close_menu.send(CloseMenuEvent);
            journal.send(JournalEvent::new(
                QUEST_MAIN,
                "Nerita fixed my hair so it doesn't look as bad as it used to.",
            ));
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
    mut queue: EventWriter<MessageEvent>,
    mut inventory: ResMut<Inventory>,
    mut spells: ResMut<Spells>,
    mut journal: EventWriter<JournalEvent>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == CRETIEN) {
        if inventory.contains_item(SCROLL) {
            queue.send(MessageEvent::new(
                "Ooohh, this scroll is a magic spell! Let me see if I can teach you the incantation (Spell gained)...",
                MessageStyle::Info,
            ));
            inventory.remove_item(SCROLL);
            let spell = Spell::new(CAT, "Create the illusion of a cat!");
            spells.add_spell(spell);
            journal.send(JournalEvent::new(
                QUEST_MAIN,
                "Cretien taught me a little spell, not sure if it'll be useful...",
            ));
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
    mut queue: EventWriter<MessageEvent>,
    mut talents: ResMut<Talents>,
    mut flags: ResMut<QuestFlags>,
    mut journal: EventWriter<JournalEvent>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == SCOPAS) {
        if talents.weapons > 0 {
            if flags.has_flag(QUEST_MAIN, TRAINED_BY_SCOPAS) {
                queue.send(MessageEvent::new(
                    "Don't tire yourself out!",
                    MessageStyle::Info,
                ));
            } else {
                flags.set_flag(QUEST_MAIN, TRAINED_BY_SCOPAS);
                journal.send(JournalEvent::new(
                    QUEST_MAIN,
                    "Scopas gave me a hard fighting lesson.",
                ));

                queue.send(MessageEvent::new(
                    "You're getting better with a weapon, but you still need to practise (Weapons +1)!",
                    MessageStyle::Info,
                ));
                talents.weapons += 1;
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
    mut body_change: EventWriter<BodyChangeEvent>,
    mut queue: EventWriter<MessageEvent>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == SWORD) {
        body_change.send(BodyChangeEvent::new(
            PlayerPart::RightHand,
            "sprites/people/short_sword.png",
        ));
        talents.weapons += 1;
        queue.send(MessageEvent::new(
            "You now have a weapon (Weapons +1)!",
            MessageStyle::Info,
        ));
    }
}

fn character_cherise(
    mut event_reader: EventReader<CharacterEvent>,
    mut queue: EventWriter<MessageEvent>,
    mut flags: ResMut<QuestFlags>,
    mut journal: ResMut<Journal>,
    mut journale: EventWriter<JournalEvent>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == CHERISE) {
        if flags.has_flag(QUEST_RATS, QUEST_STARTED) {
            if flags.has_flag(QUEST_RATS, QUEST_COMPLETED) {
                queue.send(MessageEvent::new(
                    "Thanks again for killing these rats!",
                    MessageStyle::Info,
                ));
            } else if flags.has_flag(QUEST_RATS, RATS_GONE) {
                flags.set_flag(QUEST_RATS, QUEST_COMPLETED);
                journale.send(JournalEvent::new(QUEST_MAIN,"Cherise gave me some food to thank me for getting rid of the rats in the cellar"));
                flags.set_flag(QUEST_MAIN, OBTAINED_FOOD);
                queue.send(MessageEvent::new(
                    "You got rid of the rats? Great! Here's some food for you...",
                    MessageStyle::Info,
                ));
            } else {
                queue.send(MessageEvent::new(
                    "These rats are driving me crazy!",
                    MessageStyle::Info,
                ));
            }
        } else {
            flags.set_flag(QUEST_RATS, QUEST_STARTED);
            let q = Quest::new(QUEST_RATS, "Get rid of the rats in the cellar");
            journal.add_quest(q);
            journale.send(JournalEvent::new(
                QUEST_RATS,
                "Cherise would like somebody to kill the rats in the cellar.",
            ));
            queue.send(MessageEvent::new(
                "Don't tell your brother, but there are rats in the cellar. I can't get rid of them, I wish somebody would kill them all!",
                MessageStyle::Info,
            ));
        }
    }
}

fn character_rats(
    mut event_reader: EventReader<CharacterEvent>,
    mut queue: EventWriter<MessageEvent>,
    mut menu: EventWriter<MenuEvent>,
    talents: Res<Talents>,
    spells: Res<Spells>,
    flags: Res<QuestFlags>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == RATS) {
        if flags.has_flag(QUEST_RATS, QUEST_STARTED) {
            let mut mis = vec![];
            if talents.weapons > 1 {
                mis.push(MenuItem::new(FIGHT, "Kill the rats!"));
            }
            if spells.contains_spell(CAT) {
                mis.push(MenuItem::new(SCARE, "Create the illusion of a cat"));
            }
            if mis.is_empty() {
                queue.send(MessageEvent::new(
                    "The rats are not afraid of you.",
                    MessageStyle::Info,
                ));
            } else {
                if mis.len() < 2 {
                    mis.push(MenuItem::new("", "(More options could be available)"));
                }

                let m = Menu::new(RATS, "Big cellar rats", mis);
                menu.send(MenuEvent::new(m));
            }
        } else {
            queue.send(MessageEvent::new(
                "The rats are not afraid of you.",
                MessageStyle::Info,
            ));
        }
    }
}

fn action_rats(
    mut commands: Commands,
    mut event_reader: EventReader<MenuItemEvent>,
    mut queue: EventWriter<MessageEvent>,
    mut talents: ResMut<Talents>,
    mut flags: ResMut<QuestFlags>,
    mut area: ResMut<Area>,
    mut close_menu: EventWriter<CloseMenuEvent>,
    character_query: Query<(Entity, &Character)>,
) {
    if let Some(e) = event_reader.iter().find(|e| e.menu == RATS) {
        let mut gone = false;
        if e.item == FIGHT {
            queue.send(MessageEvent::new(
                "You massacre the rats.",
                MessageStyle::Info,
            ));
            gone = true;
        } else if e.item == SCARE {
            talents.animals += 1;
            queue.send(MessageEvent::new(
                "You pronounce the incantation, a big cat appears, scaring the rats away (Animals+1).",
                MessageStyle::Info,
            ));
            gone = true;
        }
        if gone {
            flags.set_flag(QUEST_RATS, RATS_GONE);

            close_menu.send(CloseMenuEvent);
            for (e, _i2) in character_query.iter().filter(|(_e, c)| c.name == RATS) {
                commands.entity(e).despawn_recursive();
            }
            area.characters.retain(|_, v| v.name == RATS);
        }
    }
}

fn character_theon(
    mut event_reader: EventReader<CharacterEvent>,
    mut queue: EventWriter<MessageEvent>,
    mut flags: ResMut<QuestFlags>,
    mut journal: EventWriter<JournalEvent>,
    mut remove_tile: EventWriter<RemoveTileEvent>,
    mut area: ResMut<Area>,
) {
    for _e in event_reader.iter().filter(|e| e.0 == THEON) {
        if flags.has_flag(QUEST_MAIN, ALLOWED_TO_LEAVE) {
            if flags.has_flag(QUEST_MAIN, OPENED_EXIT) {
                queue.send(MessageEvent::new("Good day, my lady.", MessageStyle::Info));
            } else {
                flags.set_flag(QUEST_MAIN, OPENED_EXIT);
                journal.send(JournalEvent::new(
                    QUEST_MAIN,
                    "I can now go out of the palace",
                ));
                queue.send(MessageEvent::new(
                    "Peleus told us we could let you go. Careful out there, my lady.",
                    MessageStyle::Info,
                ));

                let gate_pos = vec![
                    SpritePosition::new(-20, 29),
                    SpritePosition::new(-21, 29),
                    SpritePosition::new(-22, 29),
                ];
                for pos in gate_pos.into_iter() {
                    remove_tile.send(RemoveTileEvent::new(pos, 1));
                }
                for x in 20..=22 {
                    let outside1 =
                        Affordance::new(format!("{}_{}", OUTSIDE, x), "The outside world", x, 29);
                    area.add_affordance(outside1);
                }
            }
        } else {
            queue.send(MessageEvent::new(
                " You are forbidden to go outside. I'm sorry my lady, your brother's orders.",
                MessageStyle::Info,
            ));
        }
    }
}

fn affordance_outside(
    talents: Res<Talents>,
    flags: Res<QuestFlags>,
    mut event_reader: EventReader<AffordanceEvent>,
    mut queue: EventWriter<MessageEvent>,
    mut state: ResMut<NextState<GameState>>,
) {
    for _e in event_reader.iter().filter(|e| e.0.starts_with(OUTSIDE)) {
        if !flags.has_flag(QUEST_MAIN, OBTAINED_FOOD) {
            queue.send(MessageEvent::new(
                "You should get food before venturing outside",
                MessageStyle::Info,
            ));
        } else if talents.weapons < 1 {
            queue.send(MessageEvent::new(
                "You should get a weapon, the outside world is not safe",
                MessageStyle::Info,
            ));
        } else {
            queue.send(MessageEvent::new_multi(vec![
                Message {
                    contents: "Success!".to_owned(),
                    style: MessageStyle::Title,
                },
                Message {
                    contents: "You pass the castle gate. Your adventure truly begins!".to_owned(),
                    style: MessageStyle::Info,
                },
            ]));
            state.set(GameState::End);
        }
    }
}
