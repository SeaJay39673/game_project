# UI/UX Design Document

The UI style matches the cozy-isometric fantasy aesthetic.

---

# 1. UI Philosophy

* Minimal, clean, readable
* Pixel/illustrated aesthetic
* Everything readable at a glance
* Controller-friendly layout
* Drag-and-drop inventory (mouse)

---

# 2. UI Screens

## A) **HUD (In-Game)**

* Health bar (top-left)
* Stamina bar
* Mana bar
* Hotbar (bottom center)
* Compass (optional)
* Minimal quest/achievement hints

## HUD Tasks

* Bars with animation
* Hotbar inventory linking
* Spell cooldown indicators

---

## B) **Inventory**

Grid-based layout:

```
[ 10x5 grid ]
[ Bag extensions ]
[ Character paper doll (optional v1.0+) ]
```

### Features:

* Drag-and-drop
* Item tooltips
* Weight or slot limits
* Quick-equip with hotkeys

---

## C) **Crafting UI**

* Recipe list
* Recipe details panel
* Required ingredients
* “Craft X” button
* Crafting station modifiers (forge, loom, etc.)

---

## D) **Spellbook UI**

* Spell slot grid
* Sigils panel
* Drag sigils to create spells
* Visual connections between sigils

---

## E) **Trading UI**

* NPC inventory
* Player inventory
* Trade proposal list
* Confirmation button

---

## F) **Settings UI**

* Audio sliders
* Keybind configuration
* Controller bindings
* Video settings (resolution, vsync)
* Networking diagnostics

---

# 3. Input UX

## Mouse + Keyboard

* WASD movement
* Click-to-interact (optional)
* Drag to rearrange inventory
* Scroll to cycle hotbar

## Controller

* Left stick: move
* Right stick: aim/spell direction
* A: interact
* Triggers: spellcasting / item use
* Bumpers: change hotbar slot

---

# 4. Visual Language Guide

* Rounded UI panels
* Warm palette (beiges, browns, soft blues)
* Atlas sprites for UI elements
* 1–2 px outlines to match tile art style

---

# 5. Audio UX

* Soft UI clicks
* Inventory sounds (wooden “thunks”, metal “clinks”)
* Crafting sound feedback
* Spell sound cues

---

# 6. Accessibility

* Colorblind-friendly palette
* UI scaling slider
* Larger text mode
* Toggle for flashing animations (magic spells)
