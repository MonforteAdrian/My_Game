use bevy::{
    dev_tools::states::log_transitions,
    prelude::{App, AppExtStates, Plugin, StateSet, States, SubStates, Update},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.enable_state_scoped_entities::<GameState>()
            .init_state::<GameState>()
            .init_state::<MenuState>()
            .init_state::<WorldCreationState>()
            .add_sub_state::<IsPaused>()
            .add_systems(Update, log_transitions::<GameState>)
            .add_systems(Update, log_transitions::<IsPaused>)
            .add_systems(Update, log_transitions::<MenuState>)
            .add_systems(Update, log_transitions::<WorldCreationState>);
    }
}

// TODO loading should happen during splash merging them on Startin
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    InMenu,
    InMapCreation,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::InGame)]
pub enum IsPaused {
    #[default]
    Paused,
    Running,
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum WorldCreationState {
    MapSettings,
    MapGeneration,
    #[default]
    Disabled,
}
