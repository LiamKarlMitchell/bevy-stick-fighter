use bevy::prelude::*;

/// Marks an entity as a stick fighter character
#[derive(Component)]
pub struct StickFighter {
    pub speed: f32,
    pub jump_force: f32,
    pub health: f32,
    pub max_health: f32,
}

impl Default for StickFighter {
    fn default() -> Self {
        Self {
            speed: 5.0,
            jump_force: 8.0,
            health: 100.0,
            max_health: 100.0,
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
#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    #[default]
    Idle,
    Walking,
    Running,
    Jumping,
    Falling,
    Punching,
    Kicking,
    Blocking,
    Hit,
}

/// Attack hitbox component
#[derive(Component)]
pub struct AttackHitbox {
    pub damage: f32,
    pub lifetime: Timer,
}

/// Grounded state tracker
#[derive(Component, Default)]
pub struct GroundedState {
    pub is_grounded: bool,
}
