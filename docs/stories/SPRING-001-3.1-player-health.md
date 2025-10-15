# Story: Player Health & Enemy Damage

**Story ID:** SPRING-001-3.1
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 3
**Story Points:** 2
**Estimated Hours:** 2-3 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT to take damage when enemies touch me
SO THAT combat has risk
```

---

## Acceptance Criteria

- [ ] Player has HP (max 100, starts at 100)
- [ ] Enemy collision deals 5 damage to player
- [ ] Enemy attack has 1-second cooldown (can't damage constantly)
- [ ] HP displayed in UI: "HP: 85/100"
- [ ] HP text turns red when below 30%

---

## Technical Notes

- Player struct gains: { hp: i32, max_hp: i32 }
- Enemy struct gains: { last_attack_time: f32 }
- Collision check: distance < (player_radius + enemy_radius)
- Check time_since_last_attack > 1.0 before dealing damage

---

## References

- [Combat Specification - Player Health](../combat-spec.md)

---

## Implementation Checklist

- [ ] Add HP fields to Player struct
- [ ] Initialize player HP to 100
- [ ] Add last_attack_time to Enemy struct
- [ ] Implement player-enemy collision detection
- [ ] Apply damage with cooldown timer
- [ ] Display HP in debug UI
- [ ] Color HP text red when below 30%
- [ ] Test damage and cooldown mechanics

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
