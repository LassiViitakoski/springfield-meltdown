# Technical Decisions Log

**Project:** Springfield Meltdown
**Purpose:** Architecture Decision Records (ADR) for cross-cutting technical choices

---

## How to Use This Document

Each decision follows the ADR format:
- **Date:** When decision made
- **Status:** Proposed | Accepted | Superseded | Deprecated
- **Context:** Problem being solved
- **Decision:** What we chose
- **Rationale:** Why we chose it
- **Alternatives Considered:** What we rejected and why
- **Consequences:** Trade-offs and implications
- **Related:** Links to epic/story implementations

---

## Decision Log

### ADR-001: Entity Storage - Vec vs HashMap vs Spatial Grid

**Date:** 2025-10-19
**Status:** Accepted
**Context:**

During Story SPRING-001-1.4 (Camera & Debug UI) implementation, discussed entity rendering optimization strategies for scaling from 10 entities (Epic 1) to 200+ entities (Epic 4+).

Key question: How should we store entities to balance simplicity (Rust learning phase) with future performance needs?

**Decision:**

Use `Vec<Entity>` for Epic 1-2, wrapped in `World::query_visible()` abstraction layer. Transition to spatial partitioning grid in Epic 4+ when entity count exceeds 200.

**Implementation:**
```rust
pub struct World {
    entities: Vec<Entity>,  // Phase 1-2
    // Future: SpatialGrid when entity count > 200
}

impl World {
    pub fn query_visible(&self, camera: &Camera) -> impl Iterator<Item = &Entity> {
        // Phase 1: Return all (no culling)
        self.entities.iter()

        // Phase 2: Add AABB viewport culling
        // Phase 3: Query spatial grid cells
    }
}
```

**Rationale:**

1. **Learning Priority:** Developer learning Rust + ggez in Epic 1 - defer complexity until fundamentals mastered
2. **Cache Locality:** Vec stores entities contiguously in memory - faster iteration than HashMap for small counts (<200)
3. **YAGNI Principle:** Don't implement spatial grid until entity count demands it (measured performance degradation)
4. **Abstraction Safety:** Query interface allows storage to evolve without breaking render code
5. **Profile-Driven:** Wait for profiling data before optimizing (avoid premature optimization)

**Alternatives Considered:**

| Alternative | Rejected Because |
|-------------|------------------|
| `HashMap<EntityId, Entity>` | Loses cache locality, no performance benefit at <200 entities, adds ID management complexity |
| Immediate spatial grid (chunking) | Premature optimization - not needed until >200 entities, adds learning overhead during Rust fundamentals phase |
| ECS framework (hecs, bevy_ecs) | Overkill for prototype scope, steep learning curve, defer until Phase 3+ if needed |

**Consequences:**

✅ **Positive:**
- Simple mental model during Rust learning
- Fast iteration for small entity counts (cache-friendly)
- Query abstraction future-proofs render code
- Single point of change when optimizing

⚠️ **Negative:**
- Random entity access is O(n) (acceptable - not a hot path)
- Must refactor storage in Epic 4+ (planned, low risk due to abstraction)
- No spatial queries in Phase 1-2 (collision is brute-force until Phase 3)

