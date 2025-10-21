# 3D Stick Fighter Implementation Guide

This document describes the implementation of the advanced 3D stick figure fighting game built with Bevy and Avian3D.

## Architecture Overview

### Components (`src/components/stick_fighter.rs`)

#### StickFighter
The main character component containing:
- `speed`: Movement speed (default: 5.0)
- `jump_force`: Vertical jump velocity (default: 8.0)
- `jump_forward_force`: Horizontal jump velocity (default: 6.0)
- `health`: Current health
- `max_health`: Maximum health (default: 100.0)
- `is_ducking`: Boolean tracking duck state

#### BodyPart
Enum identifying different parts of the stick figure:
- Head, Torso
- LeftUpperArm, LeftLowerArm, RightUpperArm, RightLowerArm
- LeftUpperLeg, LeftLowerLeg, RightUpperLeg, RightLowerLeg

#### AnimationState
Comprehensive state tracking for all actions:

**Movement States:**
- Idle, Walking, Running, Jumping, Falling, Ducking

**Ground Attack States:**
- Punching, Kicking, Slashing

**Aerial Attack States:**
- JumpKick, JumpPunch, JumpSlash

**Duck Attack States:**
- DuckKick, DuckSlash

**Defensive States:**
- Blocking, Parrying

**Grapple States:**
- Grappling, Throwing, BeingGrappled, BeingThrown

**Reaction States:**
- Hit, Stunned

#### AttackType
Enum for different attack types:
- Punch, Kick, Slash
- JumpKick, JumpPunch, JumpSlash
- DuckKick, DuckSlash
- Throw

#### AttackHitbox
Component for attack collision detection:
- `damage`: Amount of damage to deal
- `lifetime`: Timer for auto-despawn
- `attack_type`: Type of attack
- `attacker`: Entity that spawned the hitbox
- `hit_entities`: Vec of already-hit entities (prevents double-hitting)

#### CombatState
Tracks combat-related state and timers:
- `action_timer`: Cooldown between attacks
- `block_active`: Currently blocking flag
- `parry_window`: 200ms timing window for successful parry
- `can_parry`: Cooldown flag for parrying
- `combo_count`: Number of consecutive hits
- `combo_timer`: 1 second window to continue combo

#### GrappleState
Manages grappling interactions:
- `is_grappling`: Currently holding an opponent
- `grappled_entity`: Optional entity being held
- `grapple_timer`: 1 second maximum hold duration

#### TrainingDummy
Training opponent for practice:
- `spawn_position`: Vec3 position to respawn at
- `health_regen_rate`: HP regeneration per second (default: 20.0)

#### KnockdownState
Tracks knockdown and recovery:
- `is_knocked_down`: Currently knocked down flag
- `recovery_timer`: 2 second timer before getting back up

#### UI Components
- `HealthBarUI`: Marks the health bar container entity
- `HealthBarFill`: Marks the health bar fill element (updates width)

#### Other Components
- `StickFigureRoot`: Marks the root entity
- `PlayerController`: Identifies player-controlled characters
- `GroundedState`: Tracks if character is on ground

## Systems Overview

### Startup Systems

#### `setup_arena`
Creates the fighting environment:
1. Spawns a 20x20 ground plane with green material
2. Adds static physics collider to ground
3. Spawns directional light with shadows
4. Adds ambient lighting

#### `spawn_stick_fighter`
Procedurally generates a stick figure character:
1. Creates root entity with all necessary components:
   - StickFighter, PlayerController, AnimationState
   - CombatState, GrappleState, GroundedState
   - RigidBody::Dynamic for physics
   - Capsule collider for collision
   - LockedAxes to prevent tipping

2. Spawns 10 body part entities as children:
   - Head: Sphere mesh
   - Torso: Capsule mesh (vertical)
   - Arms: 4 capsule meshes (posed with rotation)
   - Legs: 4 capsule meshes (positioned below torso)

3. All parts use the same red material
4. Parts are parented to root for hierarchical transformation

#### `spawn_training_dummy`
Spawns a training opponent for practice:
1. Creates identical stick figure structure to player
2. Uses blue material instead of red
3. Spawns at position (0, 5, -5) - 5 units in front
4. Adds TrainingDummy and KnockdownState components
5. No PlayerController - won't respond to input
6. Stands still for combo practice and damage testing

#### `setup_health_bar_ui`
Creates the player health bar UI:
1. Spawns root UI node at top-left (20px, 20px)
2. Creates "PLAYER HEALTH" text label (white, 14px font)
3. Creates health bar container (300x20px) with:
   - White 2px border
   - Dark gray background (0.2, 0.2, 0.2)
