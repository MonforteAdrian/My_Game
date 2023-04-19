use crate::{despawn_screen, loading::TextureAssets, AppState};
use bevy::prelude::*;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `AppState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_system(splash_setup.in_schedule(OnEnter(AppState::Splash)))
            // While in this state, run the `countdown` system
            .add_system(countdown.in_set(OnUpdate(AppState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system(despawn_screen::<OnSplashScreen>.in_schedule(OnExit(AppState::Splash)));
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let icon = texture_assets.bevy.clone();
    // Display the logo
    commands
        .spawn(ImageBundle {
            style: Style {
                // This will center the logo
                margin: UiRect::all(Val::Auto),
                // This will set the logo to be 200px wide, and auto adjust its height
                size: Size::new(Val::Px(200.0), Val::Auto),
                ..default()
            },
            image: UiImage::new(icon),
            ..default()
        })
        .insert(OnSplashScreen);
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::Menu);
    }
}
