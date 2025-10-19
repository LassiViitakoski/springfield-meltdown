# Story: Projectile Spawning & Movement

**Story ID:** SPRING-001-2.2
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 2
**Story Points:** 3
**Estimated Hours:** 3-6 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT to shoot projectiles toward my mouse cursor
SO THAT I can attack enemies
```

---

## Acceptance Criteria

- [ ] Left mouse click spawns projectile at player position
- [ ] Projectile moves toward mouse position at 400 px/sec
- [ ] Projectile renders as small yellow circle (8px diameter)
- [ ] Fire rate limited to 0.3 seconds (can't spam-click)
- [ ] Projectiles despawn when off-screen (outside 1000px radius)

---

## Technical Notes

- Projectile struct: { pos: Vec2, velocity: Vec2, lifetime: f32 }
- Store projectiles in Vec<Projectile>
- Update each projectile: pos += velocity * delta_time
- Cooldown timer tracks last_shot_time
- **Architecture Note:** Projectiles are entities (pos, movement, rendering). Phase 1 uses Vec storage with naive rendering (all entities processed). Future: query_visible() abstraction per [system-architecture.md](../architecture/system-architecture.md) Phase 2+ when entity count >50

---

## References

- [Combat Specification - Weapon #1: Pistol](../specs/combat-spec.md)

---

## Implementation Checklist

- [ ] Create Projectile struct
- [ ] Add projectile list to GameState
- [ ] Implement mouse click detection
- [ ] Spawn projectile on click with cooldown
- [ ] Calculate projectile velocity toward cursor
- [ ] Update all projectiles each frame
- [ ] Render projectiles
- [ ] Despawn off-screen projectiles
- [ ] Test fire rate limiting

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
