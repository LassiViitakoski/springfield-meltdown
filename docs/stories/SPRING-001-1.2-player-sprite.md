# Story: Player Entity & Sprite Rendering

**Story ID:** SPRING-001-1.2
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 1
**Story Points:** 2
**Estimated Hours:** 2-3 hours
**Status:** draft

---

## User Story

```
AS A player
I WANT to see my character on screen
SO THAT I know where I am in the game world
```

---

## Acceptance Criteria

- [ ] Player struct created with position (x, y)
- [ ] Player renders as colored circle (placeholder sprite)
- [ ] Player spawns at center of screen (400, 300)
- [ ] World-to-screen coordinate transform implemented
- [ ] Player sprite drawn at correct isometric position

---

## Technical Notes

- Use ggez::graphics::Mesh::new_circle() for placeholder
- Implement world_to_screen() helper function
- Player color: yellow (#FFD700)
- Radius: 16 pixels

---

## References

- [Combat Specification - Coordinate System](../combat-spec.md)

---

## Implementation Checklist

- [ ] Create Player struct with position field
- [ ] Initialize player at screen center
- [ ] Implement world_to_screen() coordinate transform
- [ ] Create circle mesh for player sprite
- [ ] Render player in draw() method
- [ ] Verify player appears at correct position

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay

---

## Dev Agent Record

### Context Reference
- [story-context-SPRING-001.SPRING-001-1.2.xml](story-context-SPRING-001.SPRING-001-1.2.xml) - Generated 2025-10-18

---

_Story created: 2025-10-15_
