# System Architecture - Springfield Meltdown

**Version:** 1.0
**Date:** 2025-10-19
**Status:** Living Document
**Author:** Lassi Viitakoski

---

## Overview

This document defines the cross-cutting technical architecture for Springfield Meltdown that spans multiple epics and development phases. It establishes design principles, system boundaries, and evolution strategies that guide implementation decisions across the project lifecycle.

**Scope:** Technical systems and patterns that affect multiple epics or require long-term planning.

**Not Covered Here:** Epic-specific implementations (see tech-specs/), feature specifications (see specs/), or product vision (see game-design-document.md).

---

## Architectural Principles

### 1. Profile-Driven Optimization
Measure performance before optimizing. Accept simple implementations until profiling proves need for complexity.

### 2. Abstraction at System Boundaries
Isolate implementation details behind query interfaces. Allow internal storage to evolve without breaking consumers.

### 3. Phase-Gated Complexity
Start simple (Phase 1), add sophistication when measured thresholds trigger (Phase 2+). Avoid premature optimization during learning phases.

### 4. Rust Ownership as Architecture
Leverage Rust's type system and ownership model to enforce architectural boundaries. Prefer compile-time safety over runtime flexibility.

### 5. Cache-Friendly Data Layout
Structure data for sequential access patterns. Prefer Vec over HashMap unless random access required by design.

---

## Entity Management & Rendering

### Overview

Entity management encompasses storage, querying, updating, and rendering of all game objects (player, enemies, projectiles, items, NPCs). This system evolves across three phases based on entity count and measured performance.

### Evolution Strategy

#### Phase 1: Naive Rendering (Epic 1 - Current)

**Implementation Status:** SPRING-001 (Combat Prototype Foundation)

**Storage:**
```rust
pub struct World {
    entities: Vec<Entity>,  // All entities in flat vector
}
```

**Query Pattern:**
```rust
impl World {
    pub fn query_visible(&self, camera: &Camera) -> impl Iterator<Item = &Entity> {
        // Phase 1: Return all entities (no culling)
        self.entities.iter()
    }
}
```

**Rendering:**
```rust
// Render code (never changes across phases)
for entity in world.query_visible(camera) {
    draw_sprite(entity, camera);
}
```

**Characteristics:**
- **Complexity:** O(n) - iterate all entities every frame
- **Entity Count:** <50 entities
- **Performance Target:** 60 FPS
- **Rationale:** Simple, cache-friendly, appropriate for learning Rust + small entity counts

**Limitations:**
- All entities processed even if off-screen
- Frame time increases linearly with entity count
- Acceptable until >50 entities or frame drops below 60 FPS

---

#### Phase 2: Viewport Culling (Epic 2-3)

**Implementation Status:** Planned for Epic 2-3 (entity count >50)

**Storage:**
```rust
pub struct World {
    entities: Vec<Entity>,  // Storage unchanged from Phase 1
}
```

**Query Pattern:**
```rust
impl World {
    pub fn query_visible(&self, camera: &Camera) -> impl Iterator<Item = &Entity> {
        // Phase 2: Filter by viewport bounds
        let visible_bounds = self.calculate_visible_bounds(camera);
        self.entities.iter()
            .filter(move |e| visible_bounds.contains(e.pos))
    }

    fn calculate_visible_bounds(&self, camera: &Camera) -> Rect {
        // Viewport + 10% margin for smooth camera movement
        Rect {
            x: camera.pos.x - 440.0,  // 400 + 10% margin
            y: camera.pos.y - 330.0,  // 300 + 10% margin
            w: 880.0,  // 800 + 10% margin
            h: 660.0,  // 600 + 10% margin
        }
    }
}
```

**Rendering:**
```rust
// Render code UNCHANGED - abstraction isolates implementation
for entity in world.query_visible(camera) {
    draw_sprite(entity, camera);
}
```

**Characteristics:**
- **Complexity:** O(n) - still check all entities, but skip rendering off-screen
- **Entity Count:** 50-200 entities
- **Performance Target:** 60 FPS
- **Optimization:** Only submit visible entities to GPU (reduces draw call overhead)

