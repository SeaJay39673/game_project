# Networking Protocol Diagram (Text Format)

Diagram of the data flow between Client ↔ Server.

---

## 1. Connection Setup

Client → Server:

```
[QUIC Handshake]
  - Client Hello
  - Authentication (optional)
  - Stream negotiation
```

Server:

```
Allocates session ID
Opens movement datagram channel
Opens reliable world-data stream
```

---

## 2. Continuous Data Flow

### CLIENT → SERVER

```
┌─────────────────────────────────────────────────────┐
│                   Input Datagram                    │
│  { input_id, timestamp, movement_vector }           │
└─────────────────────────────────────────────────────┘
                (60–120 messages/sec)

┌─────────────────────────────────────────────────────┐
│               Reliable Commands Stream              │
│  Inventory actions                                  │
│  Tile modification requests                         │
│  Crafting requests                                  │
└─────────────────────────────────────────────────────┘
                  (reliable stream)
```

---

### SERVER → CLIENT

```
┌─────────────────────────────────────────────────────┐
│                  Snapshot Datagram                  │
│  { entity_positions, velocities, animations }       │
└─────────────────────────────────────────────────────┘
                 (60 messages/sec)

┌─────────────────────────────────────────────────────┐
│                Chunk Data Stream                    │
│  Large chunk transfers (tile data)                  │
└─────────────────────────────────────────────────────┘
                 (reliable stream)
```

---

## 3. Client Processing Pipeline

```
Input → Predict locally
     → Send to server
     ← Receive authoritative snapshot
     → Reconcile local player
     → Interpolate remote players
     → Render
```

---

## 4. Server Tick Loop

```
Receive Inputs
→ Process Inputs
→ Run Physics
→ Run AI
→ Update World
→ Build Snapshot
→ Broadcast Snapshot
```

This diagram maps directly to the networking & multiplayer design doc.
