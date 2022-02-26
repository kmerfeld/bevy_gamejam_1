use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::math::{const_vec2, Vec3Swizzles};
//use rand::prelude::random;

const TIME_STEP: f32 = 0.1;

const WINDOW_HEIGHT: f32 = 500.0;
const WINDOW_WIDTH: f32 = 500.0;

const ARENA_WIDTH: u32 = 200;
const ARENA_HEIGHT: u32 = 200;

const BOUNDS: Vec2 = const_vec2!([WINDOW_HEIGHT, WINDOW_WIDTH]);

const FORWARD_MOVE_DIST: f32 = 100.0;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
enum Turn {
    Player1,
    Player2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct PlayerTurn(Turn);

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Player1;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Player2;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(PlayerTurn(Turn::Player1))
        .insert_resource(ClearColor(Color::rgb(0.00, 0.50, 0.70)))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                // .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ship_movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ship_movement_2),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct PositionText;

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_ship = asset_server.load("textures/ships/ship (8).png");
    let enemy_ship = asset_server.load("textures/ships/ship (10).png");
    // let water_bkg = asset_server.load("assets/textures/tiles/tile_73.png");
    // let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            texture: player_ship,
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                translation: Vec3::new(100.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position { x: ARENA_WIDTH as i32 / 2, y: 10 })
        .insert(Player)
        .insert(PlayerTurn(Turn::Player1))
        .insert(Size::square(0.3));

    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_ship,
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                translation: Vec3::new(-100.0, 0.0, 0.0),
                rotation: Quat::from_rotation_z(f32::to_radians(180.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position { x: ARENA_WIDTH as i32 / 2, y: 10 })
        .insert(Player)
        .insert(PlayerTurn(Turn::Player2))
        .insert(Size::square(0.3));
}

fn ship_movement(
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&Player, &mut Transform)>,
    mut ship_positions: Query<&mut Position, With<Player>>,
) {
    for (ship, mut transform) in player_q.iter_mut() {

        let mut rotation_factor = 0.0;
        let mut movement_factor = 0.0;

        // rotate on left/right
        if keyboard_input.pressed(KeyCode::Left) {
            rotation_factor += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rotation_factor -= 1.0;
        }

        // move only on up
        for mut pos in ship_positions.iter_mut() {
            if keyboard_input.pressed(KeyCode::Up) {
                movement_factor += 20.0;
            }
        }

        let rotation_delta = Quat::from_rotation_z(rotation_factor * f32::to_radians(45.0));
        transform.rotation *= rotation_delta;

        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = movement_factor * 1.0;
        let translation_delta = movement_direction * movement_distance;
        
        transform.translation += translation_delta;

        let extents = Vec3::from((BOUNDS / 2.0, 0.0));
        transform.translation = transform.translation.min(extents).max(-extents);
    }
}

fn ship_movement_2(
    windows: Res<Windows>,
    mut player_turn: ResMut<PlayerTurn>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&Player, &mut Transform, &PlayerTurn)>,
    mut ship_positions: Query<&mut Position, With<Player>>,
) {
    println!("{:?}", player_turn.0);
    for (ship, mut transform, player) in player_q.iter_mut() {
        if player.0 == player_turn.0 {
            println!("{:?}", player.0);
            let mut rotation_factor = 0.0;
            let mut movement_factor = 0.0;

            // rotate on left/right
            if keyboard_input.pressed(KeyCode::A) {
                movement_factor += FORWARD_MOVE_DIST;
                rotation_factor += 1.0;
            }
            if keyboard_input.pressed(KeyCode::D) {
                movement_factor += FORWARD_MOVE_DIST;
                rotation_factor -= 1.0;
            }

            // rotate on left/right
            if keyboard_input.pressed(KeyCode::Q) {
                movement_factor += FORWARD_MOVE_DIST;
                rotation_factor += 2.0;
            }
            if keyboard_input.pressed(KeyCode::E) {
                movement_factor += FORWARD_MOVE_DIST;
                rotation_factor -= 2.0;
            }

            // move only on up
            for mut pos in ship_positions.iter_mut() {
                if keyboard_input.pressed(KeyCode::W) {
                    movement_factor += FORWARD_MOVE_DIST;
                }
            }

            let rotation_delta = Quat::from_rotation_z(rotation_factor * f32::to_radians(45.0));
            transform.rotation *= rotation_delta;

            let movement_direction = transform.rotation * Vec3::Y;
            let movement_distance = movement_factor * 1.0;
            let translation_delta = movement_direction * movement_distance;
            if translation_delta.x != 0.0 {
                println!("move 1: {}", translation_delta);
            } 
            transform.translation += translation_delta;

            let movement_direction = Vec3::Y * 0.0;
            let movement_distance = movement_factor * 1.0;
            let translation_delta = movement_direction * movement_distance;
            if translation_delta.x != 0.0 {
                println!("move 2: {}", translation_delta);
            } 
            transform.translation += translation_delta;

            let extents = Vec3::from((BOUNDS / 2.0, 0.0));
            transform.translation = transform.translation.min(extents).max(-extents);
        }  
    }

    if player_turn.0 == Turn::Player1 {
        player_turn.0 = Turn::Player2;
    } else {
        player_turn.0 = Turn::Player1;
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

// fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
//     fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
//         let tile_size = bound_window / bound_game;
//         pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
//     }
//     let window = windows.get_primary().unwrap();
//     for (pos, mut transform) in q.iter_mut() {
//         transform.translation = Vec3::new(
//             convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
//             convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
//             0.0,
//         );
//     }
// }