**AABB Collision Check:**
```rust
impl Rect {
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.w &&
        point.y >= self.y && point.y <= self.y + self.h
    }
}
```

**Margin Rationale:**
- 10% buffer prevents pop-in when camera moves smoothly
- Entities entering viewport already rendered
- Camera zoom support: margin scales with zoom level

---

#### Phase 3: Spatial Partitioning (Epic 4+)

**Implementation Status:** Planned for Epic 4+ (entity count >200)

**Storage:**
```rust
pub struct World {
    grid: SpatialGrid,                    // Grid-based spatial index
    entity_to_cell: HashMap<EntityId, CellCoords>,  // Track entity locations
}

pub struct SpatialGrid {
    cells: HashMap<CellCoords, Vec<Entity>>,
    cell_size: f32,  // 1000-1500 pixels (2-4x viewport dimensions)
}

#[derive(Hash, Eq, PartialEq)]
pub struct CellCoords {
    x: i32,
    y: i32,
}
```

**Query Pattern:**
```rust
impl World {
    pub fn query_visible(&self, camera: &Camera) -> impl Iterator<Item = &Entity> {
        // Phase 3: Query only cells overlapping viewport
        let visible_cells = self.grid.get_cells_in_bounds(camera.visible_bounds());
        visible_cells.iter()
            .flat_map(|cell| cell.entities.iter())
    }
}

impl SpatialGrid {
    pub fn get_cells_in_bounds(&self, bounds: Rect) -> Vec<&Vec<Entity>> {
        let min_cell = self.world_to_cell(bounds.top_left());
        let max_cell = self.world_to_cell(bounds.bottom_right());

        let mut cells = Vec::new();
        for x in min_cell.x..=max_cell.x {
            for y in min_cell.y..=max_cell.y {
                if let Some(cell) = self.cells.get(&CellCoords { x, y }) {
                    cells.push(cell);
                }
            }
        }
        cells
    }

    fn world_to_cell(&self, pos: Vec2) -> CellCoords {
        CellCoords {
            x: (pos.x / self.cell_size).floor() as i32,
            y: (pos.y / self.cell_size).floor() as i32,
        }
    }
}
```

**Rendering:**
```rust
// Render code STILL UNCHANGED - abstraction maintained
for entity in world.query_visible(camera) {
    draw_sprite(entity, camera);
}
```

**Entity Movement (Grid Updates):**
```rust
impl World {
    pub fn update_entity_position(&mut self, entity_id: EntityId, new_pos: Vec2) {
        let old_cell = self.entity_to_cell.get(&entity_id).copied();
        let new_cell = self.grid.world_to_cell(new_pos);

        // Only update grid if entity crossed cell boundary
        if old_cell != Some(new_cell) {
            if let Some(old) = old_cell {
                self.grid.remove_entity(old, entity_id);
            }
            self.grid.insert_entity(new_cell, entity_id);
            self.entity_to_cell.insert(entity_id, new_cell);
        }

        // Update entity position in storage
        self.entities.get_mut(&entity_id).unwrap().pos = new_pos;
    }
}
```

**Characteristics:**
- **Complexity:** O(visible_cells × avg_entities_per_cell)
- **Entity Count:** 200+ entities
- **Performance Target:** 60 FPS
- **Cell Size:** 1000-1500px (sweet spot: 2-4x viewport dimensions)

**Performance Impact Example:**
- **Without spatial grid:** 10,000 entities × 60 FPS = 600,000 checks/sec
- **With spatial grid:** ~50 visible entities × 60 FPS = 3,000 checks/sec
- **Improvement:** ~200x reduction in entity iteration cost

**Large Entity Handling:**
- Entities larger than cell size register in multiple cells
- Example: 2000px boss spans 4 cells → inserted into all 4
- Query returns duplicates → use `HashSet<EntityId>` to deduplicate

---

### Optimization Triggers

**Decision Framework:**

