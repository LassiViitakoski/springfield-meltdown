# Story: Camera & Debug UI

**Story ID:** SPRING-001-1.4
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 1
**Story Points:** 1
**Estimated Hours:** 1-2 hours
**Status:** draft

---

## User Story

```
AS A developer
I WANT to see player position and FPS on screen
SO THAT I can debug movement and performance
```

---

## Acceptance Criteria

- [ ] Camera centered on player (simple fixed follow)
- [ ] Debug text displays player position (x, y)
- [ ] Debug text displays current FPS
- [ ] Text renders in top-left corner (white font)
- [ ] Text updates every frame

---

## Technical Notes

- Use ggez::graphics::Text for debug display
- Format: "Pos: (123.4, 567.8) | FPS: 60"
- Camera offset = screen_center - player_screen_pos

---

## References

- [Combat Specification](../combat-spec.md)

---

## Implementation Checklist

- [ ] Implement camera following logic
- [ ] Create debug text rendering function
- [ ] Display player position in UI
- [ ] Display FPS counter
- [ ] Position text in top-left corner
- [ ] Verify text updates each frame

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

_Story created: 2025-10-15_
