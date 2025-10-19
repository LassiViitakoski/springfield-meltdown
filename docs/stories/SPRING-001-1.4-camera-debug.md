# Story: Camera & Debug UI

**Story ID:** SPRING-001-1.4
**Epic:** [SPRING-001 - Combat Prototype Foundation](../epics/SPRING-001-combat-prototype.md)
**Sprint:** Sprint 1
**Story Points:** 1
**Estimated Hours:** 1-2 hours
**Status:** Done

---

## User Story

```
AS A developer
I WANT to see player position and FPS on screen
SO THAT I can debug movement and performance
```

---

## Acceptance Criteria

- [x] Camera centered on player (simple fixed follow)
- [x] Debug text displays player position (x, y)
- [x] Debug text displays current FPS
- [x] Text renders in top-left corner (white font)
- [x] Text updates every frame

---

## Technical Notes

- Use ggez::graphics::Text for debug display
- Format: "Pos: (123.4, 567.8) | FPS: 60"
- Camera offset = screen_center - player_screen_pos

---

## References

- [Combat Specification](../specs/combat-spec.md)

---

## Implementation Checklist

- [x] Implement camera following logic
- [x] Create debug text rendering function
- [x] Display player position in UI
- [x] Display FPS counter
- [x] Position text in top-left corner
- [x] Verify text updates each frame

---

## Definition of Done

- [x] Code compiles without warnings (isometric constants reserved for future use)
- [x] Manual playtesting confirms acceptance criteria
- [ ] Code committed to git with descriptive message
- [x] No known bugs blocking gameplay

---

## Dev Agent Record

### Context Reference
- Story Context XML: [story-context-SPRING-001.SPRING-001-1.4.xml](story-context-SPRING-001.SPRING-001-1.4.xml)

### Debug Log
**Implementation Approach:**
- Implemented true camera centering: player always renders at screen center (400, 300)
- Camera offset calculated as: `screen_center - player.pos`
- World bounds box (600x400) now moves with camera to create proper world-movement illusion
- Debug UI uses ggez::graphics::Text with format: "Pos: (x, y) | FPS: fps"
- Text positioned at (10, 10) top-left with white color
- FPS from ctx.time.fps(), position shows player's world coordinates
- Updates automatically every frame in draw() function

**Screen Coordinate System:**
- Y-axis increases downward (graphics convention)
- Origin (0,0) at top-left of screen
- Player world position: negative Y = above origin, positive Y = below origin
- This differs from mathematical Cartesian coordinates (Y-up)

**Camera Design:**
- Fixed camera (no smooth lerp) - player stays perfectly centered
- World space rendered relative to camera offset
- Green debug box demonstrates world-relative rendering

**Performance Discussion:**
- Documented rendering optimization strategy with user
- Phase 1 (current): Naive rendering for <50 entities
- Phase 2: Manual viewport culling for 50-100 entities
- Phase 3: Spatial partitioning grid for 200+ entities
- Summary prepared for Game Architect review

### File List
- `src/main.rs` - Modified draw() function: added camera offset calculation, world-relative boundary box rendering, debug UI text display

### Change Log
- **2025-10-19**: Implemented camera-centered rendering with player fixed at screen center. Added debug UI displaying player world position and FPS in top-left corner. World boundary box now renders relative to camera for proper movement illusion.
- **2025-10-19**: Senior Developer Review notes appended. Review outcome: Approve. 2 optional low-priority enhancement suggestions documented.

### Completion Notes
All acceptance criteria validated via manual testing:
- Camera keeps player centered on screen at (400, 300)
- Debug text displays real-time position and FPS
- White text at top-left corner, readable on black background
- World boundary box moves with camera creating proper game feel
- Screen coordinate system (Y-down) confirmed and explained to user
- Performance optimization strategy documented for future architect consultation

---

## Senior Developer Review (AI)

**Reviewer:** Lassi Viitakoski
**Date:** 2025-10-19
**Outcome:** **Approve**

### Summary

Story SPRING-001-1.4 successfully implements camera centering and debug UI functionality. The implementation correctly keeps the player fixed at screen center while applying camera offset to world objects, creating proper camera-follow illusion. Debug text displays player position and FPS as specified. Code quality is excellent for a prototype - clean, well-commented, and follows Rust best practices. All acceptance criteria validated through manual testing.

### Key Findings

**Strengths:**
- ✅ Camera system correctly implemented with proper offset calculation (lines 118-124)
- ✅ World-relative rendering for boundary box creates convincing movement illusion (lines 126-147)
- ✅ Debug UI formatting matches specification exactly: "Pos: (x, y) | FPS: fps" (lines 166-178)
- ✅ Excellent code documentation explaining coordinate system and camera design
- ✅ Performance discussion documented for future architect review