| Metric | Action |
|--------|--------|
| Frame time >16ms (60 FPS drop) | Profile first, identify bottleneck |
| Entity count >50 | Consider viewport culling (Phase 2) |
| Entity count >200 | Plan spatial partitioning (Phase 3) |
| Draw calls >1000/frame | Investigate batch rendering |
| Update phase >8ms | Profile entity logic, consider parallel update |

**Important:** Optimize when measured performance degrades, not when entity count hits arbitrary threshold. Different entity types (static vs moving, simple vs complex AI) have different performance characteristics.

---

### Profiling Strategy

#### Epic 1-2: Manual Timing

**Implementation:**
```rust
use std::time::Instant;

// In update loop
let start = Instant::now();
update_entities(&mut world);
let update_time = start.elapsed();

let start = Instant::now();
render_entities(&world, camera);
let render_time = start.elapsed();

// Display in debug UI (Story 1.4 feature)
println!("Update: {:.2}ms | Render: {:.2}ms",
    update_time.as_secs_f32() * 1000.0,
    render_time.as_secs_f32() * 1000.0
);
```

**Frame Budget Allocation (60 FPS = 16.67ms/frame):**
- Input: 1ms
- Update: 8ms (entity logic, AI, collision)
- Render: 6ms (transform, draw calls, GPU submit)
- Overhead: 1.67ms buffer

**Warning Threshold:**
- Log warning if frame time >18ms for >1 second
- Investigate if consistent drops below 55 FPS

#### Epic 3+: Profiler Integration

**Tools:**
- `puffin` crate (visual profiler, frame-by-frame analysis)
- `cargo flamegraph` (identify hot paths)
- ggez built-in timing stats

**Example puffin integration:**
```rust
use puffin;

fn update(&mut self, ctx: &mut Context) {
    puffin::profile_scope!("update");

    {
        puffin::profile_scope!("update_entities");
        self.update_entities(ctx);
    }

    {
        puffin::profile_scope!("collision_detection");
        self.check_collisions();
    }
}
```

---

### Camera Evolution Impact

#### Smooth Camera (Epic 2)

**Implementation:**
```rust
// Lerp-based camera follow
camera.pos = camera.pos.lerp(player.pos, 0.1);
```

**Culling Impact:**
- Viewport bounds change gradually (not instant snap)
- **Solution:** 10% margin already accounts for smooth movement
- No special handling needed - margin prevents pop-in

#### Zoom System (Epic 3+)

**Implementation:**
```rust
pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,  // 1.0 = default, 0.5 = zoomed out, 2.0 = zoomed in
}
```

**Culling Impact:**
- Viewport size changes dynamically (zoomed out = larger visible area)
- **Solution:** Calculate culling bounds from camera state:

```rust
fn calculate_visible_bounds(&self, camera: &Camera) -> Rect {
    let half_width = (400.0 / camera.zoom) * 1.1;  // Base viewport / zoom + margin
    let half_height = (300.0 / camera.zoom) * 1.1;

    Rect {
        x: camera.pos.x - half_width,
        y: camera.pos.y - half_height,
        w: half_width * 2.0,
        h: half_height * 2.0,
    }
}
```

**No architectural changes needed** - culling bounds already calculated from camera.

#### Predictive Culling (Epic 4+ - Optional)

**Concept:** Pre-load entities about to enter viewport based on camera velocity.

```rust
fn calculate_predictive_bounds(&self, camera: &Camera) -> Rect {
    let base_bounds = self.calculate_visible_bounds(camera);
    let velocity_offset = camera.velocity * 0.5;  // Look ahead 0.5 seconds

    base_bounds.expand_in_direction(velocity_offset)
}
```

**When to implement:** Only if profiling shows viewport culling is bottleneck (unlikely).

---

### Design Rationale

**Why Vec<Entity> instead of HashMap<EntityId, Entity>?**
- Cache locality: Vec stores entities contiguously in memory
- Iteration speed: Sequential access pattern (hot path in game loop)
- Simplicity: No ID management overhead in Phase 1
- Trade-off: Random access O(n) - acceptable for small counts, addressed by spatial grid in Phase 3

