use bevy::{app::AppExit, prelude::*};
use super::{despawn_screen, AppState, TEXT_COLOR};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(MenuState::Disabled)
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup))
            //main
            .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(MenuState::Main).with_system(despawn_screen::<OnMainMenuScreen>))
            .add_system_set(SystemSet::on_update(AppState::Menu)
                .with_system(menu_action)
                .with_system(button_system)
            );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Main,
    Disabled,
}

#[derive(Component)]
struct OnMainMenuScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.15);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn setup(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::Main);
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        position: UiRect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Display the game name
            parent.spawn(
                TextBundle::from_section(
                    "Settlers of the Honeyverse",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            // Display three buttons for each action available from the main menu:
            // - new game
            // - settings
            // - quit
            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/GameIcons/right.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "New Game",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    style: button_style,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/GameIcons/exitRight.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style,
                        image: UiImage(icon),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Quit", button_text_style));
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(AppState::Game).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                }
            }
        }
    }
}
