# Stick Fighter Controls

## Movement Controls

- **W / Up Arrow**: Move forward (away from camera)
- **S / Down Arrow**: Move backward (toward camera)
- **A / Left Arrow**: Move left
- **D / Right Arrow**: Move right
- **Space**: Jump

## Combat Controls

- **J**: Punch attack (10 damage, 0.2s duration)
- **K**: Kick attack (15 damage, 0.25s duration)

## Features

### Character Stats
- **Health**: 100 HP
- **Speed**: 5.0 units/second
- **Jump Force**: 8.0 units

### Fighting Mechanics
- Attack hitboxes spawn during attacks
- Proximity-based hit detection
- Different damage values for different attacks
- Visual feedback with semi-transparent attack spheres

### Animation States
The character tracks the following animation states:
- Idle (standing still)
- Walking (slow movement)
- Running (fast movement)
- Jumping (moving upward)
- Falling (moving downward)
- Punching (J key attack)
- Kicking (K key attack)
- Blocking (not yet implemented)
- Hit (not yet implemented)

## Technical Details

### 3D Character Structure
The stick figure is composed of:
- **Head**: Sphere (0.3 radius)
- **Torso**: Capsule (0.15 radius, 0.8 height)
- **Arms**: 4 capsules (upper/lower left/right, 0.08 radius, 0.4 height each)
- **Legs**: 4 capsules (upper/lower left/right, 0.1 radius, 0.5 height each)

### Physics
- Uses Avian3D physics engine
- Gravity: -9.81 m/s² (realistic)
- Character has locked rotation to prevent tipping
- Dynamic rigidbody with capsule collider
- Grounded state detection for jump control

### Arena
- 20x20 unit ground plane (green)
- Static collider for ground
- Directional lighting with shadows
- Ambient lighting for visibility

## Future Improvements
- Add AI opponents
- Implement combo system
- Add blocking mechanic
- Implement proper animation timers for attacks
- Add health bar UI
- Implement knockback on hits
- Add special moves
- Multiplayer support
- More sophisticated hit detection (instead of proximity)
- Ragdoll physics on knockout
