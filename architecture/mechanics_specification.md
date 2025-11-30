# Gameplay Mechanics Specification

This document defines gameplay rules and systems, independent of implementation.

## Movement

* Free movement in isometric space (no tile snapping).
* World is tile-based but entity movement is continuous (Vec3).
* Movement mapped to isometric axes (WASD → ±X, ±Y transforms).
* Server authoritative:

  * Client predicts movement immediately.
  * Server validates movement + collisions.
  * Client reconciles on correction.
* Other players are interpolated from snapshot updates.
* Tile height determines movement constraints:

  * Cliffs/walls block.
  * Slopes/stairs allow traversal.
  * Jumping may allow small height differences.

## Tile Interaction

* Players can place/remove tiles.
* Solid tile rules:

  * Placing over water replaces the water tile.
  * Placing on top of solid adds a new layer.
* Water rules:

  * Water placed on solid tile becomes a water tile at that location (height-based).
  * Digging at water edges can trigger simple spreading logic (heuristic system TBD).

## Inventory & Items

* Finite inventory.
* Bags of holding (upgradeable).
* Item types:

  * Resources (wood, ore, herbs, pelts)
  * Crafted tools
  * Weapons/armor components
  * Spell components/sigils
  * Potions
  * Food

## Crafting

* Skill-based crafting types:

  * Cooking
  * Sewing
  * Smithing
  * General crafting (tools, tiles)
  * Writing (spells)
* Recipes discovered via:

  * Finding
  * Trading
  * Skill leveling
  * Experimentation (optional feature)

## Progression

### Stats

* Health
* Stamina
* Mana
  Stats increase with XP or specific trainers.

### Perks

Perks unlock based on achievements, usage, or level. Examples:

* +10% spell recovery speed
* +5% damage resistance in cloth armor
* Faster gathering speed
* Extra inventory slots

## Combat

* Real-time combat.
* Hit detection is server authoritative.
* Magic:

  * Construct spells from sigils.
  * Arrange spells in spellbooks.
  * Spellbooks can have modifiers, slots, or affinities.

## Multiplayer

* Default max players per world: 5.
* Worlds save every 5 minutes.
* Sync model:

  * QUIC reliable streams for requests (inventory, map, chunks)
  * QUIC datagrams for movement/combat snapshots
