# Server Tick Architecture Diagram (Text Format)

Below is the ideal 60 Hz server simulation loop architecture.

---

# High-Level Loop (runs every 16.66 ms)

```
tick() {
    collect_inputs();
    process_inputs();
    physics_step();
    collision_step();
    world_step();
    npc_ai_step();
    combat_step();
    generate_snapshots();
    send_snapshots();
}
```

---

# Detailed Breakdown

## 1. Input Collection

```
input_queue ← read all client datagrams
timestamp reorder (if needed)
```

## 2. Input Processing

```
for each player:
    apply movement commands
    clamp velocity
```

## 3. Physics Step

```
update positions: pos += vel * dt
gravity, slopes, jumping
apply friction
```

## 4. Collision Step

```
player vs tiles
player vs entities
resolve penetration
```

## 5. World Step

```
tile updates (water spreading, etc.)
resource nodes (regen timers)
environment effects
```

## 6. NPC AI Step

```
state machines or behavior trees
wander/patrol/chase
update NPC actions
```

## 7. Combat Step

```
process attacks
hit detection
apply damage
calculate knockback
```

## 8. Snapshot Build

```
snapshot = {
    positions,
    velocities,
    animations,
    tile deltas,
    NPC states
}
```

## 9. Snapshot Send

```
send via QUIC datagram to all clients
```

This is the authoritative loop the clients must predict against.
