# Networking & Multiplayer Design

## Transport: QUIC

Using QUIC provides:

* UDP-grade speed (low latency)
* TCP-like reliability where needed
* Built-in TLS
* Multiple streams + datagrams
* No head-of-line blocking

This replaces a TCP + UDP dual stack.

---

# Model Overview

## Server Authoritative Simulation

* Server runs the world at 60 ticks/sec.
* Client sends inputs and predicts movement locally.
* Server validates inputs → sends authoritative states.
* Clients reconcile if predictions diverge.

## Client-side Prediction

Predict:

* Movement
* Facing direction
* Animation triggers

Do NOT predict:

* Tile modifications
* Combat hits
* Inventory crafting outcomes
* NPC behaviors

## Reconciliation

Client stores input history:

* input_id, timestamp, movement vector

When receiving a snapshot:

1. Snap to authoritative position.
2. Reapply unprocessed inputs.
3. Smooth out corrections to avoid jitter.

---

# Player Interpolation

For **remote players**, not the local player:

* Maintain snapshot history `N` milliseconds behind server time.
* For each frame, find:

  * State A at t₀
  * State B at t₁
  * Render time t (t₀ ≤ t ≤ t₁)
* Interpolate position using:

```
pos = lerp(A.position, B.position, alpha)
```

Interpolation delay target: **100–150 ms** (tunable).

---

# Snapshot System

Server generates snapshots each tick (60 Hz):

* position
* velocity
* rotation
* animation state
* basic combat state (attacking/charging)
* tile deltas (if changed)
* chunk deltas (rare)

Snapshots are compressed and sent via QUIC datagrams.

---

# Protocol Format

Binary format using:

* Bit-packing for booleans + small ints.
* Delta compression for unchanged fields.
* Quantization to reduce payload:

  * Position stored as int16 or int24 per dimension.
  * Velocities quantized.
  * Angles encoded in 1 byte (0–255).
