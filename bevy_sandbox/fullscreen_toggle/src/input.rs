use bevy::prelude::*;
use crate::ui::setup_menu;
use crate::components::{MenuVisible, OptionsMenu};

pub fn toggle_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_visible: ResMut<MenuVisible>,
    mut commands: Commands,
    menu_query: Query<Entity, With<OptionsMenu>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        menu_visible.0 = !menu_visible.0;
        if menu_visible.0 {
            setup_menu(commands);
        } else {
            for entity in menu_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn toggle_fullscreen(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    use bevy::window::{MonitorSelection, WindowMode};
    if keyboard_input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = windows.single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                _ => WindowMode::Windowed,
            };
        } else {
            warn!("Could not find primary window to toggle fullscreen.");
        }
    }
}

pub fn handle_exit(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        EventWriter::write(&mut app_exit_events, AppExit::Success);
    }
}