**Minor Observations:**
- **Low Priority**: Unused isometric constants (TILE_WIDTH_HALF, TILE_HEIGHT_HALF) and world_to_screen() function. These are intentionally reserved for future stories per tech spec.
- **Recommended**: Consider extracting camera offset calculation into a helper method when adding more entities (Sprint 2) to avoid duplication.

### Acceptance Criteria Coverage

| AC | Status | Evidence |
|----|--------|----------|
| Camera centered on player | ✅ Pass | Player renders at fixed screen_center (line 150), camera_offset applied to world objects (lines 121-124) |
| Debug text displays position | ✅ Pass | Format string displays player.pos.x and player.pos.y with 1 decimal precision (line 169) |
| Debug text displays FPS | ✅ Pass | ctx.time.fps() called and displayed with 0 decimal precision (lines 167, 169) |
| Text in top-left, white font | ✅ Pass | Text positioned at (10, 10), Color::WHITE applied (lines 174-177) |
| Text updates every frame | ✅ Pass | Debug text rendered in draw() function, executed every frame by ggez event loop |

**Coverage:** 5/5 acceptance criteria met (100%)

### Test Coverage and Gaps

**Manual Testing:** All ACs validated through cargo run playtesting per development-workflow.md protocol.

**Test Evidence:**
- Camera centering: Player stays at (400, 300) while world boundary box moves
- Position display: Coordinates update dynamically during WASD movement
- FPS display: Shows ~60 FPS during gameplay
- Visual confirmation: White text visible in top-left corner
- Frame updates: Position values change smoothly without lag

**Test Gaps:** None for prototype phase. Automated tests deferred to Phase 2 per tech spec strategy.

### Architectural Alignment

**✅ Compliant** with tech spec SPRING-001:
- Camera offset calculation matches spec formula: `screen_center - player_screen_pos` (lines 121-124)
- Debug UI uses ggez::graphics::Text as specified (line 173)
- Rendering follows EventHandler pattern (draw() implementation)
- Screen coordinate system correctly implements Y-down convention
- World-relative rendering pattern established for future entities

**Design Patterns:**
- Camera system uses "fixed camera with world offset" pattern (correct for this AC)
- Separation of concerns: camera logic in draw(), not mixed with update()
- Comments clearly explain coordinate system (lines 118-124, 126-132)

### Security Notes

**N/A** - Offline single-player game prototype. No security concerns for this story.

**Input Validation:** ctx.time.fps() returns f64, no user input processed in this story. Safe.

### Best-Practices and References

**Rust Best Practices:**
- ✅ Follows Rust 2021 edition idioms
- ✅ Proper use of ggez 0.9 API patterns ([ggez docs](https://docs.rs/ggez/0.9.0/ggez/))
- ✅ Clear variable naming (screen_center, camera_offset, world_bounds_*)
- ✅ Comments explain "why" not just "what" (lines 118-120, 126-128)

**Game Development Patterns:**
- ✅ Camera-follow pattern correctly implemented
- ✅ World-relative rendering establishes foundation for entity management
- ✅ Debug UI provides essential development visibility

**Coordinate System Documentation:**
- Excellent inline explanation of screen Y-down convention vs mathematical Y-up
- Dev Agent Record documents coordinate system for future reference
- User education provided during development (documented in completion notes)

### Action Items

**Optional Enhancements (Low Priority):**

1. **[Optional][Low] Extract camera offset to helper method**
   - **Reason:** When adding enemies/projectiles (Sprint 2), camera offset will be reused. Extract to avoid duplication.
   - **Suggested approach:**
     ```rust
     fn calculate_camera_offset(&self) -> Vector2<f32> {
         Vector2 {
             x: 400.0 - self.player.pos.x,
             y: 300.0 - self.player.pos.y,
         }
     }
     ```
   - **File:** src/main.rs:114-124
   - **Timing:** Address when implementing Story 2.1 (mouse aiming)

2. **[Optional][Low] Consider adding screen dimension constants**
   - **Reason:** Hardcoded 400.0, 300.0 appear multiple times. Consider SCREEN_CENTER constant.
   - **Suggested approach:**
     ```rust
     const SCREEN_WIDTH: f32 = 800.0;
     const SCREEN_HEIGHT: f32 = 600.0;
     const SCREEN_CENTER: Vector2<f32> = Vector2 { x: 400.0, y: 300.0 };
     ```
   - **File:** src/main.rs (top-level constants)
   - **Timing:** Refactor when modularizing code (future sprint)

**No Blocking Issues** - Story approved for completion.

---

_Story created: 2025-10-15_
