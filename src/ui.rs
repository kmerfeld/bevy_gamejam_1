use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerTextHealth;

#[derive(Component)]
pub struct EnemyTextHealth;

#[derive(Component)]
pub struct PlayerTextAmmo;

#[derive(Component)]
pub struct EnemyTextAmmo;

pub fn game_over(
    mut app_state: ResMut<State<crate::AppState>>,
    player: Query<&crate::Health, (With<crate::Player>, Without<crate::Enemy>)>,
    enemy: Query<&crate::Health, (With<crate::Enemy>, Without<crate::Player>)>,
) {
    for health in player.iter() {
        if health.value <= 0 {
            match app_state.current() {
                crate::AppState::InGame => app_state.set(crate::AppState::Lose).unwrap(),
                _ => (),
            }
        }
    }
    for health in enemy.iter() {
        if health.value <= 0 {
            match app_state.current() {
                crate::AppState::InGame => app_state.set(crate::AppState::Win).unwrap(),
                _ => (),
            }
        }
    }
}

pub fn button_system(
    app_state: ResMut<State<crate::AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "exit".to_string();
                *color = PRESSED_BUTTON.into();
                std::process::exit(0);
            }
            Interaction::Hovered => {
                text.sections[0].value = "exit".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                match app_state.current() {
                    crate::AppState::Win => text.sections[0].value = "You Win".to_string(),
                    crate::AppState::Lose => text.sections[0].value = "You Lose".to_string(),
                    _ => (),
                }
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn end_message(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Regular.ttf");
    let text_style = TextStyle {
        font,
        font_size: 12.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Left,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("Health: {}", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(-350.0, -330.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerTextHealth);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("Enemy Health: {}", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(-350.0, -340.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyTextHealth);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Turns till cannon readies: {}",
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform {
                translation: Vec3::new(-350.0, -350.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerTextAmmo);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Turns till enemy can shoot: {}",
                text_style.clone(),
                text_alignment,
            ),
            transform: Transform {
                translation: Vec3::new(-350.0, -360.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EnemyTextAmmo);
}

pub fn enemy_text_update_system(
    mut e_health_text: Query<&mut Text, (With<EnemyTextHealth>, Without<EnemyTextAmmo>)>,
    mut e_ammo_text: Query<&mut Text, (With<EnemyTextAmmo>, Without<EnemyTextHealth>)>,
    enemy: Query<
        (&crate::Health, &crate::ActionPoints),
        (With<crate::Enemy>, Without<crate::Player>),
    >,
) {
    for (e_health, e_ammo) in enemy.iter() {
        for mut h_text in e_health_text.iter_mut() {
            h_text.sections[0].value = format!("Enemy Health: {}", e_health.value);
        }
        for mut ammo in e_ammo_text.iter_mut() {
            ammo.sections[0].value = format!("Turn till enemy cannon readies: {}", e_ammo.value);
        }
    }
}
pub fn player_text_update_system(
    mut p_health_text: Query<&mut Text, (With<PlayerTextHealth>, Without<PlayerTextAmmo>)>,
    mut p_ammo_text: Query<&mut Text, (With<PlayerTextAmmo>, Without<PlayerTextHealth>)>,
    player: Query<
        (&crate::Health, &crate::ActionPoints),
        (With<crate::Player>, Without<crate::Enemy>),
    >,
) {
    for (p_health, p_ammo) in player.iter() {
        for mut h_text in p_health_text.iter_mut() {
            h_text.sections[0].value = format!("Health: {}", p_health.value);
        }
        for mut h_ammo in p_ammo_text.iter_mut() {
            h_ammo.sections[0].value = format!("Turns till cannon readies: {:?}", p_ammo.value);
        }
    }
}
