# Springfield Meltdown - Game Design Document

**Session Date:** 2025-10-12
**Facilitator:** Business Analyst Mary
**Participant:** Lassi Viitakoski

---

## Executive Summary

**Project Name:** Springfield Meltdown (Working Title)

**Genre:** Real-time Action Roguelite with Rescue Mechanics

**Theme:** The Simpsons - Nuclear Meltdown Scenario

**Core Concept:** Navigate through mutated Springfield, rescue trapped citizens, manage resources strategically, and unlock new playable characters through successful rescue missions.

**Development Philosophy:**
- Start simple, build extensible foundation
- Learn Rust through practical game development
- Progress from basic mechanics to complex AI systems
- Phase-based development with clear milestones

---

## Game Overview

### High-Level Concept

Mr. Burns' nuclear power plant has exploded, covering Springfield in radioactive contamination. The city's residents and wildlife have mutated into hostile creatures. Players must navigate dangerous zones, rescue trapped citizens, and escort them to safety while managing limited resources and permanent character death.

### Core Gameplay Loop

1. **Pre-Mission Phase**
   - Select playable character (unlocked through rescues)
   - Choose 2 weapons from owned arsenal
   - Purchase armor tier (optional, provides HP buffer)
   - Purchase temporary buffs (optional, lost on death/completion)
   - Purchase ammunition for selected weapons

2. **Mission Phase - Journey to Rescue**
   - Navigate procedurally generated contaminated Springfield
   - Combat mutated enemies
   - Manage ammunition and resources
   - Reach trapped citizen location

3. **Rescue & Shop Phase**
   - Rescue citizen (triggers unlock progression)
   - Random shop spawns during return journey
   - Purchase ammunition/supplies for return trip
   - Equip helpers (future feature)

4. **Return Phase**
   - Fight back to safe zone
   - Protect rescued citizen (future escort AI feature)
   - Survive to complete mission

5. **Post-Mission Phase**
   - Earn currency rewards (cash + donuts)
   - Unlock rescued character as playable
   - Spend donuts on permanent upgrades
   - Plan next mission strategy

---

## Theme & Setting

### Story Premise

**Nuclear Meltdown Catastrophe**

Springfield's nuclear power plant has suffered a catastrophic meltdown due to Mr. Burns' negligence and Homer's incompetence. Radiation has spread across the city, mutating citizens, animals, and objects into hostile creatures. The few survivors must rescue trapped family and friends while fighting through the contaminated wasteland that was once their home.

The absurd tone maintains Simpsons humor while justifying combat mechanics through radiation-induced chaos.

### Visual Style

- **Art Direction:** 2.5D isometric perspective with sprite-based graphics
- **Camera View:** Fixed isometric angle (similar to Hades, Bastion)
- **Rendering:** 2D engine with coordinate transforms to create depth illusion
- **Asset Strategy:** Leverage existing Simpsons fan art and sprite resources (non-commercial project)
- **Aesthetic:** Colorful but contaminated - familiar Springfield locations twisted by radiation
- **Technical Note:** World uses 2D coordinates (X/Y), transformed to screen isometric projection with depth sorting

---

## Core Mechanics

### Combat System

**Implementation Status:** See [combat-spec.md](specs/combat-spec.md) for detailed prototype specifications.

**High-Level Design:**
- **Movement:** 8-directional smooth movement in world space, isometric rendering
- **Combat:** Mouse-aimed projectile shooting, responsive controls
- **Enemy AI:** Chase behavior with contact damage
- **Feel:** Fast-paced action with hit feedback and game juice

**Phase 1 Scope:**
- Player movement (WASD, 150 px/sec)
- Pistol weapon (mouse-aimed, projectile-based)
- Radioactive Rat enemy (chase AI, contact damage)
- Basic health system and victory/failure conditions

**Future Additions:**
- Weapon variety (shotgun, rifle, melee)
- Enemy variety (ranged attackers, fast chasers)
- Character-specific combat differences
- Advanced hit detection and feedback systems

### Weapon & Loadout System

