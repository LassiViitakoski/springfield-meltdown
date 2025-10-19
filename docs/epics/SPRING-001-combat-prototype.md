# Epic: Combat Prototype Foundation

**Epic ID:** SPRING-001
**Epic Name:** Combat Prototype Foundation
**Epic Owner:** Lassi Viitakoski
**Target Release:** Phase 1 MVP
**Epic Goal:** Prove core combat loop is fun through playable prototype
**Estimated Hours:** 5-10 hours/week for 6-12 weeks
**Status:** Draft

---

## Epic Overview

**Description:**
Build minimal viable combat system to validate Springfield Meltdown's core gameplay. Implement player movement, mouse-aimed shooting, basic enemy AI, and health/damage systems. Success = playtester says "this feels good to play."

**Success Criteria:**
- Player can move smoothly in 8 directions
- Shooting projectiles at mouse cursor works reliably
- Enemy chases player and deals damage on contact
- Victory/defeat states function correctly
- Combat feels responsive and satisfying (subjective playtesting)

**Out of Scope (Future Epics):**
- Multiple weapons/enemies (Epic SPRING-002)
- Ammunition economy (Epic SPRING-003)
- Save/progression systems (Epic SPRING-004)
- Polish/juice (Epic SPRING-002)

**Story Point Estimate:** 21 points (Fibonacci scale)
**Expected Duration:** 3-6 sprints (6-12 weeks at 5-10 hours/week, includes Rust + ggez learning curve)

---

## User Stories

### Sprint 1: Core Movement & Setup (8 points)
**Sprint Goal:** Player can move in empty world, project setup complete
**Expected Duration:** 2-4 weeks (8-20 hours)

- [SPRING-001-1.1: Project Setup & Window Rendering](../stories/SPRING-001-1.1-project-setup.md) - 2 points
- [SPRING-001-1.2: Player Entity & Sprite Rendering](../stories/SPRING-001-1.2-player-sprite.md) - 2 points
- [SPRING-001-1.3: 8-Directional Player Movement](../stories/SPRING-001-1.3-movement.md) - 3 points
- [SPRING-001-1.4: Camera & Debug UI](../stories/SPRING-001-1.4-camera-debug.md) - 1 point

### Sprint 2: Combat Mechanics (8 points)
**Sprint Goal:** Player can shoot projectiles at mouse cursor, basic combat loop works
**Expected Duration:** 2-4 weeks (8-20 hours)

- [SPRING-001-2.1: Mouse Aiming Visualization](../stories/SPRING-001-2.1-mouse-aiming.md) - 2 points
- [SPRING-001-2.2: Projectile Spawning & Movement](../stories/SPRING-001-2.2-projectiles.md) - 3 points
- [SPRING-001-2.3: Enemy Entity & Chase AI](../stories/SPRING-001-2.3-enemy-ai.md) - 2 points
- [SPRING-001-2.4: Collision Detection - Projectile vs Enemy](../stories/SPRING-001-2.4-collision.md) - 1 point

### Sprint 3: Health System & Victory/Defeat (5 points)
**Sprint Goal:** Complete combat loop with win/loss conditions
**Expected Duration:** 1-3 weeks (5-15 hours)

- [SPRING-001-3.1: Player Health & Enemy Damage](../stories/SPRING-001-3.1-player-health.md) - 2 points
- [SPRING-001-3.2: Death State & Game Over](../stories/SPRING-001-3.2-death-state.md) - 2 points
- [SPRING-001-3.3: Multiple Enemies & Victory Condition](../stories/SPRING-001-3.3-victory.md) - 1 point

---

## Sprint Planning Summary

### Sprint 1: Foundation (Week 1-4)
- **Story Points:** 8
- **Estimated Hours:** 8-14 hours
- **Goal:** Playable character moving in empty world
- **Deliverable:** Can move player with WASD, see debug info
- **Risk:** Learning Rust + ggez basics (might take longer than estimated)
- **Mitigation:** Budget extra time for tutorials, documentation reading

