use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Highscores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), remove_score)
            .add_systems(
                Update,
                (
                    update_score.run_if(in_state(AppState::Game)),
                    handle_highscores,
                    highscores_updated,
                ),
            );
    }
}