**Weapon Acquisition (Permanent Purchases)**
- **Open shop model:** All weapons available from start, differentiated by price
- **High price ceiling:** Basic weapons affordable, best weapons require completing hard missions
- **One-time purchase:** Buy weapon with cash, own permanently
- **Universal pool:** All characters can access owned weapons (size restrictions apply - Maggie can't use rocket launcher)

**Pre-Mission Loadout Selection**
- Choose 2 weapons from owned arsenal (1 primary + 1 secondary)
- Purchase armor tier: Light/Medium/Heavy (cash cost, provides HP buffer)
- Purchase temporary buffs (cash cost, one of each type maximum)
- Purchase ammunition for selected weapons (cash cost)

**Armor System**
- Tiered options: Light (low HP, cheap) → Medium → Heavy (high HP, expensive)
- Provides additional HP buffer on top of character base health
- Lost when depleted or on death

**Temporary Buffs**
- **Purchase limit:** One of each buff type per mission
- **Duration:** Active for entire mission duration
- **Lost on:** Death or mission completion (success or failure)
- **Types (examples):** Speed boost, damage multiplier, fire rate increase, luck boost

**Ammunition Economy**
- Separate ammo types per weapon category (pistol ammo, shotgun shells, rifle rounds, etc.)
- Purchased with cash (consumable resource)
- **Persists between runs:** Not lost on death, only consumed when fired
- Strategic depth: invest ammo budget in favorite weapons

**Character Abilities (Cooldown-Based)**
- Unique abilities per character (to be designed)
- Cooldown-based activation (not ammo-dependent)
- Design deferred until character system implementation

### Shop System

**Shop Placement (Phase 1):**
- One random shop spawns AFTER rescue (during return journey)
- Preserves resource scarcity and management challenge
- Strategic checkpoint for restocking

**Shop Inventory:**
- Ammunition (restock during mission)
- Health packs (consumables)
- Additional armor (if current armor depleted)
- Temporary buffs (if not already active)

**Future Shop Locations:**
- Random Kwik-E-Mart appearances during outbound journey
- Additional shops during return journey

### Economy System

**Dual Currency Design**

**Cash (Common Currency)**
- **Earned:** Enemy drops, completing missions, finding chests
- **Spent on:**
  - Permanent weapon purchases (one-time, own forever)
  - Ammunition (pre-mission and in-mission shop purchases)
  - Armor (pre-mission and in-mission shop purchases, lost on depletion/death)
  - Temporary buffs (pre-mission purchase, lost on death/mission completion)
  - Consumables (health packs during missions)
  - Future: NPC bribes, shortcuts, weapon upgrades

**Donuts (Premium/Rare Currency)**
- **Earned:** Mission completion rewards from Mayor, rare drops, achievements
- **Spent on:**
  - Permanent upgrades (max health, movement speed, damage)
  - Unlock new playable characters
  - Unlock new zones/biomes
  - Meta-progression systems
  - Future: Special abilities, cosmetic options

**Economic Balance Goals:**
- Resource scarcity encourages strategic planning
- Permanent weapon purchases provide tangible progression
- Risk/reward in consumable investments (armor, buffs)
- Ammunition persistence reduces death frustration while maintaining tactical depth

### Rescue Mechanics

**Phase 1: Single Rescue Missions**
- Each mission rescues ONE citizen
- Citizen is stationary/trapped at rescue location
- Successful rescue unlocks them as playable character
- Mayor rewards players with cash + donuts per rescue

**Future Phases:**
- Helper AI system (rescued citizens assist on return journey)
- RNG-based helper capabilities (injured vs healthy)
- Family rescue missions (multiple targets, higher difficulty)
- Trophy system (complete rescue with specific helpers)

---

## Progression Systems

### Character Unlock System

**Progression Path:**
1. Start with single playable character (Homer)
2. Complete rescue mission → unlock that character as playable
3. Each character offers different difficulty/playstyle
4. Extended roster: Simpson family → Springfield citizens

**Character Difficulty Tiers:**
- **Easy:** Homer, Bart (strong combat capabilities)
- **Medium:** Marge (balanced)
- **Hard:** Lisa (support-focused, requires strategy), Maggie (limited weapons)
- **Unlockables:** Apu, Chief Wiggum, Moe, etc.

### Meta-Progression

**Permanent Upgrades (Donut Currency):**
- Max health increases
- Movement speed boosts
- Base damage multipliers
- Starting cash bonuses
- Unlock additional loadout slots (future)
- Unlock special abilities (future)

### Mission Progression

**Phase 1:** Sequential rescue missions
- Linear unlock path through Simpson family
- Increasing difficulty per rescue

**Future Phases:**
- Zone-based progression (unlock new biomes)
- Branching rescue paths (choose who to rescue)
- Difficulty modifiers (challenge modes)
- Endless/arcade modes

---

## Enemy Design

### Enemy Categories

**Mutated Wildlife:**
- Radioactive rats, dogs, cats
- Contaminated birds
- Glowing insects
- Generic threat types (not personality-dependent)

**Mutated Citizens:**
- Former Springfield residents
- Recognizable character silhouettes but twisted
- Varying threat levels

**Environmental Hazards:**
- Radioactive ooze creatures
- Animated contaminated objects
- Glowing blobs and slimes
- Mr. Burns' malfunctioning security systems

**Design Philosophy:**
- Mutation theme allows unlimited enemy variety
- Scalable complexity (simple AI → advanced behaviors)
- Visual distinctiveness for threat recognition

---

## World & Zones

### Springfield Locations (To Be Designed)

**Potential Biomes:**
- Nuclear Power Plant (epicenter)
- Downtown Springfield
- Suburbs (Evergreen Terrace)
- Springfield Elementary
- Kwik-E-Mart district
- Moe's Tavern area
- Springfield Mall

**Zone Differentiation (To Be Defined):**
- Enemy type variations per zone
- Environmental hazards specific to location
- Visual theming and atmosphere
- Difficulty progression

---

## Development Roadmap

### Phase 1: MVP (Foundation)

**Core Systems:**
- Basic player movement and combat
- Simple enemy AI (chase player)
- Single weapon implementation
- One playable character (Homer)
- One zone/biome (Power Plant or simple test area)
- Cash currency and simple shop
- Single rescue mission implementation
- Basic UI (health, ammo, cash display)

**Goals:**
- Learn Rust fundamentals through implementation
- Establish ggez game loop architecture
- Prove core gameplay loop is fun

**Estimated Timeline:** 2-3 months

---

### Phase 2: Content & Economy (Mid-Term)

**Expanded Systems:**
- Full weapon variety (5-7 weapon types)
- Dual currency economy (cash + donuts)
- Multiple playable characters (Simpson family)
- 3-4 distinct zones/biomes
- Procedural generation basics
- Permanent upgrade system (donut-based)
- Simple companion AI (single helper, basic follow/attack)
- Multiple enemy types (8-10 varieties)
- Armor tier system
- Temporary buff system

**Goals:**
- Build content library
- Refine economic balance
- Implement simple AI systems (learning opportunity)
- Create replayability through variety

**Estimated Timeline:** 4-6 months

---

### Phase 3: Advanced Features (Long-Term)

**Complex Systems:**
- Advanced companion AI (multiple helpers, carry/protect mechanics)
- Family rescue missions (multiple targets)
- RNG helper system (injured vs healthy)
- Trophy/achievement system
- Additional Springfield citizens as playables
- Complex procedural generation
- Meta-progression depth
- Difficulty modifiers and challenge modes
- Enemy AI complexity (state machines, emergent behavior)

**Goals:**
- Deep AI implementation (aligns with Master's degree in AI/Data Analytics)
- Long-term engagement systems
- Polish and refinement
- Community feedback integration

**Estimated Timeline:** 6-12+ months (ongoing hobby project)

---

## Technical Considerations

### Technology Stack

**Language:** Rust
- Learning goal: Master Rust ownership, traits, and patterns
- Performance benefits for game logic
- Memory safety for complex systems

**Game Framework:** ggez
- Code-first approach (no heavy editor)
- Trait-based EventHandler pattern
- Good documentation and community
- Scales well with project growth

**Development Tools:**
- VS Code + rust-analyzer
- cargo-watch (live reload)
- Git version control

### Learning Priorities

**Short-Term (Phase 1):**
- Rust fundamentals through game systems
- ggez framework patterns
- Game loop architecture
- Basic state management
- Isometric coordinate transformations (world ↔ screen)
- Depth sorting for sprite rendering

**Mid-Term (Phase 2):**
- Procedural generation algorithms
- Pathfinding (A* or similar)
- Simple AI behaviors
- Data serialization (save systems)
- Performance optimization

**Long-Term (Phase 3):**
- Complex AI systems and state machines
- Multi-agent coordination
- Emergent gameplay systems
- Advanced algorithms (relevant to AI Master's degree)

---

## Design Decisions - Deferred

These decisions are intentionally postponed to later development phases:

### Combat Mechanics
- Exact movement style (twin-stick shooter, hybrid melee/ranged, etc.)
- Weapon behavior specifics
- Enemy engagement patterns
- Hit detection and feedback

### Character Design
- Specific character abilities and stats
- Character differentiation mechanics
- Unique weapon designs
- Starting equipment per character

### Zone Design
- Specific biome layouts and themes
- Zone progression order
- Environmental hazard types
- Visual style per location

### AI Systems
- Companion pathfinding algorithms
- Enemy behavior complexity
- Multi-agent coordination
- State machine architectures

---

## Key Themes & Insights

### Design Pillars

1. **Strategic Resource Management**
   - Dual currency economy creates meaningful choices
   - Ammunition scarcity forces tactical thinking
   - Risk/reward in pre-mission investment

2. **Extensible Foundation**
   - Simple core mechanics with growth potential
   - Modular systems for easy content addition
   - Phase-based development allows learning and iteration

3. **Unique Rescue Mechanic**
   - Differentiates from standard roguelites
   - Natural progression through character unlocks
   - Future AI development opportunities

4. **Learning Through Development**
   - Rust language mastery via practical application
   - AI systems align with academic goals
   - Code-first approach emphasizes engineering skills

### Innovation Points

- **Permanent weapon ownership** provides clear progression while maintaining resource scarcity through consumables
- **Persistent ammunition** reduces death frustration while maintaining tactical depth
- **Post-rescue shop** creates strategic checkpoint decision
- **Rescue-to-unlock progression** naturally gates content
- **Dual currency** separates consumable vs permanent progression
- **Buff system** (one per type, lost on death/completion) encourages strategic pre-mission planning
- **Helper AI future system** provides algorithmic challenge

---

## Next Steps - Prototype-First Development

### Phase 1: Combat Foundation (CURRENT FOCUS)

**Philosophy:** Build playable prototype first, refine systems based on actual gameplay experience.

**Step 1: Minimal Combat Prototype**
- Implement player movement (8-directional in isometric space)
- Add 1 weapon (pistol - simplest case)
- Create 1 enemy type (basic chaser AI)
- Basic collision detection
- Health system for player and enemies

**Step 2: Feel Iteration**
- Polish movement responsiveness
- Add game juice (hit feedback, screen shake, particles)
- Tune combat pacing until it feels good
- Learn Rust + ggez patterns through implementation

**Step 3: Combat Variety**
- Add 2-3 more weapons with distinct behavior
- Add 2-3 enemy types with different patterns
- Implement ammunition system basics
- Simple UI (health, ammo counter)

**Step 4: Basic Rescue Flow**
- Implement "walk to point A, walk back to point B" mission structure
- No escort AI yet - just prove the loop works
- Simple victory/failure states
- Cash reward system (basic economy)

**Step 5: Playtest & Evaluate**
- Does core combat feel fun?
- Is resource management interesting?
- What needs more depth?

### Phase 2: Return to Design Questions (DEFERRED)

After prototype proves core loop is fun, revisit:
- Failure states and death penalties (informed by actual deaths)
- Character unlock economy clarity (informed by progression pacing)
- Safe zone functionality (informed by mission flow experience)
- Character differentiation mechanics (informed by what feels samey)
- Shop placement and economy balancing (informed by resource tension)
- Zone progression and variety (informed by replayability needs)

**Rationale:** Real gameplay experience answers theoretical design questions faster and more accurately than paper design.

---

## Questions for Further Exploration

1. Should helper AI have resource costs (feeding them, healing them)?
2. How do different characters mechanically differ beyond stats?
3. What makes each Springfield zone feel unique beyond visuals?
4. Should there be boss encounters or special mission types?
5. How does difficulty scale across missions and character unlocks?
6. What meta-narrative connects the rescue missions?
7. Should there be time pressure mechanics (radiation spreading)?

---

_Game design consolidated from brainstorming session using BMAD framework_