**Why abstraction layer (query_visible) instead of direct access?**
- **Future-proof:** Render code unchanged when storage evolves (Phase 1 → 2 → 3)
- **Single point of change:** Optimization logic centralized in World implementation
- **Testing:** Can swap query implementation without touching render tests
- **Cost:** Zero-cost abstraction (inlined by compiler, no runtime overhead)

**Why three phases instead of implementing spatial grid immediately?**
- **Learning priority:** Rust + ggez learning curve high in Epic 1 - defer complexity
- **YAGNI principle:** Don't need spatial grid until entity count demands it
- **Measurement-driven:** Premature optimization wastes effort - wait for profiling data
- **Incremental risk:** Phase 2 (AABB culling) is low-risk stepping stone to Phase 3

---

## Coordinate Systems

### Overview

Springfield Meltdown uses a dual coordinate system: 2D world space for game logic and isometric screen space for rendering. This separation allows simple logic (movement, collision) while achieving 2.5D visual depth.

### World Space (Logic Layer)

**Definition:** 2D Cartesian coordinate system where all game logic operates.

**Characteristics:**
- Origin: (0, 0) at top-left
- Axes: +X right, +Y down
- Units: Pixels (1 unit = 1 pixel in world space)
- Data Type: `Vec2 { x: f32, y: f32 }`

**Usage:**
- Entity positions: `player.pos = Vec2::new(100.0, 200.0)`
- Velocity calculations: `velocity = (target - pos).normalize()`
- Collision detection: `distance = (pos1 - pos2).length()`
- AI pathfinding: A* operates in world space grid

**Rationale:**
- Conceptually simple: "top-down" mental model
- Matches physics conventions (2D vector math)
- Distance calculations straightforward (Euclidean distance)

### Screen Space (Rendering Layer)

**Definition:** Isometric projection transforms world coordinates to achieve 2.5D depth illusion.

**Projection Formula:**
```rust
const TILE_WIDTH_HALF: f32 = 32.0;   // Half of isometric tile width
const TILE_HEIGHT_HALF: f32 = 16.0;  // Half of isometric tile height

pub fn world_to_screen(world_pos: Vec2, camera_offset: Vec2) -> Vec2 {
    // Isometric projection: rotate 45° and compress Y axis
    let iso_x = (world_pos.x - world_pos.y) * TILE_WIDTH_HALF;
    let iso_y = (world_pos.x + world_pos.y) * TILE_HEIGHT_HALF;

    Vec2::new(iso_x, iso_y) + camera_offset
}
```

**Inverse Projection (Mouse Input):**
```rust
pub fn screen_to_world(screen_pos: Vec2, camera_offset: Vec2) -> Vec2 {
    let adjusted = screen_pos - camera_offset;

    let world_x = (adjusted.x / TILE_WIDTH_HALF + adjusted.y / TILE_HEIGHT_HALF) / 2.0;
    let world_y = (adjusted.y / TILE_HEIGHT_HALF - adjusted.x / TILE_WIDTH_HALF) / 2.0;

    Vec2::new(world_x, world_y)
}
```

**Visual Explanation:**
```
World Space (Top-Down):         Screen Space (Isometric):

     Y
     ↓                               ╱ Y'
  ┌─────┐                         ╱
  │  E  │                      ╱
  └─────┘                   ╱
→ X                      X' →

Entity at (100, 100)     Entity renders at (0, 216)
                         (assuming TILE_WIDTH_HALF=32, TILE_HEIGHT_HALF=16)
```

### Depth Sorting

**Problem:** Isometric rendering requires entities to render in correct order (back-to-front) to simulate depth.

**Solution:** Sort entities by world Y coordinate before rendering.

```rust
impl World {
    pub fn render(&self, canvas: &mut Canvas, camera: &Camera) {
        // Sort by Y coordinate (ascending)
        let mut visible = self.query_visible(camera).collect::<Vec<_>>();
        visible.sort_by(|a, b| a.pos.y.partial_cmp(&b.pos.y).unwrap());

        // Render back to front
        for entity in visible {
            let screen_pos = world_to_screen(entity.pos, camera.offset);
            draw_sprite(canvas, entity.sprite, screen_pos);
        }
    }
}
```