4. Creates health bar fill (red 0.8, 0.2, 0.2):
   - Width: 100% initially
   - Updated dynamically by update_health_bar system
5. Uses Bevy UI flexbox layout

### Update Systems

#### `update_combat_timers`
Manages all combat-related timers:
- Ticks action_timer (attack cooldowns)
- Ticks parry_window (200ms parry timing)
- Ticks combo_timer (1s combo window)
- Resets combo count when timer expires
- Resets parry availability when window closes
- Ticks grapple_timer and releases grapple when finished

#### `player_movement`
Handles WASD/Arrow key movement:
- Reads keyboard input for directional movement
- Blocks movement during attacks, blocking, or grappling
- Normalizes direction vector for consistent diagonal speed
- Applies 50% speed reduction while ducking
- Updates LinearVelocity component

#### `player_duck`
Manages ducking state:
- Activates when holding Shift or C
- Only works when grounded and not in action
- Automatically deactivates when jumping or attacking
- Reduces movement speed to 50%

#### `player_jump`
Handles jumping with directional control:
- Applies vertical jump force (8.0 units)
- Detects held direction keys during jump
- Applies horizontal force (6.0 units) in jump direction
- Supports: forward, backward, left, right, and diagonal jumps
- Only works when grounded and not ducking/attacking

#### `player_block`
Manages blocking state:
- Activates when holding B key
- Only works when grounded and not in action
- Sets block_active flag
- Changes animation to Blocking
- Automatically deactivates when key released

#### `player_parry`
Handles parry timing mechanic:
- Activates when pressing P key
- Only works when grounded and parry is available
- Creates 200ms parry window
- Sets 500ms cooldown timer
- Changes animation to Parrying
- High risk, high reward mechanic

#### `spawn_hitbox` (helper function)
Creates attack hitbox entities:
- Takes attacker entity, attack type, position, and properties
- Spawns sphere collider marked as Sensor
- Adds semi-transparent visual mesh
- Initializes hit tracking (prevents double-hitting)
- Auto-despawns after lifetime expires

#### `player_attack`
Comprehensive attack system with state-based attacks:

**Aerial Attacks** (when not grounded):
- Jump Kick (K): 18 damage, 0.3s duration
- Jump Punch (J): 12 damage, 0.25s duration
- Jump Slash (L): 20 damage, 0.35s duration

**Duck Attacks** (while ducking):
- Duck Kick (K): 12 damage, 0.3s duration, lower position
- Duck Slash (L): 15 damage, 0.3s duration

**Ground Attacks** (default):
- Punch (J): 10 damage, 0.2s duration
- Kick (K): 15 damage, 0.25s duration
- Slash (L): 18 damage, 0.3s duration

All ground attacks increment combo counter and reset combo timer.

#### `player_grapple`
Manages grappling and throwing:

**Grapple (G key):**
- Searches for nearby fighters (1.5 unit range)
- Initiates grapple with closest opponent
- Sets both entities' animation states
- Starts 1 second grapple timer

**Throw (T key, while grappling):**
- Spawns high-damage hitbox (25 damage)
- Updates animations for both entities
- Releases grapple immediately
- Throws opponent away

#### `check_grounded`
Simple ground detection:
- Character is grounded if Y position ≤ 0.6 and vertical velocity ≈ 0
- More sophisticated detection could use raycasting
- Updates GroundedState component

#### `update_animation_state`
State machine for animations:
1. Preserves active combat states (attacks, blocks, grapples)
2. Checks ducking state (when grounded)
3. Checks airborne state → Jumping or Falling
4. Checks horizontal movement → Walking or Running
5. Default → Idle

Only updates when action timer is finished and not blocking.

#### `despawn_expired_hitboxes`
Cleanup system:
- Ticks all AttackHitbox lifetime timers
- Despawns hitboxes when lifetime finishes
- Prevents entity buildup

