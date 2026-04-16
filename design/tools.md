# Tools

## Overview

CLI and in-app tools for content creation, game management, and system integration.

## CLI Tools (dllm-cli)

### Data Import

```bash
# Import 5e.tools JSON into database
dllm-cli import rules --system dnd5e --source ./5etools-data/

# Import specific content
dllm-cli import monsters --system dnd5e --file monsters.json
dllm-cli import spells --system dnd5e --file spells.json

# Import campaign from PDF
dllm-cli import campaign --pdf curse-of-strahd.pdf --system dnd5e
```

### Character Creation

```bash
# Interactive character builder
dllm-cli character create --system dnd5e

# Quick create from template
dllm-cli character create --system dnd5e --class fighter --race human --level 5

# Import character sheet
dllm-cli character import --file character.json
```

### Map Tools

```bash
# Create map from image
dllm-cli map create --image dungeon.png --grid 30x30 --scale 5ft

# Generate map (LLM-assisted)
dllm-cli map generate --prompt "tavern with 3 rooms, common area, kitchen, cellar"

# Add to campaign
dllm-cli map add --campaign curse-of-strahd --name "Death House Floor 1"
```

### Session Management

```bash
# Generate session summary
dllm-cli session summary --save my-game

# Generate recap for players
dllm-cli session recap --save my-game --format markdown

# Export session log
dllm-cli session export --save my-game --output session-log.md
```

### System Integration

```bash
# Scaffold new TTRPG system
dllm-cli system new --name "my-system"

# Validate system implementation
dllm-cli system validate --path ttrpg/my-system/

# Generate schema templates
dllm-cli system templates --output ttrpg/my-system/rules/
```

## Maps

### Map Format

```json
{
  "id": "death-house-f1",
  "name": "Death House - Floor 1",
  "image": "death-house-f1.png",
  "grid": {
    "width": 30,
    "height": 40,
    "cell_size": 5,
    "unit": "ft"
  },
  "walls": [...],           // Line segments for walls
  "doors": [...],           // Door positions and states
  "lighting": [...],        // Light sources
  "regions": [              // Named areas
    {"name": "Entrance Hall", "polygon": [...]}
  ]
}
```

### Map Features

- **Grid overlay**: Configurable size, hex or square
- **Tokens**: Character/NPC positions, conditions shown
- **Fog of war**: GM reveals areas as explored
- **Dynamic lighting**: Optional, based on light sources and walls
- **Measurements**: Distance tool for movement/range
- **Layers**: Background, grid, tokens, GM notes

### Map Creation

1. **From image**: Import existing map image, add grid and metadata
2. **From tiles**: Assemble from tileset (dungeon, forest, etc.)
3. **Generated**: LLM-assisted generation from description
4. **Drawn**: Simple drawing tools (planned, lower priority)

## Character Builder

### Features

- Guided creation per game system
- Validates choices against rules
- Calculates derived stats
- Equipment selection
- Spell/ability selection
- Backstory prompts (optional LLM assist)
- Export to multiple formats

### Flow (5e example)

1. Select race → apply racial traits
2. Select class → apply class features
3. Generate/assign ability scores
4. Select background
5. Choose equipment/starting gold
6. Select spells (if applicable)
7. Details (name, backstory, appearance)

## Session Summaries

### Auto-generated Content

After each session, LLM can generate:
- **Session summary**: What happened, key events
- **Character journal**: From each PC's perspective
- **World state changes**: NPCs met, locations discovered, items gained
- **Next session hooks**: Where we left off, open threads

### Playback

Save files include full session log. Tools for:
- Browse session history
- Search for specific events ("when did we meet Strahd?")
- Generate highlight reels
- Export campaign diary

## TTRPG Integration Kit

For adding new game systems.

### Scaffold Structure

```
ttrpg/new-system/
├── system.json           # System metadata
├── rules/
│   ├── schema/           # JSON schemas for validation
│   │   ├── character.schema.json
│   │   ├── monster.schema.json
│   │   └── ...
│   ├── characters/       # Character options
│   ├── items/
│   ├── abilities/
│   └── ...
└── campaigns/
```

### Required Implementations

Checklist for new systems:
- [ ] System metadata (name, dice, basic mechanics)
- [ ] Character schema and creation rules
- [ ] Combat resolution
- [ ] Skill/ability checks
- [ ] Damage and healing
- [ ] Conditions/status effects
- [ ] Advancement/leveling
- [ ] Equipment and inventory

### Validation Tool

```bash
dllm-cli system validate --path ttrpg/my-system/

✓ system.json valid
✓ Character schema valid
✗ Missing: combat resolution rules
✗ Missing: skill check definitions
⚠ Warning: No monsters defined
```

### Documentation Generator

```bash
dllm-cli system docs --path ttrpg/my-system/ --output docs/my-system/
```

Generates reference docs from system data for players/GMs.
