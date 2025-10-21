use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::stick_fighter::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_arena, spawn_stick_fighter))
        .add_systems(
            Update,
            (
                update_combat_timers,
                player_movement,
                player_duck,
                player_jump,
                player_block,
                player_parry,
                player_attack,
                player_grapple,
                check_grounded,
                update_animation_state,
                despawn_expired_hitboxes,
                apply_damage,
                handle_parry_counter,
                update_grapple_state,
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
            CombatState::default(),
            GrappleState::default(),
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

/// Update all combat-related timers
fn update_combat_timers(
    mut query: Query<&mut CombatState>,
    mut grapple_query: Query<&mut GrappleState>,
    time: Res<Time>,
) {
    for mut combat_state in query.iter_mut() {
        combat_state.action_timer.tick(time.delta());
        combat_state.parry_window.tick(time.delta());
        combat_state.combo_timer.tick(time.delta());

        // Reset combo if timer expires
        if combat_state.combo_timer.finished() {
            combat_state.combo_count = 0;
        }

        // Parry window expired
        if combat_state.parry_window.finished() && !combat_state.can_parry {
            combat_state.can_parry = true;
        }
    }

    for mut grapple_state in grapple_query.iter_mut() {
        grapple_state.grapple_timer.tick(time.delta());

        // Release grapple after timer
        if grapple_state.grapple_timer.finished() && grapple_state.is_grappling {
            grapple_state.is_grappling = false;
            grapple_state.grappled_entity = None;
        }
    }
}

/// Handle player movement input
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (&StickFighter, &mut LinearVelocity, &GroundedState, &CombatState, &GrappleState),
        With<PlayerController>,
    >,
) {
    for (fighter, mut velocity, grounded, combat_state, grapple_state) in query.iter_mut() {
        // Can't move while attacking, blocking, or grappling
        if !combat_state.action_timer.finished()
            || combat_state.block_active
            || grapple_state.is_grappling {
            continue;
        }

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

        // Reduce speed when ducking
        let speed_modifier = if fighter.is_ducking { 0.5 } else { 1.0 };

        // Apply horizontal velocity
        velocity.x = direction.x * fighter.speed * speed_modifier;
        velocity.z = direction.z * fighter.speed * speed_modifier;
    }
}

/// Handle ducking
fn player_duck(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut StickFighter, &GroundedState, &CombatState), With<PlayerController>>,
) {
    for (mut fighter, grounded, combat_state) in query.iter_mut() {
        // Can only duck when grounded and not in action
        if !grounded.is_grounded || !combat_state.action_timer.finished() {
            fighter.is_ducking = false;
            continue;
        }

        // Hold Shift or C to duck
        fighter.is_ducking = keyboard.pressed(KeyCode::ShiftLeft)
            || keyboard.pressed(KeyCode::ShiftRight)
            || keyboard.pressed(KeyCode::KeyC);
    }
}

/// Handle jumping with directional control
fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (&StickFighter, &mut LinearVelocity, &GroundedState, &CombatState),
        With<PlayerController>,
    >,
) {
    for (fighter, mut velocity, grounded, combat_state) in query.iter_mut() {
        if keyboard.just_pressed(KeyCode::Space)
            && grounded.is_grounded
            && combat_state.action_timer.finished()
            && !fighter.is_ducking {

            // Vertical jump force
            velocity.y = fighter.jump_force;

            // Directional jumps
            if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
                // Jump forward
                velocity.z -= fighter.jump_forward_force;
            } else if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
                // Jump backward
                velocity.z += fighter.jump_forward_force;
            }

            if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
                velocity.x -= fighter.jump_forward_force;
            } else if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
                velocity.x += fighter.jump_forward_force;
            }
        }
    }
}

/// Handle blocking
fn player_block(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut CombatState, &mut AnimationState, &GroundedState), With<PlayerController>>,
) {
    for (mut combat_state, mut anim_state, grounded) in query.iter_mut() {
        // Hold B to block (only when grounded and not in action)
        if keyboard.pressed(KeyCode::KeyB)
            && grounded.is_grounded
            && combat_state.action_timer.finished() {
            combat_state.block_active = true;
            *anim_state = AnimationState::Blocking;
        } else if combat_state.block_active {
            combat_state.block_active = false;
        }
    }
}

