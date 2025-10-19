# Story: Mouse Aiming Visualization

**Story ID:** SPRING-001-2.1
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 2
**Story Points:** 2
**Estimated Hours:** 2-4 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT to see where I'm aiming
SO THAT I can understand the shooting direction
```

---

## Acceptance Criteria

- [ ] Mouse position tracked in world coordinates
- [ ] Line drawn from player to mouse cursor (debug visualization)
- [ ] Line color: red, thickness: 2px
- [ ] Line updates every frame as mouse moves
- [ ] Screen-to-world coordinate transform implemented

---

## Technical Notes

- Implement screen_to_world() helper function
- Use ggez::graphics::Mesh::new_line()
- Calculate aim vector: (mouse_world - player_pos).normalize()

---

## References

- [Combat Specification - Mouse Aim System](../specs/combat-spec.md)

---

## Implementation Checklist

- [ ] Implement screen_to_world() coordinate transform
- [ ] Track mouse position in update()
- [ ] Convert mouse screen coords to world coords
- [ ] Calculate aim direction vector
- [ ] Draw line from player to cursor
- [ ] Verify line updates smoothly

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
