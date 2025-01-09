use crate::{map::a_star, Creature, IsoGrid, PathfindingSteps, Position};
use bevy::{input::ButtonInput, log, math::Vec3, prelude::*};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
    }
}

pub fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale -= 0.1;
        }
        if keyboard_input.pressed(KeyCode::KeyQ) {
            ortho.scale += 0.1;
        }
        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_secs() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

pub fn on_click(
    ev: Trigger<Pointer<Click>>,
    mut sprites: Query<&mut Sprite>,
    pos: Query<&Position>,
    mut grid: ResMut<IsoGrid>,
    mut creature_query: Query<(&Position, &mut PathfindingSteps), With<Creature>>,
) {
    let Ok(pos) = pos.get(ev.entity()) else {
        return;
    };
    match ev.button {
        PointerButton::Primary => {
            for (creature_pos, mut creature_steps) in creature_query.iter_mut() {
                let Some(path) = a_star(*creature_pos, *pos, |_, h| {
                    (grid.entities.contains_key(&h) && !grid.blocked_coords.contains(&h)).then_some(1)
                }) else {
                    log::info!("No path found");
                    return;
                };
                creature_steps.steps.clear();
                creature_steps.steps = path
                    .into_iter()
                    .inspect(|h| {
                        if grid.blocked_coords.contains(h) {
                            log::error!("A star picked a blocked coord: {h:?}");
                        }
                    })
                    .collect();
            }
        }
        PointerButton::Secondary => {
            let Ok(mut sprite) = sprites.get_mut(ev.entity()) else {
                return;
            };
            let Some(ref mut atlas) = &mut sprite.texture_atlas else {
                return;
            };
            if grid.blocked_coords.contains(pos) {
                grid.blocked_coords.remove(pos);
                atlas.index = 5;
            } else {
                grid.blocked_coords.insert(*pos);
                atlas.index = 2;
            }
        }
        PointerButton::Middle => {}
    }
}
