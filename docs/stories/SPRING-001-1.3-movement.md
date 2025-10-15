# Story: 8-Directional Player Movement

**Story ID:** SPRING-001-1.3
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 1
**Story Points:** 3
**Estimated Hours:** 3-5 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT to move my character with WASD keys
SO THAT I can navigate the game world
```

---

## Acceptance Criteria

- [ ] WASD keys move player in 8 directions (up, down, left, right, diagonals)
- [ ] Movement speed = 150 pixels/second
- [ ] Diagonal movement normalized (same speed as cardinal directions)
- [ ] Movement is smooth and responsive
- [ ] Player cannot leave screen bounds (collision with edges)

---

## Technical Notes

- Use ggez keyboard input API
- Normalize velocity vector when length > 0
- Apply movement: pos += velocity * speed * delta_time
- Screen bounds check: clamp position to (0, 0) → (800, 600)

---

## References

- [Combat Specification - Player Movement](../combat-spec.md)

---

## Implementation Checklist

- [ ] Add velocity field to Player struct
- [ ] Implement keyboard input handling (WASD)
- [ ] Calculate movement vector from input
- [ ] Normalize diagonal movement vectors
- [ ] Apply velocity with delta_time multiplier
- [ ] Implement screen bounds collision
- [ ] Test all 8 movement directions
- [ ] Verify movement speed consistency

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
