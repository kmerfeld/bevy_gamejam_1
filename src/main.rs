use bevy::core::FixedTimestep;
use bevy::math::const_vec2;
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;
use std::string::String;

mod enemy_ai;
mod ui;

const TIME_STEP: f32 = 0.1;

const WINDOW_HEIGHT: f32 = 750.0;
const WINDOW_WIDTH: f32 = 750.0;
const BOUNDS: Vec2 = const_vec2!([WINDOW_HEIGHT, WINDOW_WIDTH]);

const FORWARD_MOVE_DIST: f32 = 10.0;

const SHIP_SIZE: f32 = 0.15;

const MAX_ROUNDS: i32 = 10;
const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Win,
    Lose,
}

fn main() {
    App::new()
        .add_state(AppState::InGame)
        .add_system_set(SystemSet::on_enter(AppState::Lose).with_system(ui::lose_message))
        .add_system_set(SystemSet::on_enter(AppState::Win).with_system(ui::win_message))
        .add_system_set(SystemSet::on_update(AppState::Win).with_system(ui::button_system))
        .add_system_set(SystemSet::on_update(AppState::Lose).with_system(ui::button_system))
        .insert_resource(WindowDescriptor {
            title: "bevy!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(enemy_ai::think),
        )
        .insert_resource(PlayerTurn(Turn::Player))
        .insert_resource(ClearColor(Color::rgb(0.00, 0.50, 0.70)))
        .insert_resource(Round { count: MAX_ROUNDS })
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_rocks)
        .add_startup_system(ui::setup)
        .add_system(ui::enemy_text_update_system)
        .add_system(ui::game_over)
        .add_system(ui::player_text_update_system)
        .add_startup_system(spawn_player_ship)
        .add_system(
            enemy_ai::think
                .label(TurnLabel::Enemy)
                .before(TurnLabel::Player),
        )
        .add_startup_system(spawn_enemy_ships)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ship_movement.label(TurnLabel::Player)),
        )
        .add_system(ship_collision)
        .add_system(cannon_fodder)
        .add_plugins(DefaultPlugins)
        .run();
}

// players
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CannonBall;

#[derive(Component)]
pub struct Health {
    pub value: i32,
}

#[derive(Component)]
pub struct ActionPoints {
    pub value: i32,
}

///0 => up
///1 => up_left
///2 => left
///3 => down_left
///4 => down
///5 => down_right
///6 => right
///7 => up_right
#[derive(Component)]
pub struct Direction {
    d: i32,
}

