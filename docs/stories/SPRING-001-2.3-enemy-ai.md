# Story: Enemy Entity & Chase AI

**Story ID:** SPRING-001-2.3
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 2
**Story Points:** 2
**Estimated Hours:** 2-4 hours
**Status:** draft

---

## User Story

```
AS A game designer
I WANT an enemy that chases the player
SO THAT there is a threat to avoid
```

---

## Acceptance Criteria

- [ ] Enemy struct created with position and HP
- [ ] Enemy renders as green circle (24px diameter)
- [ ] Enemy spawns at fixed position (200, 200)
- [ ] Enemy moves toward player at 80 px/sec every frame
- [ ] Enemy AI calculates direction: (player_pos - enemy_pos).normalize()

---

## Technical Notes

- Enemy struct: { pos: Vec2, hp: i32, speed: f32 }
- Chase AI: velocity = direction * speed * delta_time
- HP = 30 (will be used in next story)
- Green color: #00FF00

---

## References

- [Combat Specification - Enemy #1: Radioactive Rat](../specs/combat-spec.md)

---

## Implementation Checklist

- [ ] Create Enemy struct
- [ ] Add enemy to GameState
- [ ] Spawn enemy at initial position
- [ ] Implement chase AI logic
- [ ] Calculate direction vector to player
- [ ] Apply velocity to enemy position
- [ ] Render enemy sprite
- [ ] Test enemy follows player correctly

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
