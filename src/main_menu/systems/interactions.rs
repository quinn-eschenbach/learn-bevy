use bevy::{app::AppExit, prelude::*};

use crate::{
    main_menu::{
        components::*,
        styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    },
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        *background_color = match *interaction {
            Interaction::Pressed => {
                app_state_next_state.set(AppState::Game);
                PRESSED_BUTTON_COLOR.into()
            }
            Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
            Interaction::None => NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        *background_color = match *interaction {
            Interaction::Pressed => {
                app_exit_event_writer.send(AppExit);
                PRESSED_BUTTON_COLOR.into()
            }
            Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
            Interaction::None => NORMAL_BUTTON_COLOR.into(),
        }
    }
}