// combat
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum Turn {
    Player,
    Enemy,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct PlayerTurn(Turn);

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum TurnLabel {
    Player,
    Enemy,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct Round {
    count: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct TargetReticule;

// collision
#[derive(PhysicsLayer)]
enum Layer {
    Player,
    Enemy,
    Rock,
    CannonBall,
}

// game
#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct GameOverEvent;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_rocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rocks: [Handle<Image>; 3] = [
        asset_server.load("textures/tiles/tile_49.png"),
        asset_server.load("textures/tiles/tile_50.png"),
        asset_server.load("textures/tiles/tile_51.png"),
    ];

    let mut spawned_rocks = vec![];

    for _ in 0..3 {
        let rock_type: usize = rand::thread_rng().gen_range(0, rocks.len());
        let mut rock_x: f32 = rand::thread_rng().gen_range(
            (-(WINDOW_WIDTH as f32) / 2.0) + 200.0,
            ((WINDOW_WIDTH as f32) / 2.0) - 200.0,
        );
        let mut rock_y: f32 = rand::thread_rng().gen_range(
            (-(WINDOW_HEIGHT as f32) / 2.0) + 100.0,
            ((WINDOW_HEIGHT as f32) / 2.0) - 100.0,
        );
        let rock_rot: f32 = rand::thread_rng().gen_range(0.0, 360.0);
        let rock_size: f32 = rand::thread_rng().gen_range(0.4, 1.1);

        // make sure rocks are spaced apart
        if spawned_rocks.len() > 0 {
            for j in 0..spawned_rocks.len() {
                let spawned_tmp: (f32, f32) = spawned_rocks[j];
                let spawned_x: f32 = spawned_tmp.0;
                let spawned_y: f32 = spawned_tmp.1;

                while (rock_x >= spawned_x - 60.0 && rock_x <= spawned_x + 60.0)
                    && (rock_y >= spawned_y - 60.0 && rock_y <= spawned_y + 60.0)
                {
                    rock_x = rand::thread_rng().gen_range(
                        (-(WINDOW_WIDTH as f32) / 2.0) + 100.0,
                        ((WINDOW_WIDTH as f32) / 2.0) - 100.0,
                    );
                    rock_y = rand::thread_rng().gen_range(
                        (-(WINDOW_HEIGHT as f32) / 2.0) + 100.0,
                        ((WINDOW_HEIGHT as f32) / 2.0) - 100.0,
                    );
                }
            }
            spawned_rocks.push((rock_x, rock_y));
        } else {
            spawned_rocks.push((rock_x, rock_y));
        }

        commands
            .spawn_bundle(SpriteBundle {
                texture: rocks[rock_type].clone(),
                transform: Transform {
                    scale: Vec3::new(2.0, 2.0, 2.0),
                    rotation: Quat::from_rotation_z(f32::to_radians(rock_rot)),
                    translation: Vec3::new(rock_x, rock_y, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(CollisionShape::Sphere {
                radius: rock_size * 13.0,
            })
            .insert(
                CollisionLayers::none()
                    .with_group(Layer::Rock)
                    .with_masks(&[Layer::Enemy, Layer::Player, Layer::CannonBall]),
            );
    }
}

fn spawn_player_ship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_ship = asset_server.load("textures/ships/ship (10).png");
    // let ship_rot: i32 = rand::thread_rng().gen_range(-90, 1);

    commands
        .spawn_bundle(SpriteBundle {
            texture: player_ship,
            transform: Transform {
                scale: Vec3::new(0.75, 0.75, 0.75),
                translation: Vec3::new(
                    (WINDOW_WIDTH as f32) - 500.0,
                    -(WINDOW_HEIGHT as f32) + 500.0,
                    0.0,
                ),
                // rotation: Quat::from_rotation_z(f32::to_radians(ship_rot)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Health { value: 3 })
        .insert(ActionPoints { value: 0 })
        .insert(Direction { d: 0 })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Sphere {
            radius: SHIP_SIZE * 100.0,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Player)
                .with_masks(&[Layer::Enemy, Layer::Rock, Layer::CannonBall]),
        );
}

fn spawn_enemy_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_ship = asset_server.load("textures/ships/ship (8).png");
    // let ship_rot: i32 = rand::thread_rng().gen_range(90, 181);

    commands
        .spawn_bundle(SpriteBundle {
            texture: enemy_ship,
            transform: Transform {
                scale: Vec3::new(0.75, 0.75, 0.75),
                translation: Vec3::new(
                    -(WINDOW_WIDTH as f32) + 500.0,
                    (WINDOW_HEIGHT as f32) - 500.0,
                    0.0,
                ),
                rotation: Quat::from_rotation_z(f32::to_radians(180.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Direction { d: 4 })
        .insert(Health { value: 5 })
        .insert(ActionPoints { value: 0 })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Sphere {
            radius: SHIP_SIZE * 100.0,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Enemy)
                .with_masks(&[Layer::Player, Layer::Rock, Layer::CannonBall]),
        );
}

pub fn get_gun_arc(d: i32) -> Vec3 {
    match d {
        0 => Vec3::new(0.0, 1.0, 0.0),
        1 => Vec3::new(1.0, 1.0, 0.0),
        2 => Vec3::new(1.0, 0.0, 0.0),
        3 => Vec3::new(1.0, -1.0, 0.0),
        4 => Vec3::new(0.0, -1.0, 0.0),
        5 => Vec3::new(-1.0, -1.0, 0.0),
        6 => Vec3::new(-1.0, 0.0, 0.0),
        7 => Vec3::new(-1.0, 1.0, 0.0),
        _ => Vec3::new(0.0, 0.0, 0.0),
    }
}

fn ship_movement(
    mut commands: Commands,
    mut player_turn: ResMut<PlayerTurn>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut player: Query<(
        With<Player>,
        &mut Transform,
        &mut Direction,
        &mut ActionPoints,
    )>,
) {
    for (_, mut transform, mut direction, mut ap) in player.iter_mut() {
        if Turn::Player == player_turn.0 {
            let mut rotation_factor = 0.0;
            let mut movement_factor = 0.0;

            if keyboard_input.pressed(KeyCode::Space) {
                if ap.value == 3 {
                    let mut l_dir = direction.d - 2;
                    if l_dir < 0 {
                        l_dir = direction.d + 8 - 2;
                    }
                    let mut r_dir = direction.d + 2;
                    if r_dir > 7 {
                        r_dir = direction.d - 8 + 2;
                    }

                    let left = get_gun_arc(l_dir);
                    let right = get_gun_arc(r_dir);

                    //Handle direction to rotatio        t.translation.x += 10.0;
                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load("textures/ship_parts/cannonBall.png"),
                            transform: transform.clone(),
                            ..Default::default()
                        })
                        .insert(RigidBody::Dynamic)
                        .insert(CollisionShape::Sphere { radius: 10.0 })
                        .insert(
                            CollisionLayers::none()
                                .with_group(crate::Layer::CannonBall)
                                .with_masks(&[crate::Layer::Rock, crate::Layer::Enemy]),
                        )
                        .insert(Velocity::from_linear(left * 1000.0));

                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: asset_server.load("textures/ship_parts/cannonBall.png"),
                            transform: transform.clone(),
                            ..Default::default()
                        })
                        .insert(RigidBody::Dynamic)
                        .insert(CollisionShape::Sphere { radius: 10.0 })
                        .insert(
                            CollisionLayers::none()
                                .with_group(crate::Layer::CannonBall)
                                .with_masks(&[crate::Layer::Rock, crate::Layer::Enemy]),
                        )
                        .insert(Velocity::from_linear(right * 1000.0));

                    ap.value -= 3;
                }
            } else {
                // rotate on left/right
                if keyboard_input.pressed(KeyCode::A) {
                    if direction.d == 0 {
                        direction.d = 7;
                    } else {
                        direction.d -= 1;
                    }
                    movement_factor += FORWARD_MOVE_DIST;
                    rotation_factor += 1.0;

                    if ap.value < 3 {
                        ap.value += 1;
                    }
                    
                    player_turn.0 = Turn::Enemy;
                }
                if keyboard_input.pressed(KeyCode::D) {
                    if direction.d == 7 {
                        direction.d = 0;
                    } else {
                        direction.d += 1;
                    }
                    movement_factor += FORWARD_MOVE_DIST;
                    rotation_factor -= 1.0;

                    if ap.value < 3 {
                        ap.value += 1;
                    }

                    player_turn.0 = Turn::Enemy;
                }

                // move forward
                if keyboard_input.pressed(KeyCode::W) {
                    movement_factor += FORWARD_MOVE_DIST;
                    if ap.value < 3 {
                        ap.value += 1;
                    }
                    player_turn.0 = Turn::Enemy;
                }
            }

            for _ in 0..2 {
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
}

fn ship_collision(
    mut events: EventReader<CollisionEvent>,
    mut query: QuerySet<(
        QueryState<&mut Health, With<Player>>,
        QueryState<&mut Health, With<Enemy>>,
    )>,
) {
    events.iter().filter(|e| e.is_started()).for_each(|event| {
        let (layers_1, layers_2) = event.collision_layers();
        let (entity_1, entity_2) = event.rigid_body_entities();

        if (is_player(layers_1) && is_enemy(layers_2))
            || (is_player(layers_2) && is_enemy(layers_1))
        {
            println!("Collision between ships");
            for mut health in query.q0().iter_mut() {
                health.value -= 1;
                println!("Player health: {}", health.value); // DEBUG!
            }
            for mut health in query.q1().iter_mut() {
                health.value -= 1;
                println!("Enemy health: {}", health.value); // DEBUG!
            }
        }

        if (is_player(layers_1) && is_rock(layers_2)) || (is_player(layers_2) && is_rock(layers_1))
        {
            println!("Collision between player and rock");
            for mut health in query.q0().iter_mut() {
                health.value -= 1;
                println!("Player health: {}", health.value); // DEBUG!
            }
        }

        if (is_enemy(layers_1) && is_rock(layers_2)) || (is_enemy(layers_2) && is_rock(layers_1)) {
            println!("Collision between enemy and rock");
            println!("Enemy is stunned!");
        }
    });
}

fn cannon_fodder(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut queryset: QuerySet<(
        QueryState<&mut Health, With<Player>>,
        QueryState<&mut Health, With<Enemy>>,
    )>,
) {
    events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (layers_1, layers_2) = event.collision_layers();
            let (entity_1, entity_2) = event.rigid_body_entities();

            if is_player(layers_1) && is_cannonball(layers_2) {
                println!("Player hit by cannon");
                for mut health in queryset.q0().iter_mut() {
                    health.value -= 1;
                    println!("Player health: {}", health.value); // DEBUG!
                }
                return Some(entity_2);
            } else if is_cannonball(layers_2) && is_player(layers_1) {
                println!("Player hit by cannon");
                for mut health in queryset.q0().iter_mut() {
                    health.value -= 1;
                    println!("Player health: {}", health.value); // DEBUG!
                }
                return Some(entity_1);
            }

            if is_enemy(layers_1) && is_cannonball(layers_2) {
                println!("Enemy hit by cannon");
                for mut health in queryset.q1().iter_mut() {
                    health.value -= 1;
                    println!("Enemy health: {}", health.value); // DEBUG!
                }
                return Some(entity_2);
            } else if is_cannonball(layers_2) && is_enemy(layers_1) {
                println!("Enemy hit by cannon");
                for mut health in queryset.q1().iter_mut() {
                    health.value -= 1;
                    println!("Enemy health: {}", health.value); // DEBUG!
                }
                return Some(entity_1);
            } else {
                return None;
            }
        })
        .for_each(|cannon_entity| commands.entity(cannon_entity).despawn());
}

fn is_player(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Enemy) && layers.contains_group(Layer::Player)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Enemy)
}

fn is_rock(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player)
        && !layers.contains_group(Layer::Enemy)
        && layers.contains_group(Layer::Rock)
}

fn is_cannonball(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player)
        && !layers.contains_group(Layer::Enemy)
        && layers.contains_group(Layer::CannonBall)
}
