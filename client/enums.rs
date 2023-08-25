use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    subBuilding,
    MainMenu,
    MatchMaking,
    InGame,
}
