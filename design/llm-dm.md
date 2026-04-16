# LLM DM

## Overview

The LLM acts as the Dungeon Master. It narrates, controls NPCs, adjudicates rules, and drives the story. Unlike a chat interface, it has structured tools to query and modify game state.

## Tool-Based Architecture

LLM has access to tools (function calling) for:

### State Queries
- `get_player(name)` - Get player character info
- `get_npc(name)` - Get NPC info
- `get_location()` - Current location and description
- `get_combat_state()` - Initiative order, HP, conditions
- `get_inventory(character)` - Character's items
- `search_rules(query)` - Look up rules/conditions
- `search_monsters(query)` - Find monster stats
- `search_spells(query)` - Find spell info

### State Mutations
- `deal_damage(target, amount, type)` - Apply damage
- `heal(target, amount)` - Heal character
- `add_condition(target, condition, duration)` - Apply condition
- `remove_condition(target, condition)` - Remove condition
- `give_item(target, item)` - Add item to inventory
- `take_item(target, item)` - Remove item from inventory
- `set_location(location)` - Move party to location
- `add_npc(npc)` - Introduce NPC to scene
- `update_npc(npc, changes)` - Modify NPC state

### Combat
- `start_combat(combatants)` - Begin combat encounter
- `roll_initiative()` - Roll and set initiative order
- `next_turn()` - Advance to next combatant
- `end_combat()` - End combat encounter

### Dice
- `roll(dice, modifier, reason)` - Roll dice (e.g., "2d6+3")
- `roll_secret(dice, modifier, reason)` - Roll without showing players

### Narrative
- `narrate(text)` - Send narration to all players
- `whisper(player, text)` - Private message to one player
- `prompt_player(player, prompt)` - Ask player for action

## Context Management

Game state can exceed context limits. Strategy:

1. **Always in context**: Current scene, active combatants, recent narrative (last N turns)
2. **Summarized**: Earlier session events, NPC relationships
3. **Retrieved on demand**: Full character sheets, detailed monster stats, spell descriptions

Server manages what goes into LLM context based on current situation.

## Campaign Awareness

When running a premade campaign, LLM has access to:
- Campaign synopsis and chapter summaries
- Current chapter/scene objectives
- Key NPCs and their motivations
- Locations and their secrets
- Planned encounters (can adapt based on player actions)

LLM balances following campaign structure with reacting to player agency.

## Consistency

To maintain world consistency:
- NPC registry with personalities, relationships, last seen location
- Event log for major story beats
- Faction/affinity tracking (extensible module)
- LLM prompted to check these before introducing contradictions

## Example Flow

```
Player: "I want to attack the goblin with my longsword"

LLM thinks:
1. Call get_combat_state() - confirm it's player's turn, goblin exists
2. Call get_player("Thorin") - get attack bonus
3. Call roll("1d20+5", "Thorin attacks goblin")
4. Result: 18, hits (goblin AC 15)
5. Call roll("1d8+3", "longsword damage")
6. Result: 7 slashing
7. Call deal_damage("Goblin 1", 7, "slashing")
8. Goblin HP: 7 -> 0, dies

LLM responds:
"Thorin's blade arcs through the air, catching the goblin across
the chest. The creature crumples to the ground, defeated.
[Goblin 1 defeated]"
```

## Configuration

DM behavior can be configured:
- Tone (serious, lighthearted, gritty)
- Rules strictness (RAW, rule of cool, flexible)
- Lethality (forgiving, balanced, deadly)
- Pacing preferences
- House rules
