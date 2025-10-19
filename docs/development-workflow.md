# Development Workflow

**Project:** Springfield Meltdown
**Created:** 2025-10-19
**Purpose:** Standard development and testing protocol for all story implementations

---

## Manual Validation Flow

When implementing stories, follow this flow to ensure quality before marking acceptance criteria as complete:

### 1. Implementation Phase
- Implement code changes per story tasks
- Follow architecture patterns from tech specs
- Handle edge cases and error conditions

### 2. Build Verification
- Run `cargo build` to verify compilation
- Fix any compiler errors or warnings
- Ensure code compiles cleanly before testing

### 3. Manual Validation Request
**CRITICAL:** Dev agent must STOP and ask user to manually validate changes before marking ACs complete.

**Process:**
1. Dev agent lists implemented changes
2. Dev agent provides specific test scenarios for each AC
3. Dev agent asks user to run `cargo run` and validate
4. Dev agent WAITS for user feedback

### 4. User Testing & Feedback
User tests the implementation and reports:
- What works as expected
- What needs adjustment
- Any bugs or issues found
- Performance concerns

### 5. Iteration (if needed)
- Dev agent addresses reported issues
- Return to step 2 (build verification)
- Repeat until user confirms all ACs satisfied

### 6. Completion
**ONLY after user confirms all ACs work:**
- Mark acceptance criteria checkboxes [x]
- Update Implementation Checklist tasks [x]
- Update File List with changed files
- Add entry to Change Log
- Update Dev Agent Record with completion notes
- Set story status to "Ready for Review"

---

## Git Commit Guidelines

### When to Commit
- **After *review-story workflow completes successfully**
- DO NOT commit during *dev-story execution
- Wait for formal review approval before committing

### Commit Message Format
```
<type>: <short summary>

<detailed description if needed>

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Commit Types
- `feat:` New feature implementation (story completion)
- `fix:` Bug fixes
- `refactor:` Code restructuring without behavior change
- `test:` Adding or updating tests
- `docs:` Documentation updates
- `chore:` Build, config, or tooling changes

### Example Commit Messages
```
feat: Implement 8-directional player movement (SPRING-001-1.3)

- Add WASD keyboard input handling
- Normalize diagonal movement vectors
- Apply velocity with delta_time for smooth movement
- Implement screen bounds collision

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

```
fix: Correct movement speed normalization calculation

Diagonal movement was 1.414x faster due to missing normalization.
Added sqrt calculation and vector division to ensure consistent
150 px/sec speed in all 8 directions.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

---

## Story Workflow Summary

```
*dev-story → Implementation → Build → Manual Validation → Iteration (if needed) → Mark Complete
     ↓
*review-story → Review Changes → Approve/Request Changes
     ↓
Git Commit (only after review approval)
     ↓
*story-approved → Mark Story Complete → Advance Queue
```

---

## Quality Gates

Before marking story "Ready for Review":
- [ ] All acceptance criteria validated by user
- [ ] Code compiles without warnings
- [ ] Manual playtesting confirms functionality
- [ ] File List updated with all changes
- [ ] Change Log entry added
- [ ] Dev Agent Record completion notes written

---

_This workflow ensures quality through manual validation before acceptance criteria are marked complete._
