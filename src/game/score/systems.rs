use bevy::prelude::*;

use super::resources::{Highscores, Score};

use crate::events::GameOver;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score {}", score.value.to_string());
    }
}

pub fn handle_highscores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut highscores: ResMut<Highscores>,
) {
    for event in game_over_event_reader.read() {
        highscores.scores.push(("Hund".to_string(), event.score));
    }
}

pub fn highscores_updated(highscores: Res<Highscores>) {
    if highscores.is_changed() {
        println!("Highscores: {:?}", highscores);
    }
}
