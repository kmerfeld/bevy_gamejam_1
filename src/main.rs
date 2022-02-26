use bevy::core::FixedTimestep;
use bevy::prelude::*;
//use rand::prelude::random;

const TIME_STEP: f32 = 0.1;
const ARENA_WIDTH: u32 = 200;
const ARENA_HEIGHT: u32 = 200;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.00, 0.50, 0.70)))
        .add_system(update_position_text)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_startup_system(setup)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("textures/ships/ship (8).png");
    // let water_bkg = asset_server.load("assets/textures/tiles/tile_73.png");
    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("pos", text_style.clone(), text_alignment),
            ..Default::default()
        })
        .insert(PositionText);

    commands
        .spawn_bundle(SpriteBundle {
            texture: ship_handle,
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position { x: ARENA_WIDTH as i32 / 2, y: 10 })
        .insert(Player)
        .insert(Size::square(0.3));
}

fn update_position_text(
    _time: Res<Time>,
    mut query: Query<&mut Text, With<PositionText>>,
    player: Query<&mut Position, With<Player>>,
) {
    for p in player.iter() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{}/{}", p.x, p.y)
            //"this is text".to_owned();
        }
    }
}

fn ship_movement(
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&Player, &mut Transform)>,
    mut ship_positions: Query<&mut Position, With<Player>>,
) {
    let (ship, mut transform) = player_q.single_mut();

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
            movement_factor += 10.0;
        }
    }

    let rotation_delta = Quat::from_rotation_z(rotation_factor * 10.0 * TIME_STEP);
    transform.rotation *= rotation_delta;
    println!("{}", transform.rotation);

    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = movement_factor * 1.0;
    let translation_delta = movement_direction * movement_distance;
    
    transform.translation += translation_delta;
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
