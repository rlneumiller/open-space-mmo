mod components;
mod ui;
mod input;
mod gameplay;
mod resources;

use bevy::prelude::*;
use bevy::window::{WindowPlugin, Window, WindowResolution, WindowMode};
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fullscreen Toggle".to_string(),
                resolution: WindowResolution::new(800.0, 600.0),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(MenuVisible(false))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            input::toggle_fullscreen,
            input::handle_exit,
            gameplay::rotate_cube,
            input::toggle_menu,
            ui::handle_menu_interaction,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a new Entity and attach a Camera3d Component
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn a new Entity and attach a Mesh3d Component
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        SpinningCube,
    ));
    
    // Spawn a new Entity and attach a DirectionalLight Component
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 3000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
    ));
}
