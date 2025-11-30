# Task Breakdown Document

This converts the GDD into concrete actionable tasks for development.

---

# 1. High-Level Project Areas

* Rendering Engine (wgpu)
* ECS Framework
* Networking (QUIC)
* Server Simulation (60 Hz)
* World/Chunk System
* Player Systems (movement, combat)
* Items & Crafting
* NPCs/AI
* Persistence
* UI/UX
* Tools & Build Pipeline

---

# 2. Feature → System → Task Breakdown

## A) Rendering Engine

### Systems

* Camera system (isometric)
* Tile rendering
* Entity rendering
* Animation system
* Shader pipeline
* Texture atlas system

### Tasks

* Create orthographic projection for iso view
* Implement tile renderer
* Implement sprite renderer
* Add animation frame controller
* Build texture atlas + loader
* Create GPU buffers for tiles/entities
* Implement render layering (Z-order fix)

---

## B) ECS Framework

### Systems

* Component storage
* Systems scheduler
* Serialization (server-side)
* Event system

### Tasks

* Build ECS storage (sparse sets or archetypes)
* Implement system registration
* Build event queue
* Implement component serialization for network transfer

---

## C) Networking (QUIC)

### Systems

* Reliable streams
* Datagrams
* Input → server pipeline
* Snapshot distribution

### Tasks

* Set up QUIC server using `quinn`
* Implement client handshake
* Implement reliable stream for world/chunk transfers
* Implement datagram channel for movement
* Define binary protocol for snapshots
* Input buffer + timestamping
* Prediction & reconciliation logic

---

## D) Server Simulation (60 t/s)

### Systems

* Tick scheduler
* Physics + collision
* Player command queue
* NPC AI executor
* Snapshot generator

### Tasks

* Implement tick thread
* Create command queue for inputs
* Implement physics step
* Implement collision checks against tiles
* Generate compact snapshot packets
* Add lag compensation hooks (future)

---

## E) World System

### Systems

* Chunk loader/unloader
* Terrain generation
* Tilemanager
* Heightmap generation
* Water spreading module

### Tasks

* Chunk indexing scheme
* Perlin noise heightmap
* Save/load chunk
* LRU chunk cache
* Tile collision flags
* Water spread step system

---

## F) Player Systems

### Systems

* Movement
* Stats
* Inventory
* Combat
* Spellcasting

### Tasks

* Movement validator
* Client movement predictor
* Inventory container system
* Combat hit detection
* Spellbook system
* Resource gathering tools

---

## G) Crafting System

### Systems

* Recipes
* Crafting stations
* Materials
* Item metadata

### Tasks

* Implement recipe registry
* Add crafting UI
* Create basic items
* Material quality tiers

---

## H) NPC/AI System

### Systems

* Behavior tree or state machine
* Wander/Patrol
* Combat AI
* Trading interactions

### Tasks

* Define NPC component schema
* Implement simple action planner
* Add trader interaction logic
* Add hostile enemy with chase behavior

---

## I) Persistence

### Systems

* Chunk saving
* Player save
* Double-buffering

### Tasks

* Serialize chunk structure
* Write .tmp → atomic rename system
* Save inventory
* Save player position

---

## J) UI/UX

### Systems

* Inventory UI
* Crafting UI
* Dialogue bubbles
* Minimap (optional)
* Settings menu

### Tasks

* UI framework selection
* Inventory slots
* Tooltip system
* Resize-safe UI layouts
* Controller support?

---

# 3. Milestones

## **Milestone 0.1 — “World Online”**

* Render tiles
* Player moves locally
* Basic chunk loading
* QUIC connection
* Server accepts client
* Basic ECS

## **Milestone 0.2 — “Authoritative Movement”**

* Server-side movement
* Client prediction
* Reconciliation
* Snapshot interpolation
* Tile collisions

## **Milestone 0.3 — “Basic World Interaction”**

* Place/remove tiles
* Inventory prototype
* Save/load chunks
* Autosave

## **Milestone 0.4 — “Combat Prototype”**

* Basic enemy
* Hit detection server-side
* Damage + health

## **Milestone 0.5 — “Magic Prototype”**

* Spell sigil system
* Basic spellbook
* Simple projectile spell

## **Milestone 0.6 — “Crafting & Progression”**

* Recipe system
* Stats + perks
* Tool usage

## **Milestone 0.7 — “NPCs & Trading”**

* Trader NPC
* Basic AI
* World spawns

## **Milestone 0.8 — “Polish Pass 1”**

* UI refinements
* Bug fixes
* Performance optimizations

## **Milestone 1.0 — “Playable Alpha”**

* Full loop: explore → gather → craft → fight → progress
* 4–5 biomes
* Stable multiplayer
