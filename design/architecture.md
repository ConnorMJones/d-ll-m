# Architecture

## Overview

d-ll-m is a multiplayer TTRPG platform. Server holds authoritative game state, clients connect to play.

## Tech Stack

- **Server**: SpacetimeDB - handles state, persistence, real-time sync, networking
- **Core**: Rust library for game types and system interfaces
- **Client**: Library for connecting to server

## Crates

```
dllm           - Core types, game system interfaces
dllm-server    - SpacetimeDB module
dllm-client    - Client library for connecting/syncing
dllm-cli       - CLI client
data-import    - Import game data (5etools JSON → SpacetimeDB)
spacetime      - SpacetimeDB connection and query helpers
tools          - Query and data tools
```

## State Ownership

Server is authoritative. Clients send actions, server validates and applies.

- **Server owns**: All game state, dice rolls, rule adjudication
- **Clients own**: UI state, input, local preferences

## TTRPG Data

Game rules and content live outside the crates in `ttrpg/`:

```
ttrpg/
└── dnd5e/
    └── rules/    # Monsters, spells, items, classes, etc. (5etools format)
```

Server loads relevant data at runtime based on which system/campaign is active.
