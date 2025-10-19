# Story: Collision Detection - Projectile vs Enemy

**Story ID:** SPRING-001-2.4
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 2
**Story Points:** 1
**Estimated Hours:** 1-2 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT my projectiles to damage enemies
SO THAT I can eliminate threats
```

---

## Acceptance Criteria

- [ ] Projectile hitting enemy deals 10 damage
- [ ] Enemy flashes white for 0.1 seconds on hit
- [ ] Projectile despawns on hit
- [ ] Enemy despawns when HP reaches 0
- [ ] Circle-circle collision detection implemented

---

## Technical Notes

- Collision check: distance < (radius1 + radius2)
- Projectile radius = 4px, Enemy radius = 12px
- Flash effect: temporary color override
- Remove dead entities from Vec with retain()

---

## References

- [Combat Specification - Collision Detection](../specs/combat-spec.md)

---

## Implementation Checklist

- [ ] Implement circle-circle collision function
- [ ] Check projectile-enemy collisions each frame
- [ ] Apply damage to enemy on hit
- [ ] Remove projectile on collision
- [ ] Implement hit flash effect
- [ ] Remove enemy when HP <= 0
- [ ] Test collision detection accuracy

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
