use super::*;
use bevy::color::palettes::css::CRIMSON;
use bevy::prelude::*;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
pub struct OnDisplaySettingsMenuScreen;

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

pub fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
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
            OnDisplaySettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node { flex_direction: FlexDirection::Column, align_items: AlignItems::Center, ..default() },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
                    parent
                        .spawn((Node { align_items: AlignItems::Center, ..default() }, BackgroundColor(CRIMSON.into())))
                        .with_children(|parent| {
                            // Display a label for the current setting
                            parent.spawn((Text::new("Display Quality"), button_text_style.clone()));
                            // Display a button for each possible value
                            for quality_setting in [DisplayQuality::Low, DisplayQuality::Medium, DisplayQuality::High] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node { width: Val::Px(150.0), height: Val::Px(65.0), ..button_node.clone() },
                                    BackgroundColor(NORMAL_BUTTON),
                                    quality_setting,
                                ));
                                entity.with_children(|parent| {
                                    parent
                                        .spawn((Text::new(format!("{quality_setting:?}")), button_text_style.clone()));
                                });
                                if *display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });
                    // Display the back button to return to the settings screen
                    parent
                        .spawn((Button, button_node, BackgroundColor(NORMAL_BUTTON), MenuButtonAction::BackToSettings))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Back"), button_text_style));
                        });
                });
        });
}
