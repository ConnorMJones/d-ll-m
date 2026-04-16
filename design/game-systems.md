# Game Systems

## Overview

Each TTRPG (D&D 5e, Eclipse Phase, etc.) is a pluggable game system that implements core traits.

## Core Traits

```rust
trait GameSystem {
    fn id(&self) -> &str;  // "dnd5e", "eclipse-phase"
    fn name(&self) -> &str;

    // Character
    fn create_character(&self, options: CharacterOptions) -> Character;
    fn validate_character(&self, character: &Character) -> ValidationResult;
    fn level_up(&self, character: &mut Character, choices: LevelUpChoices);

    // Combat
    fn roll_initiative(&self, combatants: &[Combatant]) -> InitiativeOrder;
    fn resolve_attack(&self, attacker: &Character, target: &Character, attack: Attack) -> AttackResult;
    fn apply_damage(&self, target: &mut Character, damage: Damage);

    // Checks
    fn skill_check(&self, character: &Character, skill: Skill, dc: u32) -> CheckResult;
    fn saving_throw(&self, character: &Character, save: Save, dc: u32) -> CheckResult;

    // Rules queries
    fn lookup_rule(&self, query: &str) -> Option<RuleEntry>;
    fn lookup_monster(&self, name: &str) -> Option<Monster>;
    fn lookup_spell(&self, name: &str) -> Option<Spell>;
    fn lookup_item(&self, name: &str) -> Option<Item>;
}
```

## Data Format

Each system has a `rules/` folder with JSON data:

```
ttrpg/dnd5e/rules/
├── monsters/
│   ├── aberrations.json
│   ├── beasts.json
│   └── ...
├── spells/
│   ├── cantrips.json
│   ├── 1st-level.json
│   └── ...
├── items/
│   ├── weapons.json
│   ├── armor.json
│   ├── magic-items.json
│   └── ...
├── classes/
│   ├── fighter.json
│   ├── wizard.json
│   └── ...
├── races.json
├── backgrounds.json
├── feats.json
└── conditions.json
```

Can pull from 5e.tools JSON structure directly for D&D 5e.

## System Registration

Systems register with the server at startup:

```rust
fn register_systems(registry: &mut SystemRegistry) {
    registry.register(Dnd5e::new());
    registry.register(EclipsePhase::new());
}
```

Campaign specifies which system it uses. Server loads appropriate system and data.

## Supported Systems

### D&D 5e
- Full SRD support
- User can add licensed content (PHB, expansions)
- 5e.tools JSON format

### Eclipse Phase 2e
- Needs custom data format (no 5e.tools equivalent)
- Percentile-based system (d100)
- Ego/morph split for characters
- Mesh actions, async combat

## Adding New Systems

1. Create `ttrpg/<system>/rules/` with data files
2. Implement `GameSystem` trait
3. Register in server startup
4. Add campaign support
