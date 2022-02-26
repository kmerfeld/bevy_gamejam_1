use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::random;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            // <--
            title: "bevy!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_system(update_position_text)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_startup_system(setup)
        .add_startup_system(spawn_snake)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(snake_movement_input),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

#[derive(Component)]
struct PositionText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

fn update_position_text(
    time: Res<Time>,
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

const ARENA_WIDTH: u32 = 200;
const ARENA_HEIGHT: u32 = 200;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
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

#[derive(Component)]
struct Player;

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position { x: 1, y: 1 })
        .insert(Player)
        .insert(Size::square(0.8));
}

fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Position, With<Player>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 10;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 10;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 10;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 10;
        }
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
