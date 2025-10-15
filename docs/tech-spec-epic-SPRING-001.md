# Technical Specification: Combat Prototype Foundation

Date: 2025-10-15
Author: Lassi Viitakoski
Epic ID: SPRING-001
Status: Draft

---

## Overview

This technical specification defines the implementation architecture for Epic SPRING-001: Combat Prototype Foundation. This epic establishes the minimal viable combat system for Springfield Meltdown, a roguelite action game built in Rust using the ggez framework. The prototype validates core gameplay through player movement, mouse-aimed shooting, basic enemy AI, and health/damage systems rendered in 2.5D isometric perspective.

The combat prototype serves as the technical foundation for all future game systems, establishing patterns for entity management, coordinate transformations, game loop architecture, and state management that will scale throughout development.

## Objectives and Scope

**In Scope:**
- Rust project structure using ggez 0.9.x EventHandler pattern
- 8-directional player movement system with WASD controls (150 px/sec)
- 2D world coordinate system with isometric screen projection and depth sorting
- Mouse-aimed projectile shooting with cooldown mechanics (0.3sec fire rate)
- Basic enemy AI (chase behavior, 80 px/sec movement speed)
- Circle-circle collision detection for projectiles, enemies, and player
- Health and damage systems (player: 100 HP, enemy: 30 HP, projectile: 10 damage)
- Victory/defeat state machines and scene restart functionality
- Debug UI rendering (position, FPS display)
- Multiple enemy spawning (5-10 enemies at map edges, 200px minimum from player)

**Out of Scope (Future Epics):**
- Weapon variety beyond pistol (SPRING-002)
- Ammunition economy and resource management (SPRING-003)
- Save/load systems and progression (SPRING-004)
- Advanced enemy types and behaviors (SPRING-005)
- Visual polish, particles, screen shake (SPRING-002)
- Sound and music integration

## System Architecture Alignment

**Framework Architecture:**
- Leverages ggez's EventHandler trait pattern for game loop (update/draw separation)
- Uses ggez::graphics for 2D rendering primitives (circles, lines, text)
- Implements custom coordinate transformation layer (world ↔ screen isometric projection)
- Entity management via Vec<T> collections (no ECS framework in Phase 1)

**Design Constraints:**
- Rust ownership model requires careful entity lifetime management
- No heap allocations in hot path (pre-allocate entity vectors)
- 60 FPS target on modest hardware (single-threaded, CPU-bound)
- Simple data structures prioritized over premature optimization

## Detailed Design

### Services and Modules

**Module Structure:**

| Module | Responsibility | Inputs | Outputs | Owner |
|--------|---------------|--------|---------|-------|
| `main.rs` | Entry point, ggez context setup, event loop initialization | CLI args, window config | Initialized GameState | Core |
| `game_state.rs` | Core game state container, EventHandler implementation | User input, delta time | Render commands | Core |
| `entities/player.rs` | Player entity logic, movement, shooting | Keyboard/mouse input, delta time | Position, projectile spawns | Gameplay |
| `entities/enemy.rs` | Enemy entity logic, AI, health | Player position, delta time | Position, attack events | Gameplay |
| `entities/projectile.rs` | Projectile entity logic, movement, lifetime | Spawn position/velocity, delta time | Position, collision events | Gameplay |
| `systems/collision.rs` | Collision detection between entities | Entity positions/radii | Collision events | Gameplay |
| `systems/coordinate.rs` | World ↔ screen coordinate transformations | World coords, camera offset | Screen coords (isometric) | Rendering |
| `rendering/debug_ui.rs` | Debug text rendering (FPS, position) | Game state data | Text draw calls | Rendering |
| `utils/math.rs` | Vector math utilities (normalize, distance) | Vec2 values | Calculated results | Utils |

**Dependency Graph:**
```
main.rs
  └── game_state.rs
       ├── entities/player.rs
       ├── entities/enemy.rs
       ├── entities/projectile.rs
       ├── systems/collision.rs
       ├── systems/coordinate.rs
       └── rendering/debug_ui.rs
```

### Data Models and Contracts

**Core Data Structures:**

