use bevy::prelude::*;
use bevy::window::{MonitorSelection, Window, WindowMode};
use crate::components::{MenuVisible, OptionsMenu, WindowModeButton};

pub fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            OptionsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderRadius::all(Val::Px(10.0)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Options"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Window Mode Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                            BorderRadius::all(Val::Px(5.0)),
                            WindowModeButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Toggle Window Mode"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Close Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.6, 0.3, 0.3)),
                            BorderRadius::all(Val::Px(5.0)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Close"),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn handle_menu_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Option<&WindowModeButton>), (Changed<Interaction>, With<Button>)>,
    mut windows: Query<&mut Window>,
    mut menu_visible: ResMut<MenuVisible>,
    mut commands: Commands,
    menu_query: Query<Entity, With<OptionsMenu>>,
) {
    for (interaction, mut color, window_mode_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if window_mode_button.is_some() {
                    // Handle window mode toggle
                    if let Ok(mut window) = windows.single_mut() {
                        window.mode = match window.mode {
                            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                            WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
                            WindowMode::Fullscreen(_, _) => WindowMode::Windowed,
                        };
                    }
                } else {
                    // Close button pressed
                    menu_visible.0 = false;
                    for entity in menu_query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.6, 0.6, 0.6);
            }
            Interaction::None => {
                if window_mode_button.is_some() {
                    color.0 = Color::srgb(0.4, 0.4, 0.4);
                } else {
                    color.0 = Color::srgb(0.6, 0.3, 0.3);
                }
            }
        }
    }
}
