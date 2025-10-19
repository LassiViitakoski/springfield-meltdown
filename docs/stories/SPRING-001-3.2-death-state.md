# Story: Death State & Game Over

**Story ID:** SPRING-001-3.2
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 3
**Story Points:** 2
**Estimated Hours:** 2-3 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT the game to end when I die
SO THAT failure has consequences
```

---

## Acceptance Criteria

- [ ] Game freezes when player HP reaches 0
- [ ] "YOU DIED" text displays in center of screen (large, red font)
- [ ] Press R to restart scene (resets player, enemies, projectiles)
- [ ] Player and enemy entities stop updating during death state
- [ ] No penalties or progression loss (prototype mode)

---

## Technical Notes

- Add GameState enum: Playing, PlayerDead, Victory
- Check state before updating entities
- Restart: reset all entities to initial spawn positions/values

---

## References

- [Combat Specification - Victory & Failure Conditions](../specs/combat-spec.md)

---

## Implementation Checklist

- [ ] Create GameState enum (Playing, PlayerDead, Victory)
- [ ] Check player HP and transition to PlayerDead state
- [ ] Stop entity updates during death state
- [ ] Render "YOU DIED" message
- [ ] Implement restart functionality (R key)
- [ ] Reset all game entities on restart
- [ ] Test death and restart flow

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