```rust
// src/entities/player.rs
pub struct Player {
    pub pos: Vec2,           // World coordinates (f32, f32)
    pub velocity: Vec2,      // Current movement vector
    pub speed: f32,          // 150.0 pixels/second
    pub hp: i32,             // Current health (max 100)
    pub max_hp: i32,         // 100
    pub radius: f32,         // 16.0 pixels (collision)
    pub last_shot_time: f32, // Cooldown tracking (seconds)
}

// src/entities/enemy.rs
pub struct Enemy {
    pub pos: Vec2,              // World coordinates
    pub hp: i32,                // Current health (max 30)
    pub speed: f32,             // 80.0 pixels/second
    pub radius: f32,            // 12.0 pixels (collision)
    pub last_attack_time: f32,  // Attack cooldown (1.0 second)
    pub hit_flash_timer: f32,   // Visual feedback (0.1 seconds)
}

// src/entities/projectile.rs
pub struct Projectile {
    pub pos: Vec2,       // World coordinates
    pub velocity: Vec2,  // Direction * 400 px/sec
    pub lifetime: f32,   // Despawn when > 5.0 seconds or off-screen
    pub damage: i32,     // 10
    pub radius: f32,     // 4.0 pixels (collision)
}

// src/game_state.rs
pub struct GameState {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub game_phase: GamePhase,
    pub camera_offset: Vec2,
}

pub enum GamePhase {
    Playing,
    PlayerDead,
    Victory,
}
```

**Coordinate Systems:**
- **World Coordinates:** 2D Cartesian (X, Y) where entities live
- **Screen Coordinates:** Isometric projection for rendering
- **Transformation:** `screen = world_to_screen(world, camera_offset)`

```rust
// src/systems/coordinate.rs
pub fn world_to_screen(world_pos: Vec2, camera_offset: Vec2) -> Vec2 {
    // Isometric projection: rotate 45° and scale Y by 0.5
    let iso_x = (world_pos.x - world_pos.y) * TILE_WIDTH / 2.0;
    let iso_y = (world_pos.x + world_pos.y) * TILE_HEIGHT / 2.0;
    Vec2::new(iso_x, iso_y) + camera_offset
}

pub fn screen_to_world(screen_pos: Vec2, camera_offset: Vec2) -> Vec2 {
    let adjusted = screen_pos - camera_offset;
    let world_x = (adjusted.x / (TILE_WIDTH / 2.0) + adjusted.y / (TILE_HEIGHT / 2.0)) / 2.0;
    let world_y = (adjusted.y / (TILE_HEIGHT / 2.0) - adjusted.x / (TILE_WIDTH / 2.0)) / 2.0;
    Vec2::new(world_x, world_y)
}
```

### APIs and Interfaces

**ggez EventHandler Implementation:**

```rust
impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ctx.time.delta().as_secs_f32();

        match self.game_phase {
            GamePhase::Playing => {
                self.update_player(ctx, delta_time);
                self.update_enemies(delta_time);
                self.update_projectiles(delta_time);
                self.check_collisions();
                self.check_win_loss_conditions();
            }
            _ => {
                // Handle restart input (R key)
                if ctx.keyboard.is_key_just_pressed(KeyCode::R) {
                    self.reset_scene();
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Render entities (depth-sorted by Y coordinate)
        self.render_entities(&mut canvas);

        // Render debug UI
        self.render_debug_ui(&mut canvas, ctx);

        // Render game over / victory messages
        if self.game_phase != GamePhase::Playing {
            self.render_game_over_message(&mut canvas);
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}
```

**Key Interface Contracts:**

| Interface | Method | Parameters | Returns | Purpose |
|-----------|--------|------------|---------|---------|
| Player | `update()` | `&input, delta: f32` | `Option<Projectile>` | Process input, return projectile if fired |
| Enemy | `update()` | `player_pos: Vec2, delta: f32` | `Vec2` | Calculate AI movement |
| Projectile | `update()` | `delta: f32` | `bool` | Return true if should despawn |
| Collision | `check_circle_collision()` | `pos1, r1, pos2, r2` | `bool` | Detect overlap |
| Coordinate | `world_to_screen()` | `world: Vec2, camera: Vec2` | `Vec2` | Transform coordinates |

### Workflows and Sequencing

**Game Loop Sequence (60 FPS):**

