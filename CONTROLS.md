# Stick Fighter Controls

## User Interface

### Health Bar (Top Left)
- **Label**: "PLAYER HEALTH" in white text
- **Bar**: 300px wide red health bar with white border
- **Updates**: Real-time based on current health
- **Color**: Red fill (0-100% width)
- **Position**: 20px from top-left corner

Your health is displayed as a percentage of the bar's width. When you take damage, the bar shrinks. Full health = full bar!

## Training Opponents

### Passive Dummy (Blue)
A **blue training dummy** spawns in front of you for basic practice:
- **Color**: Blue (you are red)
- **Position**: Center, 5 units in front
- **Health**: 100 HP with 20 HP/sec regeneration
- **Behavior**: Stands still, does not attack or guard
- **Knockdown**: Gets knocked down when hit hard or health < 20
- **Recovery**: Gets back up after 2 seconds on the ground
- **Reset**: Teleports back to spawn if knocked too far (>15 units)

Perfect for testing combos and damage values!

### AI Opponent (Orange)
An **orange AI opponent** that fights back for realistic practice:
- **Color**: Orange
- **Position**: 3 units to the right of blue dummy
- **Health**: 100 HP with 15 HP/sec regeneration
- **Behavior**: Attacks and guards when you're within 5 units
- **AI Actions**:
  - 40% chance to attack (punch/kick/slash randomly)
  - 30% chance to guard (blocks for 1 second)
  - 30% chance to idle
  - Makes decisions every 1.5 seconds
- **Attack Cooldown**: 2 seconds between attacks
- **Guard Cooldown**: 3 seconds between guards
- **Movement**: Does not walk, stays in place

Perfect for practicing defense, parrying, and real combat!

## Movement Controls

- **W / Up Arrow**: Move forward (away from camera)
- **S / Down Arrow**: Move backward (toward camera)
- **A / Left Arrow**: Move left
- **D / Right Arrow**: Move right
- **Space**: Jump (hold direction for directional jump)
- **Shift / C**: Duck (hold to stay ducked)

## Directional Jumping

Press **Space** while holding a direction key to jump in that direction:
- **Space + W**: Jump forward
- **Space + S**: Jump backward
- **Space + A**: Jump left
- **Space + D**: Jump right
- Diagonal jumps work too!

## Ground Attacks

When standing or moving on the ground:
- **J**: Punch (10 damage, fast)
- **K**: Kick (15 damage, medium)
- **L**: Slash (18 damage, slower but powerful)

## Aerial Attacks

While in the air:
- **J**: Jump Punch (12 damage)
- **K**: Jump Kick (18 damage)
- **L**: Jump Slash (20 damage - most powerful aerial)

## Ducking Attacks

While holding **Shift** or **C** to duck:
- **K**: Duck Kick (12 damage, low attack)
- **L**: Duck Slash (15 damage, sweeping low attack)

Note: You move slower while ducking (50% speed)

## Defensive Mechanics

### Blocking
- **B** (hold): Block incoming attacks
  - Reduces damage by 80% while active
  - Can only block while grounded
  - Cannot move or attack while blocking

### Parry & Counter
- **P**: Parry (precise timing required)
  - 200ms parry window
  - If timed correctly, automatically counters with 1.5x damage
  - Brief cooldown after use
  - High risk, high reward!

## Grappling System

### Grab & Throw
- **G**: Grapple nearby opponent (1.5 unit range)
  - Holds opponent for up to 1 second
  - Opponent cannot escape during grapple
  - Pulls them close to you

- **T**: Throw (while grappling)
  - 25 damage - one of the most powerful moves!
  - Launches opponent away
  - Ends the grapple

## Combat Mechanics

### Combo System
- Landing consecutive attacks builds combo count
- Combo window: 1 second between attacks
- Higher combos could unlock special moves (future feature)

### Attack States
The game tracks various states:
- **Action Timer**: Prevents spam attacks, ensures commitment
- **Grounded State**: Determines available moves
- **Block State**: Active blocking
- **Parry Window**: Brief counter opportunity
- **Grapple State**: Holding or being held

### Attack Properties

| Attack Type | Damage | Duration | Color |
|------------|--------|----------|-------|
| Punch | 10 | 0.20s | Red |
| Kick | 15 | 0.25s | Orange |
| Slash | 18 | 0.30s | Blue |
| Jump Punch | 12 | 0.25s | Light Red |
| Jump Kick | 18 | 0.30s | Orange |
| Jump Slash | 20 | 0.35s | Blue |
| Duck Kick | 12 | 0.30s | Dark Orange |
| Duck Slash | 15 | 0.30s | Purple |
| Throw | 25 | 0.40s | Magenta |
| Parry Counter | 1.5x incoming | 0.30s | Yellow |

## Character Stats

- **Health**: 100 HP
- **Max Health**: 100 HP
- **Speed**: 5.0 units/second (2.5 while ducking)
- **Jump Force**: 8.0 units (vertical)
- **Jump Forward Force**: 6.0 units (horizontal)

## Fighting Strategies

### Offensive Play
1. **Combo Chains**: Mix punches, kicks, and slashes for variety
2. **Aerial Pressure**: Jump attacks deal good damage
3. **Duck Mix-ups**: Duck attacks hit low and are harder to block
4. **Grapple Punish**: When opponent whiffs, go for grapple → throw

### Defensive Play
1. **Blocking**: Safe but limits movement
2. **Parrying**: Risky but rewards with powerful counter
3. **Evasion**: Jump away to create distance
4. **Duck Under**: Duck to avoid high attacks (future feature)

### Advanced Techniques
- **Jump Back + Attack**: Create space while attacking
- **Duck Kick**: Good for hitting blocking opponents low
- **Parry Timing**: Learn opponent patterns for perfect parries
- **Grapple Range**: Stay close to threaten grabs

## Animation States

The character tracks these animation states:
- **Movement**: Idle, Walking, Running, Jumping, Falling, Ducking
- **Attacks**: Punching, Kicking, Slashing
- **Aerial**: JumpKick, JumpPunch, JumpSlash
- **Duck**: DuckKick, DuckSlash
- **Defense**: Blocking, Parrying
- **Grapple**: Grappling, Throwing, BeingGrappled, BeingThrown
- **Reactions**: Hit, Stunned

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
- Grounded state detection for move restrictions

### Arena
- 20x20 unit ground plane (green)
- Static collider for ground
- Directional lighting with shadows
- Ambient lighting for visibility

### Hit Detection
- Proximity-based (1.0 unit range)
- Tracks hit entities to prevent double-hitting
- Visual hitbox feedback with semi-transparent spheres
- Different colors for different attack types

## Tips

1. **Don't spam attacks** - Each attack has recovery time
2. **Mix up your attacks** - Alternate between high and low
3. **Use the environment** - Jump attacks from above are powerful
4. **Master parrying** - It's the highest skill technique
5. **Block when unsure** - Better than taking full damage
6. **Grapple for big damage** - Throws hurt a lot!
7. **Watch your position** - Don't get cornered
8. **Learn timings** - Each attack has different speed/power

## Future Improvements

- Multiple AI opponents with different difficulty levels
- Combo special moves and finishers
- Stamina system for resource management
- Improved hit reactions and knockback
- Health bar UI
- Round system with wins/losses
- Character customization and colors
- Different fighting styles
- Stage hazards and interactive arenas
- Multiplayer versus mode
