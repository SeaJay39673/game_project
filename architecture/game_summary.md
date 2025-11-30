# Game Summary

## Overview

This project is a multiplayer, isometric, tile-based open world survival game. The design goal is to create a cozy but deep sandbox with combat, magic, crafting, exploration, and persistent world state. The entire experience emphasizes player freedom: building, fighting, trading, and customizing gameplay styles within a procedural, infinite world.

The project is built in Rust with minimal external libraries. Rendering uses `wgpu`, networking uses QUIC, and the simulation is server-authoritative with client-side prediction.

## Core Experience

Players explore an infinite isometric world composed of chunked tiles of varying heights. They gather resources, craft equipment, build structures, fight enemies, and progress through perks, stats, and magical abilities. Cooperative multiplayer (default max 5 players) is supported.

## Core Loop (First Pass)

1. Explore the world (biomes, lakes, cliffs, enemies).
2. Gather resources through mining, chopping, hunting.
3. Craft tools, clothes, weapons, spell components.
4. Build structures and modify terrain.
5. Fight enemies, bosses, and engage in magic combat.
6. Level stats, unlock perks, craft stronger gear.
7. Repeat with higher difficulty and more biomes.

## Game Pillars

* **Infinite tile world** with verticality.
* **Cooperative multiplayer** and shared world progression.
* **Player-driven crafting & customization** (weapons, armor, spells).
* **Warm + cozy aesthetic** with isometric rendering.
* **Persistent world** with safe saves (double-buffered).
* **Real-time combat** with server-authoritative hits.
