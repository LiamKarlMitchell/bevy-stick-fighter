use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::stick_fighter::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_arena, spawn_stick_fighter))
        .add_systems(
            Update,
            (
                player_movement,
                player_attack,
                check_grounded,
                update_animation_state,
                despawn_expired_hitboxes,
                apply_damage,
            ),
        );
}

/// Setup the fighting arena with a ground plane
fn setup_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(20.0, 20.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        RigidBody::Static,
        Collider::cuboid(20.0, 0.1, 20.0),
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });
}

/// Spawn a stick figure fighter
fn spawn_stick_fighter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let stick_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        ..default()
    });

    // Create the stick figure at origin
    let root = commands
        .spawn((
            StickFigureRoot,
            StickFighter::default(),
            PlayerController { player_id: 0 },
            AnimationState::Idle,
            GroundedState::default(),
            Transform::from_xyz(0.0, 5.0, 0.0),
            Visibility::default(),
            RigidBody::Dynamic,
            Collider::capsule(0.3, 1.0),
            LockedAxes::ROTATION_LOCKED, // Prevent the character from tipping over
            LinearVelocity::default(),
        ))
        .id();

    // Head
    let head = commands
        .spawn((
            BodyPart::Head,
            Mesh3d(meshes.add(Sphere::new(0.3))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(0.0, 1.2, 0.0),
        ))
        .id();

    // Torso
    let torso = commands
        .spawn((
            BodyPart::Torso,
            Mesh3d(meshes.add(Capsule3d::new(0.15, 0.8))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(0.0, 0.4, 0.0),
        ))
        .id();

    // Arms
    let left_upper_arm = commands
        .spawn((
            BodyPart::LeftUpperArm,
            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.4))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(0.3, 0.8, 0.0).with_rotation(Quat::from_rotation_z(0.5)),
        ))
        .id();

    let left_lower_arm = commands
        .spawn((
            BodyPart::LeftLowerArm,
            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.4))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(0.6, 0.5, 0.0).with_rotation(Quat::from_rotation_z(0.8)),
        ))
        .id();

    let right_upper_arm = commands
        .spawn((
            BodyPart::RightUpperArm,
            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.4))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(-0.3, 0.8, 0.0).with_rotation(Quat::from_rotation_z(-0.5)),
        ))
        .id();

    let right_lower_arm = commands
        .spawn((
            BodyPart::RightLowerArm,
            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.4))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(-0.6, 0.5, 0.0).with_rotation(Quat::from_rotation_z(-0.8)),
        ))
        .id();

    // Legs
    let left_upper_leg = commands
        .spawn((
            BodyPart::LeftUpperLeg,
            Mesh3d(meshes.add(Capsule3d::new(0.1, 0.5))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(0.15, -0.3, 0.0),
        ))
        .id();

    let left_lower_leg = commands
        .spawn((
            BodyPart::LeftLowerLeg,
            Mesh3d(meshes.add(Capsule3d::new(0.1, 0.5))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(0.15, -0.9, 0.0),
        ))
        .id();

    let right_upper_leg = commands
        .spawn((
            BodyPart::RightUpperLeg,
            Mesh3d(meshes.add(Capsule3d::new(0.1, 0.5))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(-0.15, -0.3, 0.0),
        ))
        .id();

    let right_lower_leg = commands
        .spawn((
            BodyPart::RightLowerLeg,
            Mesh3d(meshes.add(Capsule3d::new(0.1, 0.5))),
            MeshMaterial3d(stick_material.clone()),
            Transform::from_xyz(-0.15, -0.9, 0.0),
        ))
        .id();

    // Add all body parts as children of the root
    commands.entity(root).add_children(&[
        head,
        torso,
        left_upper_arm,
        left_lower_arm,
        right_upper_arm,
        right_lower_arm,
        left_upper_leg,
        left_lower_leg,
        right_upper_leg,
        right_lower_leg,
    ]);
}

/// Handle player movement input
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&StickFighter, &mut LinearVelocity, &GroundedState), With<PlayerController>>,
) {
    for (fighter, mut velocity, grounded) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // Horizontal movement
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
        }

        // Normalize direction for consistent diagonal movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // Apply horizontal velocity
        velocity.x = direction.x * fighter.speed;
        velocity.z = direction.z * fighter.speed;

        // Jumping
        if keyboard.just_pressed(KeyCode::Space) && grounded.is_grounded {
            velocity.y = fighter.jump_force;
        }
    }
}

