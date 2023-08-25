use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::constants::{MOTHERsub_MAX_WIDTH, MOTHERsub_MAX_HEIGHT, EMPTY_CHAR};


#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "bullet.png")]
    pub bullet: Handle<Image>,
    #[asset(path = "explosion.png")]
    pub explosion: Handle<Image>,
}

#[derive(Resource)]
pub struct Colors {
    pub normal_text: Color,
    pub sub_text: Color,
    pub menu_background: Color,
    pub node_background: Color,
    pub button_normal: Color,
    pub button_pressed: Color
}

#[derive(Resource)]
pub struct sub {
    pub pieces: [[char; MOTHERsub_MAX_HEIGHT]; MOTHERsub_MAX_WIDTH]
}

impl Default for sub {
    fn default() -> Self {
        sub{ pieces: [[EMPTY_CHAR; MOTHERsub_MAX_HEIGHT]; MOTHERsub_MAX_WIDTH] }
    }
}

#[derive(Resource)]
pub struct subbuildersub {
    pub root: Entity,
    pub pieces: [[Option<Entity>; MOTHERsub_MAX_HEIGHT]; MOTHERsub_MAX_WIDTH]
}

impl Default for subbuildersub {
    fn default() -> Self {
        subbuildersub { root: Entity::PLACEHOLDER, pieces: [[None; MOTHERsub_MAX_HEIGHT]; MOTHERsub_MAX_WIDTH] }
    }
}
