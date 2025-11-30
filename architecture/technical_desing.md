# Technical Design (Engine Architecture)

This document outlines system architecture for the client, server, shared lib, and simulation.

# Client

## Rendering (wgpu)

* Isometric camera.
* Tile + entity rendering using texture atlas.
* Shader pipeline is replaceable (moddable).
* All textures live client-side; server sends metadata only.

## ECS (Client-Side)

Lightweight ECS used for:

* Player entity
* NPCs & enemies
* Renderable objects
* Temporary FX (particles, spells)

Client ECS has:

* Components: Position, Velocity, Sprite, Animation, RemotePlayerData
* Systems: MovementPrediction, RenderSystem, InterpolationSystem, AnimationSystem

## Input System

* Reads WASD, Jump, Attack.
* Converts to isometric vectors.
* Sends input packets to server with timestamps.
* Immediately applies predicted movement.

## Multiplayer Interpolation

* Maintain history buffer of server snapshots.
* Render at (serverTime - interpolationDelay).
* Linearly interpolate (lerp) between known positions.
* Ensures remote players move smoothly.

## Asset Management

* Texture atlas loaded on startup.
* Server only specifies:

  * Entity type
  * Tile type
  * Sprite ID
* Client maps those to textures/shaders.

---

# Server

## Simulation Model

* Single-threaded or multi-threaded ECS depending on scale.
* 60 ticks/sec.
* Each tick:

  1. Process queued inputs.
  2. Update movement.
  3. Run physics/collision.
  4. Update NPCs/AI.
  5. Update world interactions.
  6. Produce snapshots to send to clients.

## Entity State

Entities have server-side components:

* Position
* Velocity
* Collider
* Health
* Mana/Stamina
* Inventory
* CombatState
* NPC AI State

## TileManager

* Chunked world structure.
* Chunk size: 33x33 (center + surrounding 16).
* Quick lookup for collisions and world interactions.
* Caches active chunks.
* Saves dirty chunks (double-buffered).

## Networking (QUIC: using “quinn”)

* Reliable streams:

  * Chunk requests
  * Inventory updates
  * Tile edits
  * Login / auth
* Datagrams:

  * Movement inputs
  * Combat actions
  * Snapshot updates (positions, velocities)

## Commands + Validation

Server validates:

* Movement legality
* Tile modification rules
* Combat hit checks
* Crafting requirements
* Inventory changes

---

# Shared Library (crate)

Contains:

* Messaging protocol (bit-packed or custom binary schema)
* Component definitions (shared structs)
* Snapshot formats
* Input packet formats
* Compression helpers (delta compression, coordinate packing)