```
Frame Start (16.67ms budget)
  │
  ├─► Update Phase (ctx.update())
  │    ├─► 1. Read Input (keyboard, mouse)
  │    ├─► 2. Update Player
  │    │     ├─► Calculate movement vector from WASD
  │    │     ├─► Normalize diagonal movement
  │    │     ├─► Apply velocity: pos += vel * speed * dt
  │    │     ├─► Clamp to screen bounds
  │    │     └─► Check for mouse click → spawn projectile
  │    ├─► 3. Update Enemies (for each enemy)
  │    │     ├─► Calculate direction to player
  │    │     ├─► Normalize direction
  │    │     ├─► Apply velocity: pos += dir * speed * dt
  │    │     └─► Check attack cooldown
  │    ├─► 4. Update Projectiles (for each projectile)
  │    │     ├─► Apply velocity: pos += vel * dt
  │    │     ├─► Increment lifetime
  │    │     └─► Mark for removal if off-screen
  │    ├─► 5. Check Collisions
  │    │     ├─► Projectile ↔ Enemy collisions
  │    │     │     └─► Deal damage, remove projectile, flash enemy
  │    │     ├─► Enemy ↔ Player collisions
  │    │     │     └─► Deal damage (with cooldown)
  │    │     └─► Remove dead entities
  │    └─► 6. Check Win/Loss Conditions
  │          ├─► If player.hp <= 0 → GamePhase::PlayerDead
  │          └─► If enemies.len() == 0 → GamePhase::Victory
  │
  └─► Draw Phase (ctx.draw())
       ├─► 1. Clear screen (black background)
       ├─► 2. Depth-sort entities by world Y coordinate
       ├─► 3. Render entities (back to front)
       │     ├─► For each enemy: draw circle at screen coords
       │     ├─► Draw player: draw circle at screen coords
       │     ├─► For each projectile: draw circle at screen coords
       │     └─► Draw aim line (player → mouse cursor)
       ├─► 4. Render debug UI (FPS, position)
       └─► 5. Render game state messages
             ├─► If PlayerDead: "YOU DIED" (red, center)
             └─► If Victory: "YOU SURVIVED!" (green, center)
```

**Collision Detection Flow:**

```
For each projectile:
  For each enemy:
    distance = (projectile.pos - enemy.pos).length()
    if distance < (projectile.radius + enemy.radius):
      enemy.hp -= projectile.damage
      enemy.hit_flash_timer = 0.1
      mark projectile for removal
      if enemy.hp <= 0:
        mark enemy for removal

For each enemy:
  distance = (player.pos - enemy.pos).length()
  if distance < (player.radius + enemy.radius):
    if time_since_last_attack > 1.0:
      player.hp -= 5
      enemy.last_attack_time = current_time
```

## Non-Functional Requirements

### Performance

**Target Metrics:**
- **Frame Rate:** Maintain 60 FPS (16.67ms frame budget) on modest hardware
- **Entity Count:** Support 1 player + 10 enemies + 50 active projectiles without frame drops
- **Input Latency:** < 16ms from input to visual feedback (single-frame response)
- **Memory Footprint:** < 100 MB RAM for prototype (no asset streaming needed)
- **Startup Time:** < 2 seconds from launch to playable

**Performance Constraints:**
- No heap allocations in game loop hot path (pre-allocate Vec capacity)
- Collision detection: O(n*m) acceptable for prototype scale (n=projectiles, m=enemies)
- Depth sorting: Single-pass sort by Y coordinate per frame (acceptable for <100 entities)
- Rendering: All primitives drawn via ggez batched draw calls

**Optimization Strategy:**
- Profile-guided optimization deferred until Phase 2
- Premature optimization avoided; measure before optimizing
- Rust's zero-cost abstractions leveraged for clean code without runtime overhead

### Security

**Scope:** Single-player offline game, no network communication, no sensitive data handling.

**Security Requirements:**
- **N/A for Phase 1:** No authentication, no user data, no external integrations
- **File System Access:** Read-only access to game assets (no save system in prototype)
- **Input Validation:** Sanitize edge cases (divide-by-zero in normalize, NaN checks)