/// Handle parry timing
fn player_parry(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut CombatState, &mut AnimationState, &GroundedState), With<PlayerController>>,
) {
    for (mut combat_state, mut anim_state, grounded) in query.iter_mut() {
        // Press P to parry (timing window)
        if keyboard.just_pressed(KeyCode::KeyP)
            && grounded.is_grounded
            && combat_state.can_parry
            && combat_state.action_timer.finished() {

            *anim_state = AnimationState::Parrying;
            combat_state.parry_window = Timer::from_seconds(0.2, TimerMode::Once);
            combat_state.can_parry = false;
            combat_state.action_timer = Timer::from_seconds(0.5, TimerMode::Once);
        }
    }
}

/// Spawn an attack hitbox
fn spawn_hitbox(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    attacker: Entity,
    attack_type: AttackType,
    transform: &Transform,
    offset: Vec3,
    size: f32,
    damage: f32,
    lifetime: f32,
    color: Color,
) {
    let hitbox_pos = transform.translation + transform.rotation * offset;

    commands.spawn((
        AttackHitbox {
            damage,
            lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
            attack_type,
            attacker,
            hit_entities: Vec::new(),
        },
        Mesh3d(meshes.add(Sphere::new(size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_translation(hitbox_pos),
        Collider::sphere(size),
        Sensor,
    ));
}

/// Handle player attack input
fn player_attack(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &Transform, &mut AnimationState, &mut CombatState, &StickFighter, &GroundedState),
        (With<PlayerController>, With<StickFighter>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, transform, mut anim_state, mut combat_state, fighter, grounded) in query.iter_mut() {
        // Can't attack while in action or blocking
        if !combat_state.action_timer.finished() || combat_state.block_active {
            continue;
        }

        let is_airborne = !grounded.is_grounded;
        let is_ducking = fighter.is_ducking;

        // AERIAL ATTACKS
        if is_airborne {
            // Jump Kick - K
            if keyboard.just_pressed(KeyCode::KeyK) {
                *anim_state = AnimationState::JumpKick;
                combat_state.action_timer = Timer::from_seconds(0.3, TimerMode::Once);
                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::JumpKick, transform,
                    Vec3::new(0.0, -0.5, 1.0), 0.4, 18.0, 0.3,
                    Color::srgba(1.0, 0.5, 0.0, 0.4),
                );
            }
            // Jump Punch - J
            else if keyboard.just_pressed(KeyCode::KeyJ) {
                *anim_state = AnimationState::JumpPunch;
                combat_state.action_timer = Timer::from_seconds(0.25, TimerMode::Once);
                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::JumpPunch, transform,
                    Vec3::new(0.0, 0.0, 0.9), 0.35, 12.0, 0.25,
                    Color::srgba(1.0, 0.2, 0.2, 0.4),
                );
            }
            // Jump Slash - L
            else if keyboard.just_pressed(KeyCode::KeyL) {
                *anim_state = AnimationState::JumpSlash;
                combat_state.action_timer = Timer::from_seconds(0.35, TimerMode::Once);
                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::JumpSlash, transform,
                    Vec3::new(0.0, 0.0, 1.2), 0.5, 20.0, 0.35,
                    Color::srgba(0.5, 0.5, 1.0, 0.4),
                );
            }
        }
        // DUCKING ATTACKS
        else if is_ducking {
            // Duck Kick - K
            if keyboard.just_pressed(KeyCode::KeyK) {
                *anim_state = AnimationState::DuckKick;
                combat_state.action_timer = Timer::from_seconds(0.3, TimerMode::Once);
                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::DuckKick, transform,
                    Vec3::new(0.0, -0.7, 1.0), 0.35, 12.0, 0.3,
                    Color::srgba(0.8, 0.4, 0.0, 0.4),
                );
            }
            // Duck Slash - L
            else if keyboard.just_pressed(KeyCode::KeyL) {
                *anim_state = AnimationState::DuckSlash;
                combat_state.action_timer = Timer::from_seconds(0.3, TimerMode::Once);
                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::DuckSlash, transform,
                    Vec3::new(0.0, -0.5, 1.1), 0.45, 15.0, 0.3,
                    Color::srgba(0.6, 0.2, 0.8, 0.4),
                );
            }
        }
        // GROUND ATTACKS
        else {
            // Punch - J
            if keyboard.just_pressed(KeyCode::KeyJ) {
                *anim_state = AnimationState::Punching;
                combat_state.action_timer = Timer::from_seconds(0.2, TimerMode::Once);
                combat_state.combo_count += 1;
                combat_state.combo_timer = Timer::from_seconds(1.0, TimerMode::Once);

                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::Punch, transform,
                    Vec3::new(0.0, 0.2, 0.8), 0.3, 10.0, 0.2,
                    Color::srgba(1.0, 0.0, 0.0, 0.3),
                );
            }
            // Kick - K
            else if keyboard.just_pressed(KeyCode::KeyK) {
                *anim_state = AnimationState::Kicking;
                combat_state.action_timer = Timer::from_seconds(0.25, TimerMode::Once);
                combat_state.combo_count += 1;
                combat_state.combo_timer = Timer::from_seconds(1.0, TimerMode::Once);

                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::Kick, transform,
                    Vec3::new(0.0, -0.3, 1.0), 0.4, 15.0, 0.25,
                    Color::srgba(1.0, 0.5, 0.0, 0.3),
                );
            }
            // Slash - L
            else if keyboard.just_pressed(KeyCode::KeyL) {
                *anim_state = AnimationState::Slashing;
                combat_state.action_timer = Timer::from_seconds(0.3, TimerMode::Once);
                combat_state.combo_count += 1;
                combat_state.combo_timer = Timer::from_seconds(1.0, TimerMode::Once);

                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::Slash, transform,
                    Vec3::new(0.0, 0.0, 1.0), 0.45, 18.0, 0.3,
                    Color::srgba(0.5, 0.5, 1.0, 0.3),
                );
            }
        }
    }
}

