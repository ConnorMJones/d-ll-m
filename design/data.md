# Data Architecture

## Overview

Game rules and content stored in SpacetimeDB. JSON files are the source format, imported into DB tables for runtime queries.

## Data Flow

```
JSON files (source)
       │
       ▼ import
┌─────────────────┐
│  SpacetimeDB    │
│  ┌───────────┐  │
│  │ monsters  │  │
│  │ spells    │  │
│  │ items     │  │
│  │ classes   │  │
│  │ ...       │  │
│  └───────────┘  │
└────────┬────────┘
         │ query
    ┌────┴────┐
    ▼         ▼
 Server    Clients
 (LLM DM)  (lookup)
```

## Tables

### Core Tables (per game system)

```
monsters_{system}
├── id: string
├── name: string
├── cr: float
├── type: string
├── size: string
├── hp: int
├── ac: int
├── stats: json      # STR, DEX, etc.
├── abilities: json  # Special abilities
├── actions: json    # Attacks, etc.
└── source: string   # SRD, PHB, homebrew

spells_{system}
├── id: string
├── name: string
├── level: int
├── school: string
├── casting_time: string
├── range: string
├── components: string
├── duration: string
├── description: text
├── higher_levels: text
└── source: string

items_{system}
├── id: string
├── name: string
├── type: string
├── rarity: string
├── properties: json
├── description: text
└── source: string

classes_{system}
├── id: string
├── name: string
├── hit_die: int
├── proficiencies: json
├── features: json     # By level
├── subclasses: json
└── source: string

races_{system}
├── id: string
├── name: string
├── traits: json
├── subraces: json
└── source: string
```

### Game State Tables

```
campaigns
├── id: string
├── name: string
├── system: string
├── created_at: timestamp
├── settings: json

sessions
├── id: string
├── campaign_id: string
├── started_at: timestamp
├── ended_at: timestamp
├── summary: text

characters
├── id: string
├── campaign_id: string
├── player_id: string
├── name: string
├── data: json          # Full character sheet
├── current_hp: int
├── conditions: json
├── inventory: json
├── position: json      # Map location

npcs
├── id: string
├── campaign_id: string
├── name: string
├── stat_block: string  # Reference to monster or custom
├── personality: text
├── current_state: json

maps
├── id: string
├── campaign_id: string
├── name: string
├── image_url: string
├── grid: json
├── metadata: json

tokens
├── id: string
├── map_id: string
├── character_id: string  # Or npc_id
├── x: int
├── y: int
├── visible: bool

events
├── id: string
├── session_id: string
├── timestamp: timestamp
├── type: string        # combat, dialogue, roll, etc.
├── data: json
├── narrative: text
```

## Subscriptions

SpacetimeDB allows clients to subscribe to table changes. Examples:

- Player subscribes to their character's state
- All players subscribe to visible tokens on current map
- GM subscribes to all NPCs in current scene
- Everyone subscribes to narrative events

## Content Sources

### Official Content
- SRD (free, can distribute)
- Licensed content (user imports their own)

### Import Sources
- 5e.tools JSON (comprehensive 5e data)
- PDF extraction (campaign books)
- Manual entry

### Homebrew
- User-created content follows same schema
- Tagged as homebrew for filtering
- Can share between campaigns

## Queries

LLM DM has query tools:

```rust
// Find monsters for an encounter
search_monsters(cr_min: 1, cr_max: 3, type: "undead")

// Spell lookup
get_spell("fireball")

// Rules search (full text)
search_rules("grappled condition")

// Campaign-specific NPC
get_npc(campaign_id, "strahd")
```

## Data Validation

Import validates against schemas:
- Required fields present
- Types correct
- References valid (spell references existing conditions, etc.)
- System-specific rules (5e spell levels 0-9, etc.)

```bash
dllm-cli validate --system dnd5e --file monsters.json

✓ 320 monsters validated
✗ Error: "custom_dragon" missing required field "cr"
⚠ Warning: "goblin_king" references unknown condition "frightened_variant"
```
