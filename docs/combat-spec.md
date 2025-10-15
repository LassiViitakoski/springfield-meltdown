# Combat System Specification

**Version:** 1.0 - Prototype MVP
**Date:** 2025-10-15
**Status:** Implementation Ready

---

## Overview

This document defines the concrete combat mechanics for Springfield Meltdown's prototype (Phase 1, Step 1-3). These specifications are implementation-ready and focused on proving the core combat loop feels fun.

**Design Philosophy:**
- Smooth, responsive controls
- Mouse-aimed shooting for precision
- Simple but satisfying combat feedback
- Iterate based on feel, not theory

---

## Movement System

### Coordinate System

**World Space (Logic):**
- 2D floating-point coordinates: `(x: f32, y: f32)`
- Conceptually top-down (like viewing from above)
- All game logic (collision, AI, movement) operates in world space
- Origin `(0, 0)` at top-left, positive X right, positive Y down

**Screen Space (Rendering):**
- Isometric projection transforms world coordinates for display
- Formula:
  ```
  screen_x = (world_x - world_y) * TILE_WIDTH_HALF
  screen_y = (world_x + world_y) * TILE_HEIGHT_HALF
  ```
- Suggested constants: `TILE_WIDTH_HALF = 32`, `TILE_HEIGHT_HALF = 16`
- Depth sorting: entities with higher `world_y` render in front

### Player Movement

**Input Scheme:**
- WASD or Arrow Keys for 8-directional movement
- No acceleration/deceleration (instant response for prototype)
- Diagonal movement normalized to prevent speed boost

**Implementation:**
```rust
// Pseudo-code for movement calculation
let mut velocity = Vec2::new(0.0, 0.0);

if key_pressed(W) { velocity.y -= 1.0; }
if key_pressed(S) { velocity.y += 1.0; }
if key_pressed(A) { velocity.x -= 1.0; }
if key_pressed(D) { velocity.x += 1.0; }

// Normalize to prevent diagonal speed boost
if velocity.length() > 0.0 {
    velocity = velocity.normalize();
}

// Apply movement
const PLAYER_SPEED: f32 = 150.0; // pixels per second
player.position += velocity * PLAYER_SPEED * delta_time;
```

**Parameters:**
- **Base Speed:** 150 pixels/second
- **Collision Radius:** 16 pixels (circle collider)
- **Bounds Checking:** Prevent leaving play area (define area boundaries during implementation)

### Camera

**Step 1 (MVP):**
- Fixed camera centered on player
- No smooth follow (can add later if needed)

**Future Enhancement:**
- Smooth camera lerp: `camera.pos = lerp(camera.pos, player.pos, 0.1)`

---

## Combat Controls

### Mouse Aim System

**Why Mouse Aim:**
- Decouples movement from shooting direction
- Allows precision targeting in isometric space
- Natural for PC gaming
- Good learning exercise (vector math, coordinate transforms)

**Implementation:**
1. Get mouse screen position from ggez
2. Convert screen position to world coordinates (inverse isometric transform)
3. Calculate aiming vector: `aim_direction = (mouse_world_pos - player_pos).normalize()`
4. On left mouse click: spawn projectile in `aim_direction`
5. Weapon cooldown prevents spam-clicking

**Fallback Plan:**
- If mouse aim feels awkward, switch to directional shooting (fire in movement direction)
- Simpler implementation, common in roguelites

---

## Weapons

### Weapon #1: Pistol

**Role:** Basic starting weapon, balanced and reliable

**Stats:**
| Property | Value | Notes |
|----------|-------|-------|
| Damage | 10 HP | Per projectile hit |
| Fire Rate | 0.3 seconds | ~3.3 shots/second |
| Projectile Speed | 400 px/sec | Fast, responsive |
| Range | Infinite | Despawn off-screen |
| Ammo | Unlimited | Economy added in Step 3 |

**Projectile Behavior:**
- Spawn at player position
- Move in straight line (aimed direction)
- Despawn on: enemy collision OR off-screen
- Collision: circle, radius 4px

**Visual:**
- Small yellow/orange circle sprite (8px diameter)
- Future: Add muzzle flash, tracer effect

**Audio (Optional):**
- "Pew" sound on fire
- Impact sound on hit

### Future Weapons (Step 3)

