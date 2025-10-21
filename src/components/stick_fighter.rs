use bevy::prelude::*;

/// Marks an entity as a stick fighter character
#[derive(Component)]
pub struct StickFighter {
    pub speed: f32,
    pub jump_force: f32,
    pub jump_forward_force: f32,
    pub health: f32,
    pub max_health: f32,
    pub is_ducking: bool,
}

impl Default for StickFighter {
    fn default() -> Self {
        Self {
            speed: 5.0,
            jump_force: 8.0,
            jump_forward_force: 6.0,
            health: 100.0,
            max_health: 100.0,
            is_ducking: false,
        }
    }
}

/// Marks the root entity of a stick figure character
#[derive(Component)]
pub struct StickFigureRoot;

/// Body parts of the stick figure
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyPart {
    Head,
    Torso,
    LeftUpperArm,
    LeftLowerArm,
    RightUpperArm,
    RightLowerArm,
    LeftUpperLeg,
    LeftLowerLeg,
    RightUpperLeg,
    RightLowerLeg,
}

/// Player controller component
#[derive(Component)]
pub struct PlayerController {
    pub player_id: usize,
}

/// Current animation state
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnimationState {
    #[default]
    Idle,
    Walking,
    Running,
    Jumping,
    Falling,
    Ducking,
    // Ground attacks
    Punching,
    Kicking,
    Slashing,
    // Aerial attacks
    JumpKick,
    JumpPunch,
    JumpSlash,
    // Ducking attacks
    DuckKick,
    DuckSlash,
    // Defensive
    Blocking,
    Parrying,
    // Grapple
    Grappling,
    Throwing,
    BeingGrappled,
    BeingThrown,
    // Hit reactions
    Hit,
    Stunned,
}

/// Attack type for hitbox identification
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttackType {
    Punch,
    Kick,
    Slash,
    JumpKick,
    JumpPunch,
    JumpSlash,
    DuckKick,
    DuckSlash,
    Throw,
}

/// Attack hitbox component
#[derive(Component)]
pub struct AttackHitbox {
    pub damage: f32,
    pub lifetime: Timer,
    pub attack_type: AttackType,
    pub attacker: Entity,
    pub hit_entities: Vec<Entity>, // Track what we've already hit
}

/// Grounded state tracker
#[derive(Component, Default)]
pub struct GroundedState {
    pub is_grounded: bool,
}

/// Combat state tracker with timers
#[derive(Component)]
pub struct CombatState {
    pub action_timer: Timer,
    pub block_active: bool,
    pub parry_window: Timer,
    pub can_parry: bool,
    pub combo_count: u32,
    pub combo_timer: Timer,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            action_timer: Timer::from_seconds(0.0, TimerMode::Once),
            block_active: false,
            parry_window: Timer::from_seconds(0.2, TimerMode::Once), // 200ms parry window
            can_parry: true,
            combo_count: 0,
            combo_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

/// Grapple state component
#[derive(Component)]
pub struct GrappleState {
    pub is_grappling: bool,
    pub grappled_entity: Option<Entity>,
    pub grapple_timer: Timer,
}

impl Default for GrappleState {
    fn default() -> Self {
        Self {
            is_grappling: false,
            grappled_entity: None,
            grapple_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

/// Training dummy component - non-player opponent for practice
#[derive(Component)]
pub struct TrainingDummy {
    pub spawn_position: Vec3,
    pub health_regen_rate: f32,
}

impl Default for TrainingDummy {
    fn default() -> Self {
        Self {
            spawn_position: Vec3::new(0.0, 5.0, -5.0),
            health_regen_rate: 20.0, // HP per second
        }
    }
}

/// Knockdown state tracking
#[derive(Component)]
pub struct KnockdownState {
    pub is_knocked_down: bool,
    pub recovery_timer: Timer,
}

impl Default for KnockdownState {
    fn default() -> Self {
        Self {
            is_knocked_down: false,
            recovery_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

/// Marker component for health bar UI container
#[derive(Component)]
pub struct HealthBarUI;

/// Marker component for the health bar fill (the red/green part)
#[derive(Component)]
pub struct HealthBarFill;

/// AI opponent component - attacks and guards occasionally
#[derive(Component)]
pub struct AIOpponent {
    pub spawn_position: Vec3,
    pub health_regen_rate: f32,
    pub attack_cooldown: Timer,
    pub guard_cooldown: Timer,
    pub guard_duration: Timer,
    pub decision_timer: Timer,
}

impl Default for AIOpponent {
    fn default() -> Self {
        Self {
            spawn_position: Vec3::new(3.0, 5.0, -5.0), // To the right of training dummy
            health_regen_rate: 15.0, // HP per second (slightly slower than passive dummy)
            attack_cooldown: Timer::from_seconds(2.0, TimerMode::Once),
            guard_cooldown: Timer::from_seconds(3.0, TimerMode::Once),
            guard_duration: Timer::from_seconds(1.0, TimerMode::Once),
            decision_timer: Timer::from_seconds(1.5, TimerMode::Repeating), // Make decisions every 1.5s
        }
    }
}

/// Damage number component - floats up and fades out
#[derive(Component)]
pub struct DamageNumber {
    pub lifetime: Timer,
    pub velocity: Vec3,
    pub initial_y: f32,
}

impl Default for DamageNumber {
    fn default() -> Self {
        Self {
            lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            velocity: Vec3::new(0.0, 2.0, 0.0), // Drift upward at 2 units/sec
            initial_y: 0.0,
        }
    }
}

/// Hitbox glow effect component
#[derive(Component)]
pub struct HitboxGlow {
    pub glow_timer: Timer,
    pub original_color: Color,
}

impl Default for HitboxGlow {
    fn default() -> Self {
        Self {
            glow_timer: Timer::from_seconds(0.15, TimerMode::Once),
            original_color: Color::WHITE,
        }
    }
}

