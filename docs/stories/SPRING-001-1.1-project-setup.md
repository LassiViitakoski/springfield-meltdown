# Story: Project Setup & Window Rendering

**Story ID:** SPRING-001-1.1
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 1
**Story Points:** 2
**Estimated Hours:** 2-4 hours
**Status:** Ready

---

## User Story

```
AS A developer
I WANT a working ggez project with window rendering
SO THAT I have a foundation to build the game on
```

---

## Acceptance Criteria

- [ ] Cargo project created with ggez dependency
- [ ] 800x600 window opens with black background
- [ ] Window has title "Springfield Meltdown - Prototype"
- [ ] Game loop runs at 60 FPS
- [ ] Basic EventHandler trait implemented (update/draw)

---

## Technical Notes

- Use ggez 0.9.x (check latest stable)
- Set up git repository with .gitignore
- Create basic main.rs with GameState struct
- Cargo.toml dependencies: ggez = "0.9"

---

## References

- [Combat Specification](../combat-spec.md)
- [Game Design Document](../game-design-document.md)

---

## Implementation Checklist

- [ ] Create Cargo project
- [ ] Add ggez dependency
- [ ] Implement GameState struct
- [ ] Implement EventHandler trait (update/draw methods)
- [ ] Configure window settings (800x600, title)
- [ ] Verify 60 FPS game loop
- [ ] Initialize git repository
- [ ] Add .gitignore (target/, Cargo.lock)
- [ ] Commit initial setup

---

## Definition of Done

- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [ ] No known bugs blocking gameplay
- [ ] README updated with build instructions

---

_Story created: 2025-10-15_