### Sprint 2: Combat Mechanics (Week 5-8)
- **Story Points:** 8
- **Estimated Hours:** 8-16 hours
- **Goal:** Shooting works, enemies chase player
- **Deliverable:** Can shoot enemies, enemies die when shot
- **Risk:** Coordinate transforms might be tricky
- **Mitigation:** Use provided helper functions from specs/combat-spec.md

### Sprint 3: Complete Loop (Week 9-12)
- **Story Points:** 5
- **Estimated Hours:** 5-8 hours
- **Goal:** Win/loss conditions, multiple enemies
- **Deliverable:** Fully playable combat prototype
- **Risk:** Balancing (might need extra tuning sprint)
- **Mitigation:** Defer tuning to Sprint 4 if needed

### Optional Sprint 4: Feel Iteration (If Needed)
- **Goal:** Polish and tuning (game juice)
- **Activities:** Screen shake, hit feedback, speed tuning
- **Reference:** See specs/combat-spec.md "Combat Feel (Polish - Step 2)"

**Total Epic:** 21 story points over 3-4 sprints
**Expected Timeline:** 6-12 weeks at 5-10 hours/week

---

## Definition of Done (For All Stories)

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay
- [ ] README updated with build instructions (Sprint 1 only)

---

## Velocity Planning

**Conservative Estimate (5 hours/week):**
- Sprint 1: 2-3 weeks
- Sprint 2: 2-3 weeks
- Sprint 3: 1-2 weeks
- **Total:** 5-8 weeks

**Optimistic Estimate (10 hours/week):**
- Sprint 1: 1-2 weeks
- Sprint 2: 1-2 weeks
- Sprint 3: 1 week
- **Total:** 3-5 weeks

**Recommended Sprint Length:** 2 weeks (allows flexibility with variable weekly hours)

---

## Discussion Points for Review

1. **Velocity Estimation:**
   - Are 4-8 points per 2-week sprint realistic? (assumes 5-10 hours/week)
   - Should we plan for lower velocity initially due to learning curve?

2. **Sprint Length:**
   - 1-week sprints (more frequent check-ins) vs 2-week sprints (more flexibility)?
   - Recommend: 2-week sprints for solo hobby project

3. **Story Breakdown:**
   - Are stories too big/small?
   - Should we break Story 2.2 (Projectiles) into smaller pieces?

4. **Testing Strategy:**
   - Manual playtesting sufficient for prototype?
   - No unit tests for Phase 1? (defer to Phase 2?)

5. **Blocked Scenarios:**
   - What if mouse-aim feels terrible? (Plan B: directional shooting)
   - What if coordinate transforms are too complex? (Simplify to top-down view temporarily)

6. **Scope Creep Prevention:**
   - How to resist adding weapons/enemies mid-sprint?
   - When to say "defer to next epic"?

---

## Suggested First Sprint Commitment

**Sprint 1 (2 weeks, 10-20 hours available):**
- **Commit:** Stories 1.1, 1.2, 1.3 (7 points)
- **Stretch Goal:** Story 1.4 if time permits (+1 point)
- **Focus:** Get comfortable with Rust + ggez before adding complexity

---

## References

- **Combat Specification:** [combat-spec.md](../specs/combat-spec.md)
- **Game Design Document:** [game-design-document.md](../game-design-document.md)
- **Technology Stack:** Rust + ggez (code-first game framework)

---

## Future Epics (Not in Scope)

- **SPRING-002:** Combat Polish & Weapon Variety (shotgun, rifle, melee)
- **SPRING-003:** Economy & Progression (ammunition, currency, upgrades)
- **SPRING-004:** Rescue Mission Loop (objectives, victory rewards)
- **SPRING-005:** Multiple Enemy Types (ranged, fast, bosses)

---

_Epic created: 2025-10-15 for Springfield Meltdown Phase 1 Prototype_
