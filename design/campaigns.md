# Campaigns

## Overview

Campaigns are structured adventures the LLM DM can run. Can be premade (Curse of Strahd) or custom created.

## Campaign Structure

```
ttrpg/dnd5e/campaigns/curse-of-strahd/
├── campaign.json         # Metadata, synopsis, settings
├── chapters/
│   ├── 01-death-house.json
│   ├── 02-village-of-barovia.json
│   └── ...
├── npcs/
│   ├── strahd.json
│   ├── ireena.json
│   └── ...
├── locations/
│   ├── barovia-village.json
│   ├── castle-ravenloft.json
│   └── ...
├── encounters/
│   ├── death-house-encounters.json
│   └── ...
├── items/
│   └── campaign-items.json   # Unique items for this campaign
└── handouts/
    └── ...                   # Player handouts, letters, maps
```

## Campaign Metadata

`campaign.json`:
```json
{
  "id": "curse-of-strahd",
  "name": "Curse of Strahd",
  "system": "dnd5e",
  "level_range": [1, 10],
  "synopsis": "Gothic horror in the realm of Barovia...",
  "tone": "dark, gothic, horror",
  "themes": ["horror", "tragedy", "redemption"],
  "starting_chapter": "01-death-house",
  "dm_notes": "Key themes to emphasize..."
}
```

## Chapter Format

Each chapter contains:
- Objectives and completion conditions
- Key scenes and events
- Branch points (player choices that affect story)
- Encounters (combat, social, exploration)
- Transitions to other chapters

```json
{
  "id": "01-death-house",
  "name": "Death House",
  "synopsis": "Players explore a haunted house...",
  "objectives": [
    "Escape the Death House",
    "Discover the house's dark secret"
  ],
  "scenes": [...],
  "encounters": [...],
  "branches": {
    "saved_children": "next: 02-village-of-barovia (grateful_spirits)",
    "abandoned_children": "next: 02-village-of-barovia (haunted)"
  }
}
```

## NPC Format

```json
{
  "id": "ireena",
  "name": "Ireena Kolyana",
  "role": "key_npc",
  "description": "A striking young woman with auburn hair...",
  "personality": "Brave, determined, but haunted",
  "motivation": "Escape Strahd's obsession",
  "secrets": ["Is the reincarnation of Tatyana"],
  "relationships": {
    "strahd": "hunted_by",
    "ismark": "sibling"
  },
  "stat_block": "noble"  # Reference to rules/
}
```

## Campaign Import

### From PDF

CLI tool to extract campaign data from official PDFs:

```bash
dllm-cli campaign import --pdf curse-of-strahd.pdf --output ttrpg/dnd5e/campaigns/curse-of-strahd/
```

Pipeline:
1. PDF text extraction
2. LLM-assisted parsing (identify chapters, NPCs, locations)
3. Structure into JSON format
4. Human review/correction

### From Scratch

Create campaign manually or with LLM assistance:

```bash
dllm-cli campaign new --name "my-campaign" --system dnd5e
dllm-cli campaign add-chapter --campaign my-campaign --name "Chapter 1"
dllm-cli campaign add-npc --campaign my-campaign --name "Big Bad"
```

## Save States

Campaign progress saved separately from campaign definition:

```
saves/
└── my-game/
    ├── save.json           # Current state
    ├── characters/         # Player characters
    ├── session-log.json    # History of events
    └── world-state.json    # NPC states, locations visited, etc.
```

Save includes:
- Current chapter/scene
- Character states
- World modifications (NPCs killed, items taken, etc.)
- Narrative history (for context)
- Custom content added during play

## Homebrew Campaigns

Same format as official campaigns. Can mix:
- Custom story with official monsters/items
- Official story with homebrew additions
- Entirely custom everything

System validates that referenced rules exist (monsters, items, etc.) or flags missing content.