**Rationale:**
- Entities with lower Y (farther "back" in world) render first
- Entities with higher Y (closer "forward") render on top
- Creates correct overlap illusion

**Edge Cases:**
- Entities at same Y coordinate: render order undefined (acceptable for prototype)
- Large sprites spanning multiple Y values: use sprite base position (foot position)
- Particles/effects: render in separate pass after entities (always on top)

### Camera Transform

**Camera Offset Calculation:**
```rust
pub struct Camera {
    pub pos: Vec2,  // World position camera is focused on
}

impl Camera {
    pub fn offset(&self, screen_size: Vec2) -> Vec2 {
        // Convert camera world position to screen, then center
        let screen_cam = world_to_screen(self.pos, Vec2::ZERO);
        let center = screen_size / 2.0;
        center - screen_cam
    }
}
```

**Usage in Rendering:**
```rust
let camera_offset = camera.offset(Vec2::new(800.0, 600.0));
let screen_pos = world_to_screen(entity.pos, camera_offset);
draw_sprite(canvas, entity.sprite, screen_pos);
```

**Camera Follow (Phase 1 - Fixed):**
```rust
// Camera snaps to player position
camera.pos = player.pos;
```

**Camera Follow (Phase 2 - Smooth):**
```rust
// Camera lerps toward player (smooth follow)
camera.pos = camera.pos.lerp(player.pos, 0.1);
```

### Constants Tuning

**Current Values:**
- `TILE_WIDTH_HALF = 32.0` → Full tile width = 64px
- `TILE_HEIGHT_HALF = 16.0` → Full tile height = 32px
- Ratio: 2:1 (standard isometric ratio)

