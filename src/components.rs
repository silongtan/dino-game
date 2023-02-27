use bevy::prelude::Component;

// region: --- Common Components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_spawn: bool,
}
// endregion: --- Common Components

// region --- Player Components
#[derive(Component)]
pub struct Player;
//region --- Enemy Components