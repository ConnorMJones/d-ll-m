# Architecture

## Overview

d-ll-m is a multiplayer TTRPG platform with an LLM as the DM. Server holds authoritative game state, clients connect to play.

## Tech Stack

- **Server**: SpacetimeDB - handles state, persistence, real-time sync, networking
- **Core**: Rust library for game types, traits, system interfaces
- **Client**: Library for connecting to server, multiple frontends (CLI, web, native)
- **LLM**: Integrated into server, has tool access for DM actions

## Crates

```
dllm           - Core types, traits, game system interfaces
dllm-server    - SpacetimeDB module, LLM DM integration
dllm-client    - Client library for connecting/syncing
dllm-cli       - CLI client + tools (import, map creation, character builder, etc.)
```

## Related Docs

- [data.md](data.md) - Database schema, SpacetimeDB tables, content sources
- [game-systems.md](game-systems.md) - TTRPG plugin architecture
- [llm-dm.md](llm-dm.md) - LLM tooling and DM behavior
- [campaigns.md](campaigns.md) - Campaign structure and import
- [tools.md](tools.md) - CLI tools, map creation, TTRPG integration kit

## Data Flow

```
┌─────────────────────────────────────────────────────────┐
│                        Server                           │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐ │
│  │ SpacetimeDB │◄──►│  Game State │◄──►│   LLM DM    │ │
│  │  (persist)  │    │   (ECS?)    │    │  (tools)    │ │
│  └─────────────┘    └──────┬──────┘    └─────────────┘ │
│                            │                            │
└────────────────────────────┼────────────────────────────┘
                             │ sync
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
   ┌─────────┐          ┌─────────┐          ┌─────────┐
   │ Client  │          │ Client  │          │ Client  │
   │ (CLI)   │          │ (Web)   │          │ (App)   │
   └─────────┘          └─────────┘          └─────────┘
```

## State Ownership

Server is authoritative. Clients send actions, server validates and applies.

- **Server owns**: All game state, NPC decisions, dice rolls, rule adjudication
- **Clients own**: UI state, input, local preferences

## TTRPG Data

Game rules and content live outside the crates in `ttrpg/`:

```
ttrpg/
├── dnd5e/
│   ├── rules/       # Monsters, spells, items, classes, etc.
│   └── campaigns/   # Curse of Strahd, homebrew, etc.
└── eclipse-phase/
    ├── rules/
    └── campaigns/
```

Server loads relevant data at runtime based on which system/campaign is active.

## Extensibility

Pluggable systems for:
- Game systems (5e, Eclipse Phase, others)
- Server modules (affinity tracking, reputation, custom mechanics)
- Client frontends (CLI, web, native)