**Future Considerations (Phase 2+):**
- Save file integrity checks (prevent corrupted saves from crashing game)
- No eval() or unsafe code injection vectors (Rust's type system mitigates this)

### Reliability/Availability

**Target:**
- **Crash-Free:** No panics or unwraps in release builds (use Result/Option properly)
- **Graceful Degradation:** If frame rate drops below 30 FPS, game remains playable (time-based physics)
- **Recovery:** R key restart resets all state without restarting process

**Error Handling:**
- ggez errors (window creation, rendering) propagated via `GameResult<T>`
- Division by zero in vector normalization returns zero vector (safe fallback)
- Out-of-bounds access prevented by Rust's bounds checking

**Known Failure Modes:**
- Window resize not supported in Phase 1 (fixed 800x600 resolution)
- Alt+Tab may cause input state desync (acceptable for prototype)

### Observability

**Debug Instrumentation:**
- **On-Screen Debug UI:**
  - Player world position (X, Y) updated every frame
  - Current FPS (rolling average over 60 frames)
  - Entity counts (enemies alive, projectiles active)

**Logging Strategy (Phase 1):**
- `println!` for critical errors only (avoid performance impact)
- No structured logging framework (defer to Phase 2)
- Console output for initial setup and shutdown events

**Metrics to Track (Manual):**
- Playtesting feedback: "Does combat feel good?" (subjective)
- Time to complete victory condition (balancing metric)
- Player deaths per session (difficulty tuning)

**Future Observability (Phase 2+):**
- Integrate `log` crate for structured logging
- Performance profiling with `cargo flamegraph`
- Telemetry for gameplay metrics (time alive, enemies killed, etc.)

## Dependencies and Integrations

**Cargo.toml Dependencies:**

```toml
[package]
name = "springfield-meltdown"
version = "0.1.0"
edition = "2021"

[dependencies]
ggez = "0.9"

[profile.dev]
opt-level = 1  # Faster debug builds while learning

[profile.release]
lto = true
codegen-units = 1
```

**Dependency Rationale:**

| Dependency | Version | Purpose | Justification |
|------------|---------|---------|---------------|
| `ggez` | 0.9.x | Game framework (rendering, input, windowing) | Stable API, good docs, code-first approach, active community |
| Rust stdlib | 1.70+ | Vec, Result, Option, f32 math | No external deps needed for basic math/collections |

**External Integrations:**
- **None** - Fully self-contained single-player game
- No network, no database, no external services
- File system access: read-only for future asset loading (Phase 2)

**Build Requirements:**
- **Rust:** 1.70 or newer (2021 edition features)
- **Platform:** Windows (primary), Linux/macOS (ggez cross-platform support)
- **Graphics:** OpenGL 3.2+ or equivalent (ggez requirement)

**Known Dependency Constraints:**
- ggez 0.9.x uses `winit` for windowing (version pinned by ggez)
- Breaking changes expected in ggez 0.10+ (defer upgrade to Phase 2)

## Acceptance Criteria (Authoritative)

These criteria are extracted from the epic and serve as the **single source of truth** for implementation validation.

### Sprint 1: Core Movement & Setup

**AC-1.1: Project Setup & Window Rendering**
1. Cargo project compiles without errors with ggez = "0.9" dependency
2. Window opens at 800x600 resolution with black background
3. Window title displays "Springfield Meltdown - Prototype"
4. Game loop runs at stable 60 FPS
5. EventHandler trait implemented with empty update() and draw() methods

**AC-1.2: Player Entity & Sprite Rendering**
1. Player struct exists with position field (Vec2)
2. Player renders as yellow circle (color: #FFD700, radius: 16px)
3. Player spawns at screen center (400, 300) on startup
4. world_to_screen() function correctly transforms world coordinates to isometric screen position
5. Player sprite visible and positioned correctly

**AC-1.3: 8-Directional Player Movement**
1. WASD keys move player in 8 directions (cardinal + diagonal)
2. Movement speed measures 150 pixels/second (validated by position delta over time)
3. Diagonal movement normalized (same speed as cardinal directions, not 1.4x faster)
4. Movement feels smooth and responsive (subjective, manual testing)
5. Player collision with screen bounds prevents leaving 800x600 window area

**AC-1.4: Camera & Debug UI**
1. Camera centers on player position (player always at screen center)
2. Debug text displays player world coordinates formatted as "Pos: (X.X, Y.Y)"
3. Debug text displays current FPS formatted as "FPS: XX"
4. Text renders in top-left corner with white color
5. Text updates every frame

### Sprint 2: Combat Mechanics

**AC-2.1: Mouse Aiming Visualization**
1. Mouse position tracked in world coordinates (screen_to_world() implemented)
2. Red line drawn from player center to mouse cursor position
3. Line thickness: 2 pixels, color: red (#FF0000)
4. Line updates every frame as mouse moves
5. Aim direction visually accurate (line points at cursor)

**AC-2.2: Projectile Spawning & Movement**
1. Left mouse click spawns projectile at player position
2. Projectile moves toward click position at 400 px/sec
3. Projectile renders as yellow circle (8px diameter, #FFFF00)
4. Fire rate limited: cannot spawn projectile within 0.3 seconds of previous shot
5. Projectiles despawn when distance from spawn > 1000px or lifetime > 5 seconds

**AC-2.3: Enemy Entity & Chase AI**
1. Enemy struct created with position (Vec2) and hp (i32) fields
2. Enemy renders as green circle (24px diameter, #00FF00)
3. Enemy spawns at fixed position (200, 200) on game start
4. Enemy moves toward player at 80 px/sec every frame (validated by movement delta)
5. Enemy direction calculated as normalized vector: (player_pos - enemy_pos).normalize()

**AC-2.4: Collision Detection - Projectile vs Enemy**
1. Projectile hitting enemy deals exactly 10 damage to enemy.hp
2. Enemy flashes white (#FFFFFF) for 0.1 seconds on hit
3. Projectile despawns immediately on collision
4. Enemy despawns when hp <= 0
5. Circle-circle collision: collision = distance < (radius1 + radius2)

### Sprint 3: Health System & Victory/Defeat

**AC-3.1: Player Health & Enemy Damage**
1. Player has hp field (starts at 100, max_hp = 100)
2. Enemy collision with player deals 5 damage
3. Enemy attack has 1.0 second cooldown (cannot damage continuously)
4. HP displayed in debug UI as "HP: XX/100"
5. HP text color changes to red (#FF0000) when hp < 30

**AC-3.2: Death State & Game Over**
1. Game transitions to PlayerDead state when player.hp <= 0
2. "YOU DIED" text displays at screen center (large font, red color)
3. Press R key restarts scene (resets player, enemies, projectiles to initial state)
4. Player and enemies stop updating during death state (game frozen)
5. No progression loss or penalties (restart returns to fresh game state)

**AC-3.3: Multiple Enemies & Victory Condition**
1. Game spawns 5-10 enemies at random positions around map edges on start
2. Enemies spawn minimum 200 pixels from player spawn position
3. Victory triggers when enemies.len() == 0 (all enemies defeated)
4. "YOU SURVIVED!" text displays at screen center (large font, green color #00FF00)
5. Press R key restarts scene from victory state

## Traceability Mapping

This table maps acceptance criteria → tech spec sections → components → test approach.

| AC ID | Epic Story | Spec Section | Component(s) | Test Strategy |
|-------|-----------|--------------|--------------|---------------|
| AC-1.1 | SPRING-001-1.1 | Dependencies, Module Structure | main.rs, Cargo.toml | Manual: Verify window opens, check FPS counter |
| AC-1.2 | SPRING-001-1.2 | Data Models (Player), Coordinate System | entities/player.rs, systems/coordinate.rs | Manual: Verify sprite position, color, size |
| AC-1.3 | SPRING-001-1.3 | Workflows (Game Loop - Player Update) | entities/player.rs, game_state.rs | Manual: Test all 8 directions, measure speed with position logging |
| AC-1.4 | SPRING-001-1.4 | APIs (EventHandler), Debug UI | rendering/debug_ui.rs | Manual: Verify text displays, updates correctly |
| AC-2.1 | SPRING-001-2.1 | Coordinate System (screen_to_world) | systems/coordinate.rs | Manual: Move mouse, verify line accuracy |
| AC-2.2 | SPRING-001-2.2 | Data Models (Projectile), Workflows | entities/projectile.rs | Manual: Test fire rate, measure velocity, verify despawn |
| AC-2.3 | SPRING-001-2.3 | Data Models (Enemy), AI Logic | entities/enemy.rs | Manual: Observe chase behavior, measure speed |
| AC-2.4 | SPRING-001-2.4 | Workflows (Collision Detection) | systems/collision.rs | Manual: Fire at enemy, verify damage/despawn/flash |
| AC-3.1 | SPRING-001-3.1 | Data Models (Player.hp), Collision | entities/player.rs, systems/collision.rs | Manual: Let enemy hit player, verify damage/cooldown/UI |
| AC-3.2 | SPRING-001-3.2 | Data Models (GamePhase), State Machine | game_state.rs | Manual: Die, verify freeze/message/restart |
| AC-3.3 | SPRING-001-3.3 | Enemy Spawning, Victory Logic | game_state.rs, entities/enemy.rs | Manual: Kill all enemies, verify victory message |

## Risks, Assumptions, Open Questions

### Risks

**RISK-1: Rust/ggez Learning Curve**
- **Description:** Developer is learning Rust while implementing combat prototype
- **Impact:** HIGH - May take 2-3x longer than estimated (stories sized for experienced Rust dev)
- **Probability:** HIGH
- **Mitigation:**
  - Budget extra time in Sprint 1 (2-4 weeks instead of 1-2)
  - Focus on ggez examples and documentation before starting
  - Accept suboptimal Rust patterns in Phase 1 (refactor in Phase 2)
- **Owner:** Developer

**RISK-2: Isometric Coordinate Transform Complexity**
- **Description:** world_to_screen() and screen_to_world() math may be error-prone
- **Impact:** MEDIUM - Incorrect transforms break mouse aiming and rendering
- **Probability:** MEDIUM
- **Mitigation:**
  - Implement and test coordinate transforms in Story 1.2 (early validation)
  - Use visual debug aids (draw grid, show world coords on hover)
  - Fallback: Simplify to top-down view temporarily if blocked > 1 week
- **Owner:** Developer

**RISK-3: Combat "Feel" May Not Be Fun**
- **Description:** Subjective gameplay feel cannot be validated until playable
- **Impact:** HIGH - Core gameplay loop may feel sluggish or unsatisfying
- **Probability:** MEDIUM
- **Mitigation:**
  - Complete minimal playable loop quickly (Sprint 1-2)
  - Iterate on speed values (player speed, projectile speed, enemy speed)
  - Defer polish to optional Sprint 4 if needed
  - Accept "good enough" for prototype; refine in Phase 2
- **Owner:** Game Designer + Developer

**RISK-4: Scope Creep During Implementation**
- **Description:** Temptation to add features (new weapons, enemies, effects) mid-sprint
- **Impact:** MEDIUM - Delays epic completion, derails learning focus
- **Probability:** HIGH (common in creative projects)
- **Mitigation:**
  - Strictly adhere to epic scope (defer all additions to future epics)
  - Document new ideas in "Future Epics" section of GDD
  - Use Definition of Done as gate (don't add features until epic complete)
- **Owner:** Scrum Master + Developer

### Assumptions

**ASSUMPTION-1: ggez 0.9.x is stable and suitable**
- ggez 0.9 API is stable enough for prototype development
- Breaking changes in ggez 0.10 won't affect Phase 1 work
- Community support and documentation are sufficient for learning

**ASSUMPTION-2: Single-threaded performance is adequate**
- 60 FPS achievable with <100 entities on modest hardware
- No need for spatial partitioning or advanced optimizations in Phase 1
- Rust's zero-cost abstractions provide sufficient performance

**ASSUMPTION-3: Manual playtesting sufficient for validation**
- No automated tests required for prototype phase
- Developer can validate acceptance criteria through play
- Subjective "feel" assessment is acceptable without external playtesters

**ASSUMPTION-4: Fixed resolution is acceptable**
- 800x600 window without resize support is sufficient for prototype
- No need for fullscreen, scalable UI, or resolution options
- Defer these features to Phase 2

### Open Questions

**QUESTION-1: Coordinate system implementation**
- Should we use actual isometric projection or simplified 2.5D?
- Are the provided formulas for world_to_screen() correct for ggez coordinate system?
- **Resolution:** Test in Story 1.2, adjust if visual output incorrect

**QUESTION-2: Entity management pattern**
- Should we use Vec<T> or consider lightweight ECS pattern?
- How should we handle entity removal (retain() vs swap_remove())?
- **Resolution:** Start with Vec<T> for simplicity, refactor if performance issues

**QUESTION-3: Collision detection optimization**
- Is O(n*m) brute-force acceptable for 10 enemies + 50 projectiles?
- When should we implement spatial partitioning (grid, quadtree)?
- **Resolution:** Measure FPS in Sprint 2, optimize only if < 60 FPS

**QUESTION-4: State machine architecture**
- Should GamePhase enum be expanded for pause, menus, etc.?
- How should state transitions be managed (explicit methods vs pattern matching)?
- **Resolution:** Keep minimal for Phase 1, expand in Phase 2 as needed

## Test Strategy Summary

### Testing Approach

**Phase 1 Strategy: Manual Playtesting Only**
- **Rationale:** Prototype focuses on learning and validating gameplay feel
- **Coverage:** 100% of acceptance criteria validated through manual play
- **Frequency:** After completing each story (11 manual test sessions)

**Test Levels:**

| Level | Scope | Method | Frequency |
|-------|-------|--------|-----------|
| Unit Testing | N/A | None | Deferred to Phase 2 |
| Integration Testing | N/A | None | Deferred to Phase 2 |
| System Testing | Full gameplay loop | Manual playtesting | Per story completion |
| Acceptance Testing | Epic acceptance criteria | Manual validation checklist | Epic completion |

### Manual Test Scenarios

**Sprint 1 Testing (Stories 1.1-1.4):**
1. Launch game → verify window opens, FPS stable
2. Move player with WASD → verify 8 directions, smooth movement
3. Move diagonally → verify speed same as cardinal directions (not faster)
4. Try to leave screen → verify bounds collision
5. Check debug UI → verify position updates, FPS displays

**Sprint 2 Testing (Stories 2.1-2.4):**
1. Move mouse → verify aim line follows cursor accurately
2. Click to shoot → verify projectile spawns, moves correctly
3. Spam click → verify fire rate cooldown (can't shoot every frame)
4. Let projectile fly off-screen → verify despawn
5. Shoot enemy → verify damage, flash effect, enemy death at 30 damage

**Sprint 3 Testing (Stories 3.1-3.3):**
1. Let enemy touch player → verify damage, cooldown, HP UI
2. Let player die → verify death message, game freeze, R key restart
3. Kill all enemies → verify victory message, R key restart
4. Restart after death/victory → verify clean state reset

### Performance Testing

**Targets:**
- **Frame Rate:** 60 FPS maintained during gameplay
- **Entity Stress Test:** Spawn max entities (10 enemies, 50 projectiles) → verify no drops below 55 FPS

**Measurement:**
- Use debug FPS counter (on-screen display)
- Manual observation: does game feel smooth or choppy?
- Log warning if FPS drops below 55 for more than 1 second

### Edge Case Testing

**Known Edge Cases:**
1. **Divide-by-zero in normalize:** Zero-length vector → return (0, 0)
2. **Projectile spawns on top of enemy:** Should still register collision
3. **Multiple projectiles hit same enemy in one frame:** Apply all damage
4. **Enemy spawns overlapping player:** Unlikely with 200px minimum distance, acceptable if occurs

### Test Exit Criteria

**Sprint Completion:**
- All acceptance criteria for sprint stories manually validated ✓
- No blocking bugs (crashes, freezes, unplayable)
- FPS maintains 60 average during normal gameplay

**Epic Completion:**
- All 11 stories meet acceptance criteria
- Victory and defeat conditions work correctly
- Combat loop feels "good enough" (subjective approval)
- Epic marked complete, ready for Phase 2 polish

### Future Testing (Phase 2+)

**Deferred to Future Phases:**
- Unit tests for math utilities (normalize, distance, collision)
- Integration tests for entity interactions
- Automated acceptance tests (if Rust testing framework suitable)
- Performance profiling with cargo flamegraph
- External playtester feedback sessions