#### `apply_damage`
Hit detection and damage application:
- Checks distance between hitboxes and fighters
- Prevents self-damage (attacker can't hit self)
- Prevents double-hitting (tracks hit entities)
- Proximity threshold: 1.0 unit
- Blocking reduces damage to 20%
- Full damage when not blocking
- Sets Hit animation state on damaged fighter

#### `handle_parry_counter`
Parry timing and auto-counter:
- Checks if fighter is in Parrying state
- Verifies parry window is active (not finished)
- Detects incoming hitboxes within range
- On successful parry:
  - Spawns counter attack hitbox
  - Counter deals 1.5x the incoming damage
  - Yellow visual feedback
  - Resets parry window

#### `update_grapple_state`
Manages grapple positioning:
- Pulls grappled entity close to grappler
- Maintains 1.0 unit distance
- Sets grappled entity velocity to zero
- Only active during grapple state

#### `training_dummy_health_regen`
Regenerates training dummy health:
- Adds health_regen_rate HP per second
- Default: 20 HP/sec
- Caps at max_health (100)
- Allows continuous practice without killing the dummy

#### `training_dummy_knockdown_check`
Detects when dummy should be knocked down:
- Triggers on high velocity (>5.0 units/sec) - hit hard
- Triggers on low health (<20 HP)
- Triggers when airborne (y > 3.0) - thrown up
- Sets is_knocked_down flag
- Starts 2 second recovery timer

#### `training_dummy_recovery`
Handles getting back up:
- Ticks recovery timer while knocked down
- Must be grounded to recover
- After 2 seconds, stands back up
- Resets animation to Idle
- Zeros velocity to prevent sliding

#### `training_dummy_position_reset`
Prevents dummy from getting lost:
- Checks distance from spawn_position
- If >15 units away, teleport back to spawn
- Resets velocity and knockdown state
- Ensures dummy stays in practice area

#### `update_health_bar`
Updates the health bar UI in real-time:
- Queries player StickFighter for current health
- Calculates health percentage (health / max_health * 100)
- Updates HealthBarFill width to match percentage
- Clamps between 0-100%
- Visual feedback: Full bar = 100 HP, Empty bar = 0 HP

## Combat Mechanics Deep Dive

### State Management
The game uses multiple timers to manage combat flow:

1. **Action Timer**: Prevents attack spam, creates commitment
2. **Parry Window**: 200ms precision timing for counters
3. **Combo Timer**: 1 second window to continue combos
4. **Grapple Timer**: 1 second maximum hold duration

### Attack Priority System
Attacks are contextual based on character state:
1. Check if airborne → Aerial attacks only
2. Check if ducking → Duck attacks only
3. Default → Ground attacks

### Combo System
- Each successful ground attack increments combo_count
- Combo timer resets to 1 second on each hit
- If 1 second passes without attack, combo resets to 0
- Future feature: Special moves at certain combo thresholds

### Defensive Mechanics

**Blocking:**
- Always available when grounded
- Reduces damage to 20%
- Cannot move or attack while blocking
- Safe but passive

**Parrying:**
- Requires precise timing (200ms window)
- Automatic counter on success
- Cooldown prevents spam
- High skill ceiling

### Grappling Mechanics
- Range check: 1.5 units
- Pull effect: Keeps opponent at 1.0 unit distance
- Duration: 1 second maximum
- Throw option: 25 damage (highest single attack)
- Risk: Locks both players briefly

## Physics Integration

### Avian3D Configuration
Located in `src/third_party/avian3d.rs`:
- Uses default PhysicsPlugins
- Length unit: 20.0 (scaling factor)

### Physics Resources
Located in `src/plugins/physics.rs`:
- Gravity: Vec3::new(0.0, -9.81, 0.0) - realistic gravity
- Applied globally to all dynamic bodies

### Character Physics
- Dynamic rigidbody responds to forces
- Capsule collider for smooth movement
- Locked rotation prevents character from falling over
- Linear velocity directly controlled for responsive movement
- Directional jumping applies horizontal impulse

## 3D Rendering

### Camera Setup (`src/plugins/camera.rs`)
- Camera3d component (upgraded from 2D)
- Positioned at (0, 5, 10) looking at character
- Looking at Vec3::new(0, 2, 0) with Y-up orientation

### Lighting
- Directional light from (4, 8, 4) with shadows
- Ambient light (white, 200 brightness)
- Both necessary for 3D visibility

### Materials
- StandardMaterial with solid colors
- Red for character (0.8, 0.2, 0.2)
- Green for ground (0.3, 0.5, 0.3)
- Color-coded semi-transparent hitboxes:
  - Red: Punches
  - Orange: Kicks
  - Blue: Slashes
  - Light Red: Jump punches
  - Purple: Duck attacks
  - Magenta: Throws
  - Yellow: Parry counters

## Attack Properties Reference

| Attack | Key | Damage | Duration | Offset | Size | Context |
|--------|-----|--------|----------|--------|------|---------|
| Punch | J | 10 | 0.20s | (0, 0.2, 0.8) | 0.3 | Ground |
| Kick | K | 15 | 0.25s | (0, -0.3, 1.0) | 0.4 | Ground |
| Slash | L | 18 | 0.30s | (0, 0, 1.0) | 0.45 | Ground |
| Jump Punch | J | 12 | 0.25s | (0, 0, 0.9) | 0.35 | Aerial |
| Jump Kick | K | 18 | 0.30s | (0, -0.5, 1.0) | 0.4 | Aerial |
| Jump Slash | L | 20 | 0.35s | (0, 0, 1.2) | 0.5 | Aerial |
| Duck Kick | K | 12 | 0.30s | (0, -0.7, 1.0) | 0.35 | Duck |
| Duck Slash | L | 15 | 0.30s | (0, -0.5, 1.1) | 0.45 | Duck |
| Throw | T | 25 | 0.40s | (0, 0, 2.0) | 0.5 | Grapple |

## Code Structure

```
src/
├── components/
│   └── stick_fighter.rs    # All fighter-related components
│                            # - StickFighter, AnimationState, AttackType
│                            # - CombatState, GrappleState
├── plugins/
│   ├── game.rs              # Main game logic (900+ lines)
│   │                        # - Arena setup, fighter spawning
│   │                        # - All combat systems
│   │                        # - Movement, attacks, grappling
│   ├── camera.rs            # 3D camera setup
│   ├── physics.rs           # Physics configuration
│   └── ...
├── third_party/
│   └── avian3d.rs          # Physics engine integration
└── lib.rs                   # Plugin registration
```

## Key Design Patterns

### Component Composition
Each fighter has multiple components that work together:
- StickFighter (stats)
- CombatState (timers and flags)
- GrappleState (grappling data)
- AnimationState (current action)
- GroundedState (environmental state)

### Timer-Based Actions
All attacks and states use Timer components:
- Provides consistent timing
- Easy to tune and balance
- Prevents infinite states

### Hit Tracking
AttackHitbox tracks hit entities:
- Prevents double-hitting
- Allows hitbox to persist
- Creates active hitbox frames

### State Prioritization
Animation system uses clear priority:
1. Active combat actions (highest)
2. Environmental states (ducking, airborne)
3. Movement states (walking, running)
4. Idle (lowest)

## Performance Considerations

- Body parts are visual only (no individual colliders)
- Single collider on root for all hit detection
- Hitboxes auto-despawn to prevent entity buildup
- Materials are cloned but stored in Assets for sharing
- Efficient query filters using With/Without
- Systems only process relevant entities

## Extension Points

### Adding New Attacks
1. Add to AttackType enum
2. Add to AnimationState if needed
3. Add input detection in player_attack
4. Call spawn_hitbox with appropriate parameters

### Adding AI Opponents
1. Create AIController component
2. Add decision-making system
3. Use same attack/movement systems with AI input
4. Remove PlayerController from queries for AI-only entities

### Adding Special Moves
1. Check combo_count in player_attack
2. Add new AttackType variants
3. Spawn multiple hitboxes or special effects
4. Reset combo_count after special move

### Adding Stamina System
1. Add stamina field to StickFighter
2. Deduct stamina on actions
3. Regenerate over time
4. Block actions when stamina too low

## Known Limitations & Future Work

1. **Visual Animations**: States tracked but limbs don't animate
2. **Hit Reactions**: No knockback or hitstun
3. **Sound Effects**: No audio system
4. **UI**: No health bars, combo counter, or round timer
5. **Camera**: Fixed position, could follow action
6. **AI**: No opponent AI yet
7. **Networking**: Single-player only
8. **Balancing**: Damage values need playtesting

## Balancing Philosophy

**Damage Tiers:**
- Light (10-12): Fast, safe, combo starters
- Medium (15-18): Slower, more commitment
- Heavy (20-25): High risk, high reward

**Speed Tiers:**
- Fast (0.2s): Quick recovery, can be blocked
- Medium (0.25-0.3s): Standard attacks
- Slow (0.35-0.4s): Long recovery, big punish window

**Risk/Reward:**
- Block: Safe but passive (20% damage)
- Parry: Risky but powerful counter (150% damage)
- Grapple: High damage but locks both players

## Testing Recommendations

1. Verify all attack types spawn hitboxes
2. Test combo system timing
3. Verify parry window (200ms)
4. Test grapple range and pull effect
5. Verify blocking reduces damage correctly
6. Test directional jumps in all directions
7. Verify duck attacks have lower hitbox position
8. Test action timer prevents spam
9. Verify hit tracking prevents double-hits
10. Test all animation state transitions
