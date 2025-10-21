# 3D Stick Fighter Implementation Guide

This document describes the implementation of the 3D stick figure fighting game built with Bevy and Avian3D.

## Architecture Overview

### Components (`src/components/stick_fighter.rs`)

#### StickFighter
The main character component containing:
- `speed`: Movement speed (default: 5.0)
- `jump_force`: Jump velocity (default: 8.0)
- `health`: Current health
- `max_health`: Maximum health (default: 100.0)

#### BodyPart
Enum identifying different parts of the stick figure:
- Head, Torso
- LeftUpperArm, LeftLowerArm, RightUpperArm, RightLowerArm
- LeftUpperLeg, LeftLowerLeg, RightUpperLeg, RightLowerLeg

#### AnimationState
Tracks the current animation/action:
- Idle, Walking, Running
- Jumping, Falling
- Punching, Kicking
- Blocking, Hit

#### Other Components
- `StickFigureRoot`: Marks the root entity
- `PlayerController`: Identifies player-controlled characters
- `GroundedState`: Tracks if character is on ground
- `AttackHitbox`: Temporary hitbox for attacks with damage and lifetime

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
1. Creates root entity with physics components:
   - RigidBody::Dynamic for physics simulation
   - Capsule collider for collision detection
   - LockedAxes to prevent tipping over
   - LinearVelocity for movement

2. Spawns 10 body part entities as children:
   - Head: Sphere mesh
   - Torso: Capsule mesh (vertical)
   - Arms: 4 capsule meshes (posed with rotation)
   - Legs: 4 capsule meshes (positioned below torso)

3. All parts use the same red material
4. Parts are parented to root for hierarchical transformation

### Update Systems

#### `player_movement`
Handles WASD/Arrow key movement:
1. Reads keyboard input for directional movement
2. Normalizes direction vector for consistent diagonal speed
3. Applies velocity to character physics body
4. Checks Space for jumping (only when grounded)

#### `player_attack`
Handles J (punch) and K (kick) attacks:
1. Detects attack key presses
2. Updates animation state
3. Spawns temporary hitbox entity:
   - Positioned in front of/below character
   - Spherical collider marked as Sensor
   - Semi-transparent visual feedback
   - Timer for automatic despawn

#### `check_grounded`
Simple ground detection:
- Character is grounded if Y position ≤ 0.6 and vertical velocity ≈ 0
- More sophisticated detection could use raycasting

#### `update_animation_state`
State machine for animations:
1. Preserves attack states (don't interrupt punching/kicking)
2. Checks if airborne → Jumping or Falling
3. Checks horizontal movement → Walking or Running
4. Default → Idle

#### `despawn_expired_hitboxes`
Cleanup system:
- Ticks attack hitbox timers
- Despawns expired hitboxes

#### `apply_damage`
Simple damage application:
- Checks distance between hitboxes and fighters
- Applies damage if within range
- Note: Could be improved to prevent multiple hits per attack

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
- Semi-transparent red/orange for hitboxes (using AlphaMode::Blend)

## Code Structure

```
src/
├── components/
│   └── stick_fighter.rs    # All fighter-related components
├── plugins/
│   ├── game.rs              # Main game logic (arena, fighter, combat)
│   ├── camera.rs            # 3D camera setup
│   ├── physics.rs           # Physics configuration
│   └── ...
├── third_party/
│   └── avian3d.rs          # Physics engine integration
└── lib.rs                   # Plugin registration
```

## Migration from 2D to 3D

Changes made to convert the starter template:

1. **Dependencies** (Cargo.toml):
   - avian2d → avian3d
   - Updated feature flags

2. **Physics**:
   - Vec2 → Vec3 for gravity
   - 2D colliders → 3D colliders

3. **Camera**:
   - Camera2d → Camera3d
   - Added explicit positioning and look-at

4. **Rendering**:
   - Added Mesh3d and MeshMaterial3d components
   - Added lighting (directional + ambient)

## Extending the Game

### Adding More Fighters
To spawn multiple fighters:
```rust
fn spawn_stick_fighter(/* ... */) {
    for i in 0..2 {
        let position = Vec3::new(i as f32 * 3.0, 5.0, 0.0);
        // ... spawn fighter at position
        PlayerController { player_id: i }
    }
}
```

### Adding Animations
Currently animation states are tracked but not visually applied.
To implement:
1. Add rotation/position animation timers
2. Interpolate body part transforms based on AnimationState
3. Create keyframe animation system

### Improving Hit Detection
Current system uses simple distance check. Better approach:
1. Use Avian3D collision events
2. Track which attacks have already hit
3. Add ownership to hitboxes
4. Use CollisionLayers to prevent self-hitting

### Adding AI
To create AI opponents:
1. Create AIController component (instead of PlayerController)
2. Add decision-making system (attack when close, dodge, etc.)
3. Use navigation for movement instead of direct input

## Performance Considerations

- Body parts are visual only (no individual colliders)
- Single collider on root for all hit detection
- Hitboxes auto-despawn to prevent entity buildup
- Materials are cloned but stored in Assets for sharing

## Known Limitations

1. Attack animations don't actually move limbs
2. No timer for resetting attack states
3. Simple proximity hit detection (can hit multiple times)
4. No knockback or hitstun
5. Health doesn't affect character behavior
6. No UI for health display
7. Ground detection is position-based (not raycast)
