use bevy::core::FixedTimestep;
use bevy::math::const_vec2;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use heron::prelude::*;
use rand::Rng;

const TIME_STEP: f32 = 0.1;

const WINDOW_HEIGHT: f32 = 750.0;
const WINDOW_WIDTH: f32 = 750.0;
const BOUNDS: Vec2 = const_vec2!([WINDOW_HEIGHT, WINDOW_WIDTH]);

const FORWARD_MOVE_DIST: f32 = 10.0;

const SHIP_SIZE: f32 = 0.15;

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
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_rocks)
        .add_startup_system(spawn_player_ship)
        .add_startup_system(spawn_enemy_ships)
        .add_system(detect_collisions)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ship_movement),
        )
        .add_system(ship_collide)
        .add_plugins(DefaultPlugins)
        .run();
}

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

#[derive(PhysicsLayer)]
enum Layer {
    Enemy,
    Player,
    Rock,
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

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct TargetReticule;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct GameOverEvent;

#[derive(Component)]
struct Health {
    value: i32,
}

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
        let mut rock_x: f32 = rand::thread_rng().gen_range((-(WINDOW_WIDTH as f32) / 2.0) + 200.0, ((WINDOW_WIDTH as f32) / 2.0) - 200.0);
        let mut rock_y: f32 = rand::thread_rng().gen_range((-(WINDOW_HEIGHT as f32) / 2.0) + 100.0, ((WINDOW_HEIGHT as f32) / 2.0) - 100.0);
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
                    rock_x = rand::thread_rng().gen_range((-(WINDOW_WIDTH as f32) / 2.0) + 100.0, ((WINDOW_WIDTH as f32) / 2.0) - 100.0);
                    rock_y = rand::thread_rng().gen_range((-(WINDOW_HEIGHT as f32) / 2.0) + 100.0, ((WINDOW_HEIGHT as f32) / 2.0) - 100.0);
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
                    translation: Vec3::new(
                        rock_x,
                        rock_y,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Static)
            .insert(CollisionShape::Sphere { radius: rock_size * 100.0 })
            .insert(Size::square(rock_size));
    }
}

fn spawn_player_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
        .insert(
            Health { value: 3 },
        )
        .insert(Player)
        .insert(PlayerTurn(Turn::Player1))
        .insert(RigidBody::Static)
        .insert(CollisionShape::Sphere { radius: SHIP_SIZE * 100.0 })
        .insert(CollisionLayers::new(Layer::Player, Layer::Enemy))
        .insert(CollisionLayers::none()
                    .with_group(Layer::Player)
                    .with_masks(&[Layer::Enemy, Layer::Rock]))
        .insert(Size::square(SHIP_SIZE))
        .with_children(|parent| {
            parent
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(128. * 3.0))
                        .with_translation(Vec3::new(272.0, 0.0, 0.0))
                        .with_rotation(Quat::from_rotation_z(f32::to_radians(45.0))),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(TargetReticule);
            parent
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(128. * 3.0))
                        .with_translation(Vec3::new(-272.0, 0.0, 0.0))
                        .with_rotation(Quat::from_rotation_z(f32::to_radians(45.0))),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    visibility: Visibility { is_visible: false },
                    ..Default::default()
                })
                .insert(TargetReticule);
        });
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
        .insert(
            Health { value: 5 },
        )
        .insert(Player)
        .insert(RigidBody::Static)
        .insert(CollisionShape::Sphere { radius: SHIP_SIZE * 100.0 })
        .insert(CollisionLayers::none()
                    .with_group(Layer::Enemy)
                    .with_masks(&[Layer::Player, Layer::Rock]))
        .insert(PlayerTurn(Turn::Player2))
        .insert(Size::square(SHIP_SIZE));
}

fn ship_movement(
    mut player_turn: ResMut<PlayerTurn>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(With<Player>, &mut Transform, &PlayerTurn)>,
    mut targets: Query<(&mut Visibility, With<TargetReticule>)>,
) {
    for (_, mut transform, player) in player_q.iter_mut() {
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

            // move forward
            if keyboard_input.pressed(KeyCode::W) {
                movement_factor += FORWARD_MOVE_DIST;
            }

            //Toggle firing arcs when pressed
            if keyboard_input.pressed(KeyCode::Space) {
                for (mut visibility, _) in targets.iter_mut() {
                    visibility.is_visible = !(visibility.is_visible)
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

    if player_turn.0 == Turn::Player1 {
        // player_turn.0 = Turn::Player2;
    } else {
        player_turn.0 = Turn::Player1;
    }
}

fn detect_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} started to collide",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }

            CollisionEvent::Stopped(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} stopped colliding",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }
        }
    }
}

fn ship_collide(mut commands: Commands, mut events: EventReader<CollisionEvent>) {
    events
        .iter()
        .filter(|e| e.is_started())
        .for_each(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if (is_player(layers_1) && is_enemy(layers_2)) || (is_player(layers_2) && is_enemy(layers_1)) {
                println!("Collision between ships");
            } else if (is_player(layers_1) && is_rock(layers_2)) || (is_player(layers_2) && is_rock(layers_1)) {
                println!("Collision between ship and rock");
            }
        });
}

fn is_player(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Enemy) && layers.contains_group(Layer::Player)
}

fn is_enemy(layers: CollisionLayers) -> bool {
    !layers.contains_group(Layer::Player) && layers.contains_group(Layer::Enemy)
}

fn is_rock(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player) && layers.contains_group(Layer::Rock)
}

fn game_over(mut commands: Commands, mut reader: EventReader<GameOverEvent>) {
    if reader.iter().next().is_some() {
        println!("GAME OVER");
        // TODO: despawn everyone then respawn at starting positions
    }
}