/// Handle player attack input
fn player_attack(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &Transform, &mut AnimationState),
        (With<PlayerController>, With<StickFighter>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, transform, mut anim_state) in query.iter_mut() {
        // Punch
        if keyboard.just_pressed(KeyCode::KeyJ) && *anim_state != AnimationState::Punching {
            *anim_state = AnimationState::Punching;

            // Spawn hitbox
            let hitbox_pos = transform.translation + transform.forward() * 0.8;
            commands.spawn((
                AttackHitbox {
                    damage: 10.0,
                    lifetime: Timer::from_seconds(0.2, TimerMode::Once),
                },
                Mesh3d(meshes.add(Sphere::new(0.3))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(1.0, 0.0, 0.0, 0.3),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                Transform::from_translation(hitbox_pos),
                Collider::sphere(0.3),
                Sensor,
            ));
        }

        // Kick
        if keyboard.just_pressed(KeyCode::KeyK) && *anim_state != AnimationState::Kicking {
            *anim_state = AnimationState::Kicking;

            // Spawn hitbox
            let hitbox_pos = transform.translation + transform.forward() * 1.0 + Vec3::new(0.0, -0.3, 0.0);
            commands.spawn((
                AttackHitbox {
                    damage: 15.0,
                    lifetime: Timer::from_seconds(0.25, TimerMode::Once),
                },
                Mesh3d(meshes.add(Sphere::new(0.4))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(1.0, 0.5, 0.0, 0.3),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                Transform::from_translation(hitbox_pos),
                Collider::sphere(0.4),
                Sensor,
            ));
        }
    }
}

/// Check if the character is grounded
fn check_grounded(
    mut query: Query<(&Transform, &mut GroundedState, &LinearVelocity), With<StickFighter>>,
) {
    for (transform, mut grounded, velocity) in query.iter_mut() {
        // Simple ground check - if below a certain height and not moving up
        grounded.is_grounded = transform.translation.y <= 0.6 && velocity.y.abs() < 0.1;
    }
}

/// Update animation state based on movement
fn update_animation_state(
    mut query: Query<(&LinearVelocity, &GroundedState, &mut AnimationState), With<StickFighter>>,
) {
    for (velocity, grounded, mut anim_state) in query.iter_mut() {
        // Don't override attack animations
        if matches!(*anim_state, AnimationState::Punching | AnimationState::Kicking) {
            // Reset to idle after a short delay (this is simplified)
            // In a real game, you'd use a timer
            continue;
        }

        let horizontal_speed = (velocity.x.powi(2) + velocity.z.powi(2)).sqrt();

        if !grounded.is_grounded {
            if velocity.y > 0.1 {
                *anim_state = AnimationState::Jumping;
            } else {
                *anim_state = AnimationState::Falling;
            }
        } else if horizontal_speed > 0.1 {
            if horizontal_speed > 3.0 {
                *anim_state = AnimationState::Running;
            } else {
                *anim_state = AnimationState::Walking;
            }
        } else {
            *anim_state = AnimationState::Idle;
        }
    }
}

/// Despawn expired attack hitboxes
fn despawn_expired_hitboxes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AttackHitbox)>,
    time: Res<Time>,
) {
    for (entity, mut hitbox) in query.iter_mut() {
        hitbox.lifetime.tick(time.delta());
        if hitbox.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Apply damage when hitboxes hit fighters
fn apply_damage(
    hitbox_query: Query<(&Transform, &AttackHitbox)>,
    mut fighter_query: Query<(&Transform, &mut StickFighter), Without<AttackHitbox>>,
) {
    for (hitbox_transform, hitbox) in hitbox_query.iter() {
        for (fighter_transform, mut fighter) in fighter_query.iter_mut() {
            let distance = hitbox_transform.translation.distance(fighter_transform.translation);
            if distance < 1.0 {
                // Simple proximity-based hit detection
                fighter.health = (fighter.health - hitbox.damage).max(0.0);
                // In a real game, you'd want to prevent multiple hits from the same attack
            }
        }
    }
}
