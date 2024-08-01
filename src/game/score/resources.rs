use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct Highscores {
    pub scores: Vec<(String, u32)>,
}

impl Default for Highscores {
    fn default() -> Self {
        Highscores { scores: Vec::new() }
    }
}
