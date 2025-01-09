use super::*;
use bevy::color::palettes::css::CRIMSON;
use bevy::prelude::*;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnSettingsMenuScreen;

pub fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (TextFont { font_size: 40.0, ..default() }, TextColor(TEXT_COLOR));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((Button, button_node.clone(), BackgroundColor(NORMAL_BUTTON), action))
                            .with_children(|parent| {
                                parent.spawn((Text::new(text), button_text_style.clone()));
                            });
                    }
                });
        });
}
