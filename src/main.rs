use::bevy::{
    input::{ keyboard::KeyCode, Input },
    prelude::*,
    render::camera::RenderTarget,
};

#[derive(Component)]
struct MainCamera;

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
    }
}

fn mouse_input(
    buttons: Res<Input<MouseButton>>, 
) {
    if buttons.just_pressed(MouseButton::Left) {
        info!("'Left click' just pressed");
    }

    if buttons.just_pressed(MouseButton::Right) {
        info!("'Right click' just pressed");
    }
}

fn keyboard_input(
    key: Res<Input<KeyCode>>
) {
    if key.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    // By default camera Z is 999.9 so max z visible is 999
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    // Load the assets
    // The center of the asset is the coords where the engine spawn it
    let cube = asset_server.load("textures/cube.png");

    // TODO check block size automatically
    let size_block: f32 = 64.0;

    // ALWAYS TOP-DOWN AND RIGHT-LEFT
    let original_x: f32 = 0.0;
    let original_y: f32 = 5.0 * size_block;

    // Grid generation
    for high in 0..3 {
        for column in 0..20 {
            for row in 0..20 {
                // (-1.0) as it goes left and then down
                // row * (size_block) to calculate the next block in the left down of the previous one
                // column * (size_block) to calculate the next block in the right down of the previos row
                let position_x = (-1.0) * (row as f32 * (size_block * 0.5)) + (column as f32 * (size_block * 0.5)) + original_x;
                let position_y = (-1.0) * (row as f32 * (size_block * 0.25)) - (column as f32 * (size_block * 0.25)) + original_y;
                if (row < 14 && column < 15 && high ==2) || (row < 15 && column < 19  && high == 1) || high == 0{
                    commands.spawn_bundle(SpriteBundle {
                        texture: cube.clone(),
                        transform: Transform::from_xyz(0.0 + position_x, 0.0 + position_y, 0.0),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Mi juego!".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(my_cursor_system)
        .add_system(mouse_input)
        .add_system(keyboard_input)
        .add_startup_system(setup)
        .run()
}