**Shotgun:**
- 3 projectiles in spread pattern
- High damage, short range
- Slow fire rate

**Rifle:**
- High damage single shot
- Slower fire rate than pistol
- Longer range/faster projectile

**Melee (Deferred):**
- Close-range sweep attack
- No ammo required
- Higher risk/reward

---

## Enemies

### Enemy #1: Radioactive Rat (Chaser)

**Role:** Basic melee enemy, teaches kiting and spacing

**Stats:**
| Property | Value | Notes |
|----------|-------|-------|
| HP | 30 | 3 pistol shots to kill |
| Movement Speed | 80 px/sec | Slower than player (can escape) |
| Damage | 5 HP | Per contact hit |
| Attack Cooldown | 1.0 second | Can't damage constantly |
| Collision Radius | 12 px | Slightly smaller than player |

**AI Behavior:**
```
Every frame:
1. Calculate vector: rat_position → player_position
2. Normalize vector
3. Move toward player: position += direction * speed * delta_time
4. Check collision with player
5. If colliding AND attack_cooldown == 0:
   - Deal damage to player
   - Reset attack_cooldown to 1.0 seconds
```

**Visual:**
- Green glowing rat sprite
- Placeholder: green circle with small triangular ears
- Flash white on taking damage (0.1 second)

**Death:**
- Fade out over 0.2 seconds
- Optional: small particle burst (green glow)

**Spawning (Step 1):**
- Spawn 5-10 rats at random positions around map edges
- Minimum distance from player: 200 pixels
- No respawning in Step 1 (fixed enemy count)

### Future Enemies (Step 3)

**Radioactive Dog (Fast Chaser):**
- Faster than rat (120 px/sec)
- Less HP (20)
- Hit-and-run attack pattern

**Mutated Citizen (Ranged):**
- Stays at distance
- Shoots projectiles at player
- Low HP, high threat

---

## Combat Feel (Polish - Step 2)

Once basic mechanics work, add game juice:

### Hit Feedback

**Screen Shake:**
- Trigger on: enemy death, player takes damage
- Intensity: 2-3 pixel offset
- Duration: 0.1 seconds
- Decay: ease-out

**Enemy Hit Flash:**
- Flash white sprite for 0.1 seconds on damage
- Return to normal color

**Particle Effects:**
- Small burst at projectile impact point
- 5-10 particles, fade out over 0.3 seconds
- Color: yellow/orange for hits, red for player damage

### Audio (Optional for Prototype)

**Essential Sounds:**
- Pistol shot
- Enemy hit/death
- Player damage

**Ambient (Low Priority):**
- Tense background music loop
- Footstep sounds

---

## Health System

### Player Health

**Stats:**
| Property | Value |
|----------|-------|
| Max HP | 100 |
| Starting HP | 100 |
| Regen | None (prototype) |

**Death State:**
- HP reaches 0 → freeze game
- Display "You Died" text
- Press R to restart scene
- No progression loss (prototype only)

**UI Display:**
- Top-left corner: `HP: 85/100`
- Red text when HP < 30%
- Optional: Health bar visual

### Enemy Health

**Per-Enemy:**
- HP value from enemy stats table
- No HP bar displayed (too cluttered for prototype)
- Death animation implies HP depleted

**Damage Calculation:**
- Simple subtraction (no armor/resistance in Step 1)
- `enemy.hp -= weapon.damage`

---

## Victory & Failure Conditions

### Step 1 Victory

**Trigger:** All enemies defeated (enemy_count == 0)

**Outcome:**
- Display "You Survived!" text (center screen)
- Press R to restart scene
- No rewards/progression yet (added in Step 4)

### Step 1 Failure

**Trigger:** Player HP reaches 0

**Outcome:**
- Display "You Died" text (center screen)
- Press R to restart scene
- No penalties (prototype testing mode)

---

## Implementation Order

Suggested step-by-step build sequence:

### Phase 1: Basic Setup
1. Empty world (black background, 800x600 window)
2. Player sprite at center, can move with WASD
3. Camera follows player (simple centered view)

### Phase 2: Shooting Mechanics
4. Draw line from player to mouse cursor (debug visualization)
5. Left click spawns projectile, moves toward mouse position
6. Projectile despawns when off-screen