/// Handle grapple and throw
fn player_grapple(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &Transform, &mut AnimationState, &mut GrappleState, &CombatState),
        With<PlayerController>,
    >,
    mut target_query: Query<
        (Entity, &Transform, &mut AnimationState),
        (With<StickFighter>, Without<PlayerController>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, transform, mut anim_state, mut grapple_state, combat_state) in query.iter_mut() {
        // Press G to initiate grapple
        if keyboard.just_pressed(KeyCode::KeyG) && !grapple_state.is_grappling && combat_state.action_timer.finished() {
            // Find nearby enemy to grapple
            for (target_entity, target_transform, mut target_anim) in target_query.iter_mut() {
                let distance = transform.translation.distance(target_transform.translation);

                if distance < 1.5 {
                    // Initiate grapple
                    grapple_state.is_grappling = true;
                    grapple_state.grappled_entity = Some(target_entity);
                    grapple_state.grapple_timer = Timer::from_seconds(1.0, TimerMode::Once);
                    *anim_state = AnimationState::Grappling;
                    *target_anim = AnimationState::BeingGrappled;
                    break;
                }
            }
        }

        // Press T to throw (while grappling)
        if keyboard.just_pressed(KeyCode::KeyT) && grapple_state.is_grappling {
            *anim_state = AnimationState::Throwing;

            if let Some(target_entity) = grapple_state.grappled_entity {
                // Spawn throw hitbox
                spawn_hitbox(
                    &mut commands, &mut meshes, &mut materials,
                    entity, AttackType::Throw, transform,
                    Vec3::new(0.0, 0.0, 2.0), 0.5, 25.0, 0.4,
                    Color::srgba(1.0, 0.0, 1.0, 0.4),
                );

                // Update target animation
                if let Ok((_, _, mut target_anim)) = target_query.get_mut(target_entity) {
                    *target_anim = AnimationState::BeingThrown;
                }
            }

            // Release grapple
            grapple_state.is_grappling = false;
            grapple_state.grappled_entity = None;
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
    mut query: Query<
        (&LinearVelocity, &GroundedState, &mut AnimationState, &CombatState, &StickFighter),
        With<StickFighter>,
    >,
) {
    for (velocity, grounded, mut anim_state, combat_state, fighter) in query.iter_mut() {
        // Don't override attack, block, grapple, or parry animations
        if !combat_state.action_timer.finished()
            || combat_state.block_active
            || matches!(
                *anim_state,
                AnimationState::Punching
                    | AnimationState::Kicking
                    | AnimationState::Slashing
                    | AnimationState::JumpKick
                    | AnimationState::JumpPunch
                    | AnimationState::JumpSlash
                    | AnimationState::DuckKick
                    | AnimationState::DuckSlash
                    | AnimationState::Blocking
                    | AnimationState::Parrying
                    | AnimationState::Grappling
                    | AnimationState::Throwing
                    | AnimationState::BeingGrappled
                    | AnimationState::BeingThrown
            ) {
            return;
        }

        let horizontal_speed = (velocity.x.powi(2) + velocity.z.powi(2)).sqrt();

        // Check ducking first
        if fighter.is_ducking && grounded.is_grounded {
            *anim_state = AnimationState::Ducking;
        }
        // Check airborne
        else if !grounded.is_grounded {
            if velocity.y > 0.1 {
                *anim_state = AnimationState::Jumping;
            } else {
                *anim_state = AnimationState::Falling;
            }
        }
        // Check movement
        else if horizontal_speed > 0.1 {
            if horizontal_speed > 3.0 {
                *anim_state = AnimationState::Running;
            } else {
                *anim_state = AnimationState::Walking;
            }
        }
        // Default to idle
        else {
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
    mut hitbox_query: Query<(&Transform, &mut AttackHitbox)>,
    mut fighter_query: Query<
        (Entity, &Transform, &mut StickFighter, &mut AnimationState, &CombatState),
        Without<AttackHitbox>,
    >,
) {
    for (hitbox_transform, mut hitbox) in hitbox_query.iter_mut() {
        for (fighter_entity, fighter_transform, mut fighter, mut anim_state, combat_state) in fighter_query.iter_mut() {
            // Don't hit yourself
            if fighter_entity == hitbox.attacker {
                continue;
            }

            // Already hit this entity
            if hitbox.hit_entities.contains(&fighter_entity) {
                continue;
            }

            let distance = hitbox_transform.translation.distance(fighter_transform.translation);

            if distance < 1.0 {
                // Check if blocking
                if combat_state.block_active {
                    // Blocked! Reduce damage significantly
                    fighter.health = (fighter.health - hitbox.damage * 0.2).max(0.0);
                } else {
                    // Full damage
                    fighter.health = (fighter.health - hitbox.damage).max(0.0);
                    *anim_state = AnimationState::Hit;
                }

                // Mark this entity as hit
                hitbox.hit_entities.push(fighter_entity);
            }
        }
    }
}

/// Handle parry timing and counter attacks
fn handle_parry_counter(
    mut commands: Commands,
    hitbox_query: Query<(&Transform, &AttackHitbox)>,
    mut fighter_query: Query<
        (Entity, &Transform, &mut CombatState, &AnimationState),
        With<StickFighter>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (hitbox_transform, hitbox) in hitbox_query.iter() {
        for (fighter_entity, fighter_transform, mut combat_state, anim_state) in fighter_query.iter_mut() {
            // Don't parry yourself
            if fighter_entity == hitbox.attacker {
                continue;
            }

            // Check if in parry window
            if *anim_state == AnimationState::Parrying && !combat_state.parry_window.finished() {
                let distance = hitbox_transform.translation.distance(fighter_transform.translation);

                if distance < 1.2 {
                    // Successful parry! Spawn counter attack
                    spawn_hitbox(
                        &mut commands, &mut meshes, &mut materials,
                        fighter_entity, AttackType::Punch, fighter_transform,
                        Vec3::new(0.0, 0.0, 1.0), 0.4, hitbox.damage * 1.5, 0.3,
                        Color::srgba(1.0, 1.0, 0.0, 0.5),
                    );

                    // Reset parry cooldown
                    combat_state.parry_window.reset();
                }
            }
        }
    }
}

/// Update grapple positioning
fn update_grapple_state(
    mut grappler_query: Query<(&Transform, &GrappleState), With<PlayerController>>,
    mut target_query: Query<
        (Entity, &mut Transform, &mut LinearVelocity),
        (With<StickFighter>, Without<PlayerController>),
    >,
) {
    for (grappler_transform, grapple_state) in grappler_query.iter() {
        if !grapple_state.is_grappling {
            continue;
        }

        if let Some(grappled_entity) = grapple_state.grappled_entity {
            if let Ok((_, mut target_transform, mut target_velocity)) = target_query.get_mut(grappled_entity) {
                // Pull grappled entity close
                let direction = (grappler_transform.translation - target_transform.translation).normalize();
                target_transform.translation = grappler_transform.translation - direction * 1.0;
                target_velocity.0 = Vec3::ZERO;
            }
        }
    }
}
