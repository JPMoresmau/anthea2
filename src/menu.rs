use bevy::prelude::*;

use crate::base::*;
use crate::ui::*;

pub struct MenuPlugin;


impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .on_state_enter(STAGE, GameState::Menu,show_main_menu.system())
            .on_state_update(STAGE, GameState::Menu,click_system.system())
        ;
    }
}

fn show_main_menu(
    mut queue: ResMut<Events<MessageEvent>>,
){
    queue.send(MessageEvent::new_multi(vec![
        Message::new("Anthea",MessageStyle::MenuTitle),
        Message::new("Journal",MessageStyle::Interaction),
        Message::new("Inventory",MessageStyle::Interaction),
        Message::new("Talents",MessageStyle::Interaction),
    ]));
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
    item_query: Query<(&Interaction,&Text),(Mutated<Interaction>,With<MenuItem>)>,
    mut clearm: ResMut<Events<ClearMessage>>,
    mut appstate: ResMut<State<GameState>>,
){
    for (interaction, txt) in item_query.iter() {
        if *interaction==Interaction::Clicked {
            if txt.sections.len()>0 && txt.sections[0].value.len()>0{
                let msg = &txt.sections[0].value;
                if CLOSE==msg {
                    clearm.send(ClearMessage); 
                    appstate.set_next(GameState::Running).unwrap();
                }
            }
        }
    }
}