### Phase 3: Basic Enemy
7. Spawn 1 rat at fixed position
8. Rat moves toward player (chase AI)
9. Collision detection: projectile hits rat → rat dies

### Phase 4: Combat Loop
10. Collision detection: rat touches player → player loses HP
11. Player HP display (text: "HP: 100")
12. Player death state (HP = 0 → game over)

### Phase 5: Multiple Enemies
13. Spawn 5-10 rats at random edge positions
14. Victory condition: all rats dead → "You Survived!"
15. Press R to restart scene

### Phase 6: Polish (Step 2)
16. Add hit flash effect on enemies
17. Add screen shake on damage
18. Tune speeds/damage values until fun
19. Add particle effects (optional)

---

## Tuning Parameters

Values to adjust during Step 2 iteration:

| Parameter | Initial | Adjust For |
|-----------|---------|------------|
| Player Speed | 150 px/s | Control responsiveness |
| Rat Speed | 80 px/s | Challenge difficulty |
| Pistol Fire Rate | 0.3s | Combat pacing |
| Pistol Damage | 10 HP | Time-to-kill feel |
| Rat HP | 30 HP | Bullet sponginess |
| Rat Damage | 5 HP | Punishment severity |

**Tuning Goals:**
- Player can escape rats if skilled
- Killing 1 rat feels satisfying (not too easy/hard)
- Getting hit feels punishing but not instant death
- Combat pacing feels tense but not overwhelming

---

## Technical Notes

### Coordinate Transform Helpers

**Screen to World (for mouse input):**
```rust
// Inverse isometric projection
fn screen_to_world(screen_x: f32, screen_y: f32) -> (f32, f32) {
    let world_x = (screen_x / TILE_WIDTH_HALF + screen_y / TILE_HEIGHT_HALF) / 2.0;
    let world_y = (screen_y / TILE_HEIGHT_HALF - screen_x / TILE_WIDTH_HALF) / 2.0;
    (world_x, world_y)
}
```

**World to Screen (for rendering):**
```rust
fn world_to_screen(world_x: f32, world_y: f32) -> (f32, f32) {
    let screen_x = (world_x - world_y) * TILE_WIDTH_HALF;
    let screen_y = (world_x + world_y) * TILE_HEIGHT_HALF;
    (screen_x, screen_y)
}
```

### Collision Detection

**Circle-Circle Collision:**
```rust
fn check_collision(pos1: Vec2, radius1: f32, pos2: Vec2, radius2: f32) -> bool {
    let distance = (pos1 - pos2).length();
    distance < (radius1 + radius2)
}
```

**Usage:**
- Player vs Enemy: `check_collision(player.pos, 16.0, enemy.pos, 12.0)`
- Projectile vs Enemy: `check_collision(projectile.pos, 4.0, enemy.pos, 12.0)`

### Depth Sorting

**Rendering Order:**
- Sort all entities by `world_y` coordinate (ascending)
- Render in order: lowest Y → highest Y
- Entities with higher Y appear "in front"

```rust
entities.sort_by(|a, b| a.pos.y.partial_cmp(&b.pos.y).unwrap());
for entity in entities {
    draw_sprite(entity);
}
```

---

## Success Criteria

**Step 1 Complete When:**
- Player moves smoothly with WASD
- Shooting projectiles at mouse cursor works
- Rats chase player and deal damage
- Killing all rats shows victory screen
- Player death shows game over screen

**Step 2 Complete When:**
- Combat "feels good" (subjective, playtesting required)
- Hit feedback is satisfying
- Movement is responsive
- Challenge is balanced (not too easy/hard)

**Step 3 Complete When:**
- 2-3 weapons with distinct feel
- 2-3 enemy types with different behaviors
- Ammunition system functional
- UI shows health and ammo

---

## Open Questions (To Answer During Implementation)

1. **Projectile lifetime:** How long before despawn if doesn't hit? (Suggestion: 2 seconds OR screen bounds)
2. **Enemy spawn positions:** Random around edges OR fixed spawn points?
3. **Camera bounds:** Does camera move or stay fixed on player?
4. **World size:** Fixed arena (e.g., 1600x1200) or unbounded?
5. **Asset placeholders:** Colored circles OR simple sprite images?

**Resolve these during Step 1 coding - document decisions as you go.**

---

_Combat specification for Springfield Meltdown prototype - implementation guide for Phase 1 development._
