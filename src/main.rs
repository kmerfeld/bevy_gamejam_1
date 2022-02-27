use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::math::const_vec2;
use bevy::sprite::collide_aabb::collide;
use rand::Rng;

const TIME_STEP: f32 = 0.1;

const WINDOW_HEIGHT: f32 = 750.0;
const WINDOW_WIDTH: f32 = 750.0;
const BOUNDS: Vec2 = const_vec2!([WINDOW_HEIGHT, WINDOW_WIDTH]);

const ARENA_WIDTH: u32 = 300;
const ARENA_HEIGHT: u32 = 300;

const FORWARD_MOVE_DIST: f32 = 10.0;

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
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_startup_system(setup_camera)
        .add_startup_system(setup_rocks)
        .add_startup_system(spawn_player_ship)
        .add_startup_system(spawn_enemy_ships)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ship_movement),
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

struct GameOverEvent;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_rocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rocks: [Handle<Image>; 3] = [asset_server.load("textures/tiles/tile_49.png"),
                                     asset_server.load("textures/tiles/tile_50.png"),
                                     asset_server.load("textures/tiles/tile_51.png")];

    let mut spawned_rocks = vec![];

    for i in 0..3 {
        let rock_type: usize = rand::thread_rng().gen_range(0, rocks.len());
        let mut rock_x: f32 = rand::thread_rng().gen_range(60.0, (ARENA_WIDTH as f32) - 50.0);
        let mut rock_y: f32 = rand::thread_rng().gen_range(60.0, (ARENA_HEIGHT as f32) - 60.0);
        let rock_rot: f32 = rand::thread_rng().gen_range(0.0, 360.0);
        let rock_size: f32 = rand::thread_rng().gen_range(0.3, 1.1);

        // make sure rocks are spaced apart
        if spawned_rocks.len() > 0 {
            for j in 0..spawned_rocks.len() {
                let spawned_tmp: (f32, f32) = spawned_rocks[j];
                let spawned_x: f32 = spawned_tmp.0;
                let spawned_y: f32 = spawned_tmp.1;

                while (rock_x >= spawned_x - 50.0 && rock_x <= spawned_x + 50.0) && 
                      (rock_y >= spawned_y - 50.0 && rock_y <= spawned_x + 50.0) {
                    rock_x = rand::thread_rng().gen_range(60.0, (ARENA_WIDTH as f32) - 50.0);
                    rock_y = rand::thread_rng().gen_range(60.0, (ARENA_HEIGHT as f32) - 60.0);
                }
            }
            spawned_rocks.push((rock_x, rock_y));
        } else {
            spawned_rocks.push((rock_x, rock_y));
        }

        // println!("Spaned rocks: {:?}", spawned_rocks);

        commands
            .spawn_bundle(SpriteBundle {
                texture: rocks[rock_type].clone(),
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    rotation: Quat::from_rotation_z(f32::to_radians(rock_rot)),
                    ..Default::default()
                },
                ..Default::default()
        })
        .insert(Position {
            x: rock_x as i32,
            y: rock_y as i32,
        })
        .insert(Size::square(rock_size));
    }
}

fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_ship = asset_server.load("textures/ships/ship (10).png");
    // let ship_rot: i32 = rand::thread_rng().gen_range(-90, 1);

    commands
        .spawn_bundle(SpriteBundle {
            texture: player_ship,
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                translation: Vec3::new((ARENA_WIDTH as f32) - 30.0, -(ARENA_HEIGHT as f32) + 30.0, 0.0),
                // rotation: Quat::from_rotation_z(f32::to_radians(ship_rot)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerTurn(Turn::Player1))
        .insert(Size::square(0.3));    
}

fn spawn_enemy_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_ship = asset_server.load("textures/ships/ship (8).png");
    // let ship_rot: i32 = rand::thread_rng().gen_range(90, 181);

    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_ship,
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                translation: Vec3::new(-(ARENA_WIDTH as f32) + 30.0, (ARENA_HEIGHT as f32) - 30.0, 0.0),
                rotation: Quat::from_rotation_z(f32::to_radians(180.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerTurn(Turn::Player2))
        .insert(Size::square(0.3));
}

fn ship_movement(
    mut player_turn: ResMut<PlayerTurn>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&Player, &mut Transform, &PlayerTurn)>,
    ship_positions: Query<&mut Position, With<Player>>,
) {
    for (ship, mut transform, player) in player_q.iter_mut() {
        if player.0 == player_turn.0 {
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

            // move forward
            if keyboard_input.pressed(KeyCode::W) {
                movement_factor += FORWARD_MOVE_DIST;
            }

            for i in 0..2 {
                let rotation_delta = Quat::from_rotation_z(rotation_factor * f32::to_radians(22.5));
            
                // move and rotate
                let movement_direction = transform.rotation * Vec3::Y;
                let movement_distance = movement_factor * 1.0;
                let translation_delta = movement_direction * movement_distance;
                transform.translation += translation_delta;
                transform.rotation *= rotation_delta;    
            }

            // map boundaries
            let extents = Vec3::from((BOUNDS / 2.0, 0.0));
            transform.translation = transform.translation.min(extents).max(-extents);
        }  
    }

    if player_turn.0 == Turn::Player1 {
        // player_turn.0 = Turn::Player2;
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

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
) {
    if reader.iter().next().is_some() {
        // if you hit a rock, hit the enemy ship, get killed by the enemy, or kill the enemy - respawn
    }
}
