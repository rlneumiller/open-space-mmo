use bevy::prelude::*;
use bevy_winit::WinitWindows;

#[derive(Resource)]
struct DesktopInfo {
    monitors: Vec<MonitorInfo>,
}

#[derive(Debug, Clone)]
struct MonitorInfo {
    name: Option<String>,
    size: (u32, u32),
    position: (i32, i32),
    scale_factor: f64,
    is_primary: bool,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DesktopInfo { monitors: Vec::new() })
        .add_systems(Startup, (setup_camera, get_desktop_info))
        .add_systems(Update, display_desktop_info)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default(),
    ));
}

fn get_desktop_info(
    winit_windows: NonSend<WinitWindows>,
    mut desktop_info: ResMut<DesktopInfo>,
) {
    // Get the primary window to access the monitor information
    if let Some((_, window)) = winit_windows.windows.iter().next() {
        // Get all available monitors
        if let Some(primary_monitor) = window.primary_monitor() {
            let monitors = vec![primary_monitor.clone()];
            // Note: In winit, we can only easily get the primary monitor
            // For all monitors, we'd need to use platform-specific APIs

            for monitor in monitors {
                let size = monitor.size();
                let position = monitor.position();
                let scale_factor = monitor.scale_factor();
                let name = monitor.name();

                let monitor_info = MonitorInfo {
                    name,
                    size: (size.width, size.height),
                    position: (position.x, position.y),
                    scale_factor,
                    is_primary: true, // We only have the primary monitor
                };

                desktop_info.monitors.push(monitor_info);
            }
        }
    }
}

fn display_desktop_info(
    desktop_info: Res<DesktopInfo>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    text_entities: Query<Entity, With<Text>>,
) {
    // If we don't have text yet, create it
    if text_entities.is_empty() {
        commands.spawn((
            Text::new("Loading desktop info..."),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
        return;
    }

    // Update the text with desktop information
    if let Ok(mut text) = text_query.single_mut() {
        let mut display_text = "Desktop Information:\n".to_string();

        for (i, monitor) in desktop_info.monitors.iter().enumerate() {
            display_text.push_str(&format!(
                "Monitor {}: {}x{} at ({}, {}), Scale: {:.1}, Primary: {}\n",
                i + 1,
                monitor.size.0,
                monitor.size.1,
                monitor.position.0,
                monitor.position.1,
                monitor.scale_factor,
                monitor.is_primary
            ));

            if let Some(name) = &monitor.name {
                display_text.push_str(&format!("  Name: {}\n", name));
            }
        }

        if desktop_info.monitors.is_empty() {
            display_text.push_str("No monitor information available");
        }

        *text = Text::new(display_text);
    }
}
