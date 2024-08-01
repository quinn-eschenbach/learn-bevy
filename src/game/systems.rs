use bevy::prelude::*;

use super::SimulationState;

pub fn pause_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(match simulation_state.get() {
            SimulationState::Paused => {
                println!("Simulation running");
                SimulationState::Running
            }
            SimulationState::Running => {
                println!("Simaulation paused");
                SimulationState::Paused
            }
        });
    }
}
