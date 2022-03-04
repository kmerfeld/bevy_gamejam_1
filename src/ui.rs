use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerTextHealth;

#[derive(Component)]
pub struct EnemyTextHealth;

#[derive(Component)]
pub struct PlayerTextAmmo;

#[derive(Component)]
pub struct EnemyTextAmmo;

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
