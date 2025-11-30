# Roadmap

Long-term development timeline (flexible but structured).

---

# Phase 1 — Foundations

## v0.1 — Rendering + Basic World

* Tile rendering
* Camera movement
* Basic ECS
* Chunk loading (local only)

## v0.2 — Networking Base

* QUIC handshake
* Reliable streams + datagrams
* Input → server round trip
* Server tick loop

## v0.3 — Authoritative Movement

* Local prediction
* Reconciliation
* Remote player interpolation

---

# Phase 2 — Core Mechanics

## v0.4 — World Interaction

* Tile placement/destruction
* Resource collection
* Basic inventory

## v0.5 — World Persistence

* Chunk saving/loading
* Double-buffered writes
* Autosave system

## v0.6 — Basic Combat

* Enemy spawner
* Melee combat
* Server authoritative hits

---

# Phase 3 — Systems Expansion

## v0.7 — Crafting & Items

* Crafting recipes
* Stations
* Tool durability
* Armor/weapons prototype

## v0.8 — Magic System

* Sigils
* Spellbook construction
* Projectile spells
* Mana + recharge

---

# Phase 4 — World Expansion

## v0.9 — NPCs & AI

* Traders
* Simple AI (wander/chase)
* NPC inventory & trading

## v0.10 — Biomes & Features

* Lakes
* Mountains
* Forests
* Procedural landmarks

---

# Phase 5 — Polishing & Launch

## v0.11 — UI/UX Overhaul

* Inventory UI
* Crafting UI
* Settings UI

## v0.12 — Performance & Stability

* Batch rendering
* Snapshot compression
* Server optimizations

## v1.0 — Public Playtest

* Fully loop-complete
* Stable multiplayer