**Related:**
- Implementation: SPRING-001 (Epic 1)
- System Architecture: [Entity Management & Rendering](system-architecture.md#entity-management--rendering)
- Tech Spec: [tech-spec-epic-SPRING-001.md](../tech-specs/tech-spec-epic-SPRING-001.md)

**Revisit Trigger:**
- Entity count approaches 50 → implement viewport culling (Phase 2)
- Entity count exceeds 200 → implement spatial partitioning (Phase 3)
- Frame time >16ms with <200 entities → investigate other bottlenecks first

---

### ADR-002: Rendering Optimization - 3-Phase Evolution Strategy

**Date:** 2025-10-19
**Status:** Accepted
**Context:**

Discussion during SPRING-001-1.4: How do game engines handle world rendering when viewport (800x600) is much smaller than world size (e.g., 10,000x10,000)?

Low-level APIs (ggez, SDL, OpenGL) do NOT automatically cull off-screen objects. Developer must implement viewport culling in game code.

**Decision:**

Implement 3-phase optimization strategy based on entity count thresholds:

**Phase 1: Naive Rendering (Epic 1)**
- Render all entities every frame
- Acceptable for: 5-50 entities
- No optimization overhead

**Phase 2: Manual Viewport Culling (Epic 2-3)**
- AABB check each entity against camera bounds before rendering
- Only draw entities within viewport + 10% margin
- Acceptable for: 50-200 entities

**Phase 3: Spatial Partitioning (Epic 4+)**
- Grid-based world chunking (1000-1500px cells)
- Query only cells overlapping viewport
- Acceptable for: 200+ entities

**Rationale:**

1. **Bottleneck is CPU→GPU communication** (draw calls), not GPU rendering
2. Better to not submit off-screen objects at all (CPU-side culling)
3. Phase-gated complexity matches entity count scaling
4. Abstraction layer (`query_visible()`) isolates implementation changes

**Performance Impact (Example):**
- **Without culling:** 10,000 entities × 60 FPS = 600,000 checks/sec
- **With spatial grid:** ~50 visible entities × 60 FPS = 3,000 checks/sec
- **Improvement:** ~200x reduction

**Alternatives Considered:**

| Alternative | Rejected Because |
|-------------|------------------|
| Rely on GPU culling | GPU culls off-screen pixels, but draw call overhead already incurred |
| Octree/Quadtree | More complex than uniform grid for 2D world, diminishing returns |
| Immediate spatial partitioning | Premature optimization - not needed in Epic 1 |

**Consequences:**

✅ **Positive:**
- Scales gracefully as entity count grows
- Each phase simple to implement (incremental complexity)
- Clear transition thresholds (entity count, frame time)

⚠️ **Negative:**
- Phase 3 requires grid cell management (entities update cell on movement)
- Large entities may span multiple cells (register in all overlapping)

**Related:**
- System Architecture: [Entity Management - Evolution Strategy](system-architecture.md#evolution-strategy)
- Performance: [Optimization Triggers](system-architecture.md#optimization-triggers)

**Revisit Trigger:**
- Frame time >16ms → profile first, optimize bottleneck
- Entity count >50 → implement Phase 2
- Entity count >200 → implement Phase 3

---

### ADR-003: Coordinate System - World Space vs Screen Space Separation

**Date:** 2025-10-19
**Status:** Accepted
**Context:**

Springfield Meltdown uses 2.5D isometric rendering for visual style, but game logic (movement, collision, AI) operates in 2D top-down space.

Decision needed: Should entities store positions in world space or screen space?

**Decision:**

Use dual coordinate system:
- **World Space (2D Cartesian):** All game logic, entity positions, collision detection
- **Screen Space (Isometric):** Rendering only, transformed via `world_to_screen()`

**Rationale:**

1. **Conceptual Simplicity:** World space is top-down mental model - easier for logic/physics
2. **Math Clarity:** Distance calculations, vector math straightforward in Cartesian space
3. **Rendering Flexibility:** Isometric projection is rendering detail, doesn't leak into gameplay
4. **Future Camera Modes:** Can swap projection (top-down debug view) without changing logic

**Implementation:**
```rust
// Entities store world coordinates
pub struct Entity {
    pub pos: Vec2,  // World space (X, Y)
}

// Rendering transforms on-the-fly
fn render(entity: &Entity, camera: &Camera) {
    let screen_pos = world_to_screen(entity.pos, camera.offset);
    draw_sprite(screen_pos);
}

// Mouse input inverse-transforms
fn handle_mouse_click(screen_pos: Vec2, camera: &Camera) {
    let world_pos = screen_to_world(screen_pos, camera.offset);
    spawn_projectile(world_pos);
}
```

**Alternatives Considered:**

| Alternative | Rejected Because |
|-------------|------------------|
| Store positions in screen space | Distance calculations complex (non-Euclidean in isometric), collision detection harder |
| Store both world + screen | Redundant data, cache invalidation issues, memory overhead |
| True 3D coordinates | Overkill for 2.5D game, doesn't change gameplay (purely visual depth) |

**Consequences:**

✅ **Positive:**
- Simple game logic (2D vector math)
- Clean separation of concerns (logic vs rendering)
- Easy to test logic without rendering (unit tests)

⚠️ **Negative:**
- Transform overhead every frame (acceptable - fast math, inlined by compiler)
- Must implement inverse transform for mouse input (one-time complexity)

**Related:**
- System Architecture: [Coordinate Systems](system-architecture.md#coordinate-systems)
- Combat Spec: [Coordinate System](../specs/combat-spec.md#coordinate-system)
- Implementation: SPRING-001-1.2 (Player Entity & Sprite Rendering)

---

## Template for Future Decisions

### ADR-XXX: [Decision Title]

**Date:** YYYY-MM-DD
**Status:** Proposed | Accepted | Superseded | Deprecated
**Context:**

[What problem are we solving? What constraints exist?]

**Decision:**

[What did we choose to do?]

**Rationale:**

[Why did we make this choice? What principles guided us?]

**Alternatives Considered:**

| Alternative | Rejected Because |
|-------------|------------------|
| Option A | Reason |
| Option B | Reason |

**Consequences:**

✅ **Positive:**
- Benefit 1
- Benefit 2

⚠️ **Negative:**
- Trade-off 1
- Trade-off 2

**Related:**
- Links to docs, epics, stories

**Revisit Trigger:**
- Conditions that would invalidate this decision

---

## Decision Status Definitions

- **Proposed:** Under discussion, not yet implemented
- **Accepted:** Decision made and implemented
- **Superseded:** Replaced by newer decision (link to ADR-XXX)
- **Deprecated:** No longer relevant (explain why)

---

## Maintenance

**Update Frequency:** Add entry when making architectural decision that affects multiple epics.

**Review Cadence:** Audit at end of each phase to mark superseded/deprecated decisions.

**Owner:** Game Architect (Cloud Dragonborn)
