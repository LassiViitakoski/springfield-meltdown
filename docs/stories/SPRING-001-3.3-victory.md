# Story: Multiple Enemies & Victory Condition

**Story ID:** SPRING-001-3.3
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 3
**Story Points:** 1
**Estimated Hours:** 1-2 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT to win when all enemies are defeated
SO THAT I have a clear goal
```

---

## Acceptance Criteria

- [ ] Spawn 5-10 enemies at random positions around map edges
- [ ] Enemies spawn minimum 200px from player
- [ ] Victory triggers when all enemies defeated (enemy_count == 0)
- [ ] "YOU SURVIVED!" text displays in center (large, green font)
- [ ] Press R to restart scene

---

## Technical Notes

- Enemy spawn: random positions at screen edges
- Spawn logic: random_angle * edge_distance + screen_center
- Check enemy_list.len() == 0 for victory
- Victory state similar to death state (freeze gameplay)

---

## References

- [Combat Specification - Victory & Failure Conditions](../specs/combat-spec.md)

---

## Implementation Checklist

- [ ] Implement random enemy spawn positions
- [ ] Spawn 5-10 enemies at map edges
- [ ] Enforce minimum distance from player
- [ ] Check for victory condition (no enemies left)
- [ ] Transition to Victory state
- [ ] Render "YOU SURVIVED!" message
- [ ] Test victory flow and restart

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