**When to Adjust:**
- Visual style change (more/less aggressive angle)
- Sprite asset dimensions (match artist's tile grid)
- Performance (larger tiles = fewer on screen)

**Testing Formula:**
```rust
// Round-trip test: world → screen → world should equal original
let world_original = Vec2::new(100.0, 200.0);
let screen = world_to_screen(world_original, Vec2::ZERO);
let world_recovered = screen_to_world(screen, Vec2::ZERO);

assert!((world_original - world_recovered).length() < 0.01); // Floating point tolerance
```

---

## Collision Detection

### Overview

Collision detection uses circle-based bounding volumes for all entities. This provides acceptable accuracy with minimal computational cost for prototype and early development phases.

### Circle-Circle Collision

**Algorithm:**
```rust
pub fn check_collision(pos1: Vec2, radius1: f32, pos2: Vec2, radius2: f32) -> bool {
    let distance = (pos1 - pos2).length();
    distance < (radius1 + radius2)
}
```

**Optimization (avoid sqrt):**
```rust
pub fn check_collision_fast(pos1: Vec2, radius1: f32, pos2: Vec2, radius2: f32) -> bool {
    let distance_squared = (pos1 - pos2).length_squared();
    let radius_sum = radius1 + radius2;
    distance_squared < radius_sum * radius_sum
}
```

**When to use optimized version:** Phase 2+ when collision checks >1000/frame.

### Collision Pairs

**Epic 1-3 Scope:**

| Pair | Method | Frequency | Notes |
|------|--------|-----------|-------|
| Projectile ↔ Enemy | Circle-circle | Every frame | Most common check |
| Player ↔ Enemy | Circle-circle | Every frame | Contact damage |
| Player ↔ Item | Circle-circle | Every frame | Pickup detection |
| Projectile ↔ Obstacle | Deferred to Epic 4+ | - | Environmental collision |

**Collision Matrix (Phase 1):**
```
           Player  Enemy  Projectile  Item
Player       -      ✓        -         ✓
Enemy        ✓      -        ✓         -
Projectile   -      ✓        -         -
Item         ✓      -        -         -
```

### Broad Phase (Phase 3+)

**Problem:** O(n²) collision checks expensive with >200 entities.

**Solution:** Leverage spatial grid from rendering system.

```rust
impl World {
    pub fn check_collisions(&mut self) {
        // For each projectile
        for projectile in &self.projectiles {
            // Query only entities in same cell as projectile
            let cell = self.grid.world_to_cell(projectile.pos);
            let nearby_enemies = self.grid.get_entities_in_cell(cell);

            for enemy in nearby_enemies {
                if check_collision(projectile.pos, projectile.radius, enemy.pos, enemy.radius) {
                    self.handle_projectile_hit(projectile.id, enemy.id);
                }
            }
        }
    }
}
```

**Performance Impact:**
- **Without spatial grid:** 50 projectiles × 200 enemies = 10,000 checks/frame
- **With spatial grid:** 50 projectiles × ~5 nearby enemies = 250 checks/frame
- **Improvement:** ~40x reduction

### Collision Response

**Damage Application:**
```rust
fn handle_projectile_hit(&mut self, projectile_id: EntityId, enemy_id: EntityId) {
    // Apply damage
    if let Some(enemy) = self.enemies.get_mut(&enemy_id) {
        enemy.hp -= self.projectiles.get(&projectile_id).unwrap().damage;
        enemy.hit_flash_timer = 0.1;  // Visual feedback

        if enemy.hp <= 0 {
            self.mark_for_removal(enemy_id);
        }
    }

    // Remove projectile
    self.mark_for_removal(projectile_id);
}
```

**Cooldown-Based Contact Damage:**
```rust
fn handle_enemy_contact(&mut self, enemy_id: EntityId) {
    let current_time = /* get game time */;

    if let Some(enemy) = self.enemies.get_mut(&enemy_id) {
        if current_time - enemy.last_attack_time >= 1.0 {
            self.player.hp -= 5;
            enemy.last_attack_time = current_time;
        }
    }
}
```

### Future Enhancements (Phase 4+)

**Collision Layers:**
```rust
pub enum CollisionLayer {
    Player,
    Enemy,
    Projectile,
    Environment,
    Trigger,
}

// Collision matrix defined in config
// Example: Player projectiles don't hit player
```

**Polygon Collision (if needed):**
- SAT (Separating Axis Theorem) for complex shapes
- Only implement if circle collision proves insufficient
- Likely unnecessary for top-down roguelite genre

---

## Performance Budget & Monitoring

### Frame Budget (60 FPS = 16.67ms)

**Allocation:**

| System | Budget | Phase 1 Actual | Notes |
|--------|--------|----------------|-------|
| Input Processing | 1ms | <0.5ms | Keyboard/mouse polling |
| Entity Update | 8ms | ~2ms (50 entities) | AI, movement, logic |
| Collision Detection | 2ms | <1ms (Phase 1) | Part of update budget |
| Rendering | 6ms | ~3ms | Transform + draw calls |
| Buffer/Overhead | 1.67ms | - | OS, ggez overhead |

**Warning Thresholds:**
- Yellow flag: Frame time >18ms (sustained >1 second)
- Red flag: Frame time >20ms (FPS drops below 50)

### Debug Performance Panel

**Display in Debug UI (Story 1.4):**
```rust
pub struct DebugUI {
    frame_times: VecDeque<f32>,  // Rolling window (60 frames)
}

impl DebugUI {
    pub fn render(&self, canvas: &mut Canvas) {
        println!("FPS: {:.1}", self.calculate_fps());
        println!("Frame Time: {:.2}ms", self.avg_frame_time() * 1000.0);
        println!("Entities: {}", self.entity_count);
        println!("Update: {:.2}ms", self.last_update_time * 1000.0);
        println!("Render: {:.2}ms", self.last_render_time * 1000.0);
    }

    fn calculate_fps(&self) -> f32 {
        1.0 / self.avg_frame_time()
    }
}
```

### Performance Testing Scenarios

**Stress Test (Manual):**
1. Spawn maximum entities (10 enemies, 50 projectiles)
2. Run for 60 seconds
3. Verify FPS stays above 55
4. Check debug UI for budget violations

**Regression Test (Each Epic):**
- Record baseline FPS with fixed entity count
- Compare after major changes
- Investigate if >10% degradation

---

## State Management

### Game State Architecture

**Top-Level State Machine:**
```rust
pub enum GamePhase {
    MainMenu,
    PreMission,      // Character select, loadout, shop
    Playing,
    Paused,
    PlayerDead,
    Victory,
    PostMission,     // Rewards, unlocks
}
```

**State Transitions (Phase 1):**
```
Playing → PlayerDead (player.hp <= 0)
Playing → Victory (enemies.len() == 0)
PlayerDead → Playing (press R to restart)
Victory → Playing (press R to restart)
```

**State Transitions (Phase 2+):**
```
MainMenu → PreMission (start game)
PreMission → Playing (begin mission)
Playing ↔ Paused (ESC key)
Victory → PostMission (automatic)
PostMission → PreMission (next mission)
```

### Entity Lifecycle Management

**Creation:**
```rust
impl World {
    pub fn spawn_entity(&mut self, entity: Entity) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        self.entities.insert(id, entity);

        // Phase 3: Register in spatial grid
        // let cell = self.grid.world_to_cell(entity.pos);
        // self.grid.insert_entity(cell, id);

        id
    }
}
```

**Removal (Deferred):**
```rust
impl World {
    pub fn mark_for_removal(&mut self, entity_id: EntityId) {
        self.pending_removals.push(entity_id);
    }

    pub fn process_removals(&mut self) {
        for id in self.pending_removals.drain(..) {
            self.entities.remove(&id);

            // Phase 3: Unregister from spatial grid
            // if let Some(cell) = self.entity_to_cell.remove(&id) {
            //     self.grid.remove_entity(cell, id);
            // }
        }
    }
}
```

**Why deferred removal?**
- Avoid iterator invalidation during entity iteration
- Batch removals at end of update loop
- Safe mutation pattern (Rust borrow checker friendly)

### Save/Load System (Phase 2+)

**Serialization Strategy:**
```rust
// Use serde for serialization
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    player_unlocks: Vec<CharacterId>,
    owned_weapons: Vec<WeaponId>,
    permanent_upgrades: UpgradeState,
    currency: CurrencyAmounts,
    mission_progress: MissionProgress,
}
```

**Persistence:**
- Save location: `~/.local/share/springfield-meltdown/save.json`
- Auto-save: After mission completion (PostMission state)
- Manual save: Not needed (roguelite - no mid-mission saves)

---

## Module Organization

### Directory Structure

```
src/
├── main.rs                    # Entry point, ggez setup
├── game_state.rs              # GameState, EventHandler impl
├── entities/
│   ├── mod.rs
│   ├── player.rs
│   ├── enemy.rs
│   ├── projectile.rs
│   └── item.rs
├── systems/
│   ├── mod.rs
│   ├── collision.rs           # Collision detection
│   ├── coordinate.rs          # World ↔ screen transforms
│   ├── spatial_grid.rs        # Phase 3: Spatial partitioning
│   └── ai.rs                  # Enemy AI behaviors
├── rendering/
│   ├── mod.rs
│   ├── debug_ui.rs            # Debug overlay
│   ├── sprites.rs             # Sprite rendering
│   └── effects.rs             # Particles, screen shake
├── utils/
│   ├── mod.rs
│   └── math.rs                # Vec2 extensions, distance, normalize
└── config.rs                  # Constants, tuning parameters
```

### Module Dependency Rules

**Allowed:**
- entities → utils (math helpers)
- systems → entities (read entity data)
- rendering → systems (coordinate transforms)
- game_state → ALL (orchestrator)

**Forbidden:**
- entities → systems (entities don't know about collision system)
- entities → rendering (entities don't know how they're drawn)
- utils → ANY (pure utilities, no dependencies)

**Rationale:**
- Clear dependency hierarchy (no circular dependencies)
- Entities are data containers (minimal logic)
- Systems operate on entities (separation of concerns)

---

## Technology Evolution

### Dependency Stability

**Locked Versions (Phase 1-2):**
```toml
[dependencies]
ggez = "0.9"  # Stable API, defer ggez 0.10 migration to Phase 3
```

**Planned Additions (Phase 2+):**
```toml
serde = { version = "1.0", features = ["derive"] }  # Save system
serde_json = "1.0"
rand = "0.8"  # Procedural generation
```

**Deferred Dependencies:**
- ECS framework (hecs, bevy_ecs) - Phase 4+ if needed
- Pathfinding (pathfinding crate) - Epic 3-4
- Networking (if multiplayer ever considered) - Not planned

### Rust Edition

**Current:** 2021 edition

**Planned Upgrades:**
- Upgrade to Rust 2024 edition when stable (expect 2024-2025)
- Review breaking changes, update idioms

---

## Architectural Evolution Roadmap

### Epic 1 (SPRING-001): Foundation
- ✓ Vec<Entity> storage
- ✓ World::query_visible() abstraction (returns all)
- ✓ Manual timing profiler
- ✓ Circle-circle collision (brute force)

### Epic 2-3: Content Scaling
- → Viewport culling (AABB filtering)
- → Smooth camera (lerp follow)
- → Collision optimizations (fast distance check)
- → Performance panel improvements

### Epic 4+: Optimization
- → Spatial grid (Phase 3 rendering)
- → Broad-phase collision
- → Advanced profiling (puffin integration)
- → Possible ECS migration (if entity complexity demands)

---

## Cross-Cutting Concerns

### Error Handling

**Strategy:**
- Use `Result<T, E>` for fallible operations (file I/O, asset loading)
- Use `Option<T>` for expected absence (entity lookup by ID)
- Avoid `unwrap()` in release builds (use `expect()` with context)

**Example:**
```rust
// Good: Explicit error handling
pub fn load_sprite(&self, path: &str) -> Result<Sprite, LoadError> {
    Image::from_path(path).map_err(|e| LoadError::SpriteNotFound(path.to_string(), e))
}

// Bad: Panic in release
let sprite = Image::from_path(path).unwrap();  // Don't do this
```

### Logging Strategy

**Phase 1:** Minimal logging (println! for critical errors only)

**Phase 2+:**
```rust
use log::{info, warn, error};

// Initialization
env_logger::init();

// Usage
info!("Mission started: {}", mission_id);
warn!("Entity count approaching threshold: {}", count);
error!("Failed to load asset: {}", path);
```

**Log Levels:**
- ERROR: Unrecoverable failures (asset missing, save corruption)
- WARN: Performance issues, fallback behavior
- INFO: Major game events (mission start/end, level up)
- DEBUG: Detailed entity state (only in dev builds)

### Constants Management

**File:** `src/config.rs`

```rust
// Gameplay tuning
pub const PLAYER_SPEED: f32 = 150.0;
pub const ENEMY_CHASE_SPEED: f32 = 80.0;
pub const PISTOL_FIRE_RATE: f32 = 0.3;

// Rendering
pub const TILE_WIDTH_HALF: f32 = 32.0;
pub const TILE_HEIGHT_HALF: f32 = 16.0;

// Performance
pub const MAX_PROJECTILES: usize = 100;
pub const SPATIAL_GRID_CELL_SIZE: f32 = 1000.0;
```

**Rationale:**
- Centralized tuning (change one place, affect all systems)
- Easy playtesting iteration (tweak values without code diving)
- Future: Load from config file (TOML/JSON) for modding support

---

## Document Maintenance

**Update Triggers:**
- Major architectural decision (add new section)
- Phase transition (update implementation status)
- Performance threshold change (revise budgets)
- Technology upgrade (update dependency strategy)

**Review Cadence:**
- End of each epic (validate accuracy)
- Before starting new phase (ensure alignment)

**Version History:**

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-10-19 | Lassi Viitakoski | Initial draft: Entity Management, Coordinate Systems, Collision, Performance |

---

**Next Steps:**
1. Reference this document from tech-specs/tech-spec-epic-SPRING-001.md
2. Create technical-decisions.md with entity storage decision log entry
3. Update specs/combat-spec.md to reference coordinate system section
4. Implement Phase 1 abstractions in SPRING-001-1.4 (Camera & Debug UI)
