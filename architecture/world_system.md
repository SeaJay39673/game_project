# World System & Persistence

## World Structure

The world is:

* Infinite in X/Y tile directions.
* Height-based (Z axis).
* Split into **33×33 tile chunks**:

  * Center tile + 16 outward in each direction.

## Tile Data

Each tile stores:

* Type (stone, grass, water, wood, etc.)
* Height
* Metadata (durability, wetness, biome, flags)
* Entity references (plants, decorations)

## Chunk Lifecycle

* Only chunks near players are “active”.
* LRU cache controls chunk loading/unloading.
* On load → read from disk.
* On unload → save (double-buffered).

---

# Persistence Strategy

## Save Format Options

Use one of:

* CBOR (binary JSON-like)
* MessagePack
* Custom binary schema

Chunk files:

```
/world/saves/seed/chunk_x_y.bin
```

## Double-buffered Writes (required)

Procedure for saving chunk (safe):

1. Serialize chunk into memory.
2. Write to `chunk_x_y.tmp`.
3. Validate file size + checksum.
4. Atomically rename:

```
chunk_x_y.tmp → chunk_x_y.bin
```

5. Delete `.tmp` on startup if leftover.

Benefits:

* No corruption risk.
* Works even on power loss.
* Atomic replacement is fast.

---

# World Generation

Initial terrain:

* Heightmap from Perlin noise or simplex.
* Later biome-specific generators:

  * Forests
  * Lakes
  * Mountains
  * Magical zones
* WFC planned for structures/villages.

## Tile Interactions

Server handles:

* Block placement/removal.
* Water spreading (heuristic rules).
* Tile physics (stability, slopes).
* Trigger entities (doors, switches, etc.).

---

# Entity–World Interaction

* Collision checked against tile colliders.
* World acts as an ECS resource:

  * query tile at (x,y,z)
  * check neighbors
  * get height at position

---

# Autosave System

* Every 5 minutes (configurable).
* Saves only dirty chunks.
* Performs one chunk save per tick to avoid hitching.
* Uses double-buffering for safety.
