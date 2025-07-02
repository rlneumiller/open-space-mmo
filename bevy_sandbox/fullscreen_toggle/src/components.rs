use bevy::prelude::*;

#[derive(Component)]
pub struct SpinningCube;

#[derive(Component)]
pub struct OptionsMenu;

#[derive(Component)]
pub struct WindowModeButton;

#[derive(Resource)]
pub struct MenuVisible(pub bool);
