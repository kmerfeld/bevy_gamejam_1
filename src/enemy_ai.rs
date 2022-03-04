use bevy::prelude::*;
use heron::prelude::*;

pub fn think(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_turn: ResMut<crate::PlayerTurn>,
    player: Query<(With<crate::Player>, &Transform)>,
    mut enemy: Query<(
        With<crate::Enemy>,
        &mut Transform,
        &mut crate::Direction,
        Without<crate::Player>,
    )>,
) {
    if player_turn.0 == crate::Turn::Enemy {
        player_turn.0 = crate::Turn::Player;
        println!("{:?}", player_turn.0);
        for (_, p) in player.iter() {
            for (_, mut e, mut direction, _) in enemy.iter_mut() {
                let mut rotation_factor = 0.0;
                let mut movement_factor = 0.0;

                let player_q = get_player_direction(p, &e);

                if player_q == direction.d {
                    movement_factor += crate::FORWARD_MOVE_DIST;
                } else if (player_q - direction.d).abs() > 5 {
                    //TODO special case
                    //turn left
                    if direction.d == 0 {
                        direction.d = 7;
                    } else {
                        direction.d -= 1;
                    }
                    movement_factor += crate::FORWARD_MOVE_DIST;
                    rotation_factor += 1.0;
                } else if direction.d > player_q {
                    //turn left
                    if direction.d == 0 {
                        direction.d = 7;
                    } else {
                        direction.d -= 1;
                    }
                    movement_factor += crate::FORWARD_MOVE_DIST;
                    rotation_factor += 1.0;
                } else {
                    //turn right

                    if direction.d == 7 {
                        direction.d = 0;
                    } else {
                        direction.d += 1;
                    }
                    movement_factor += crate::FORWARD_MOVE_DIST;
                    rotation_factor -= 1.0;
                }

                for _ in 0..2 {
                    let rotation_delta =
                        Quat::from_rotation_z(rotation_factor * f32::to_radians(22.5));

                    // move and rotate
                    let movement_direction = e.rotation * Vec3::Y;
                    let movement_distance = movement_factor * 1.0;
                    let translation_delta = movement_direction * movement_distance;
                    e.translation += translation_delta;
                    e.rotation *= rotation_delta;
                }

                // map boundaries
                let extents = Vec3::from((crate::BOUNDS / 2.0, 0.0));
                e.translation = e.translation.min(extents).max(-extents);

                //GUN
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load("textures/ship_parts/cannonBall.png"),
                        transform: e.clone(),
                        ..Default::default()
                    })
                    .insert(RigidBody::Dynamic)
                    //.insert(CollisionShape::Sphere { radius: 10.0 })
                    .insert(Velocity::from_linear(crate::get_gun_arc(player_q) * 1000.0));
            }
        }
    }
}

//Try and figure out where the enemy is
fn get_player_direction(p: &Transform, e: &Transform) -> i32 {
    //Do we turn left or right?
    //Knowing our direction, which quadrant are they in
    //ex we are 7, they are 4
    //left is 3 moves, right is 5, so we go left

    //Do we shoot? Yes.

    let mut y = 0;
    if p.translation.y > e.translation.y {
        y = 1
    } else if p.translation.y < e.translation.y {
        y = -1
    }

    let mut x = 0;
    if p.translation.x > e.translation.x {
        x = 1
    } else if p.translation.x < e.translation.x {
        x = -1
    }

    //Here we have the player, quadrent

    pos_mapping(x, y)
}

fn pos_mapping(x: i32, y: i32) -> i32 {
    match (x, y) {
        (0, 1) => 0,
        (1, 1) => 1,
        (1, 0) => 2,
        (1, -1) => 3,
        (0, -1) => 4,
        (-1, -1) => 5,
        (-1, 0) => 6,
        (-1, 1) => 7,
        _ => 0,
    }
}
