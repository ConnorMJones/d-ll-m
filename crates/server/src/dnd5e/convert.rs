//! Conversions from dllm types to generated client types.
//! These exist because SpacetimeDB generates separate client types.

use dllm_bindings::{
    ability_bonus_type::AbilityBonus, ability_choice_type::AbilityChoice,
    ability_grant_type::AbilityGrant, ability_score_type::AbilityScore, ability_type::Ability,
    alignment_type::Alignment, caster_progression_type::CasterProgression,
    class_level_prereq_type::ClassLevelPrereq, class_type::Class, creature_size_type::CreatureSize,
    creature_type_type::CreatureType, feat_category_type::FeatCategory,
    feat_prereq_type::FeatPrereq, item_rarity_type::ItemRarity, item_type_type::ItemType,
    language_choice_type::LanguageChoice, language_grant_type::LanguageGrant,
    language_type::Language, optional_feature_prereq_type::OptionalFeaturePrereq,
    optional_feature_type_type::OptionalFeatureType, pact_boon_type::PactBoon, race_type::Race,
    skill_choice_type::SkillChoice, skill_grant_type::SkillGrant, skill_type::Skill,
    speed_type::Speed, spell_school_type::SpellSchool, string_choice_type::StringChoice,
    tool_grant_type::ToolGrant, warlock_patron_type::WarlockPatron,
};
use dllm_core::dnd5e as dnd;

pub fn spell_school(s: dnd::SpellSchool) -> SpellSchool {
    match s {
        dnd::SpellSchool::Abjuration => SpellSchool::Abjuration,
        dnd::SpellSchool::Conjuration => SpellSchool::Conjuration,
        dnd::SpellSchool::Divination => SpellSchool::Divination,
        dnd::SpellSchool::Enchantment => SpellSchool::Enchantment,
        dnd::SpellSchool::Evocation => SpellSchool::Evocation,
        dnd::SpellSchool::Illusion => SpellSchool::Illusion,
        dnd::SpellSchool::Necromancy => SpellSchool::Necromancy,
        dnd::SpellSchool::Transmutation => SpellSchool::Transmutation,
    }
}

pub fn alignment(a: dnd::Alignment) -> Alignment {
    match a {
        dnd::Alignment::Lawful => Alignment::Lawful,
        dnd::Alignment::Chaotic => Alignment::Chaotic,
        dnd::Alignment::Neutral => Alignment::Neutral,
        dnd::Alignment::Good => Alignment::Good,
        dnd::Alignment::Evil => Alignment::Evil,
    }
}

pub fn alignment_from_client(a: Alignment) -> dnd::Alignment {
    match a {
        Alignment::Lawful => dnd::Alignment::Lawful,
        Alignment::Chaotic => dnd::Alignment::Chaotic,
        Alignment::Neutral => dnd::Alignment::Neutral,
        Alignment::Good => dnd::Alignment::Good,
        Alignment::Evil => dnd::Alignment::Evil,
    }
}

pub fn ability(a: dnd::Ability) -> Ability {
    match a {
        dnd::Ability::Strength => Ability::Strength,
        dnd::Ability::Dexterity => Ability::Dexterity,
        dnd::Ability::Constitution => Ability::Constitution,
        dnd::Ability::Intelligence => Ability::Intelligence,
        dnd::Ability::Wisdom => Ability::Wisdom,
        dnd::Ability::Charisma => Ability::Charisma,
    }
}

pub fn creature_size(s: dnd::CreatureSize) -> CreatureSize {
    match s {
        dnd::CreatureSize::Tiny => CreatureSize::Tiny,
        dnd::CreatureSize::Small => CreatureSize::Small,
        dnd::CreatureSize::Medium => CreatureSize::Medium,
        dnd::CreatureSize::Large => CreatureSize::Large,
        dnd::CreatureSize::Huge => CreatureSize::Huge,
        dnd::CreatureSize::Gargantuan => CreatureSize::Gargantuan,
        dnd::CreatureSize::Varies => CreatureSize::Varies,
    }
}

pub fn creature_type(t: dnd::CreatureType) -> CreatureType {
    match t {
        dnd::CreatureType::Aberration => CreatureType::Aberration,
        dnd::CreatureType::Beast => CreatureType::Beast,
        dnd::CreatureType::Celestial => CreatureType::Celestial,
        dnd::CreatureType::Construct => CreatureType::Construct,
        dnd::CreatureType::Dragon => CreatureType::Dragon,
        dnd::CreatureType::Elemental => CreatureType::Elemental,
        dnd::CreatureType::Fey => CreatureType::Fey,
        dnd::CreatureType::Fiend => CreatureType::Fiend,
        dnd::CreatureType::Giant => CreatureType::Giant,
        dnd::CreatureType::Humanoid => CreatureType::Humanoid,
        dnd::CreatureType::Monstrosity => CreatureType::Monstrosity,
        dnd::CreatureType::Ooze => CreatureType::Ooze,
        dnd::CreatureType::Plant => CreatureType::Plant,
        dnd::CreatureType::Undead => CreatureType::Undead,
    }
}

pub fn item_rarity(r: dnd::ItemRarity) -> ItemRarity {
    match r {
        dnd::ItemRarity::NoRarity => ItemRarity::NoRarity,
        dnd::ItemRarity::Common => ItemRarity::Common,
        dnd::ItemRarity::Uncommon => ItemRarity::Uncommon,
        dnd::ItemRarity::Rare => ItemRarity::Rare,
        dnd::ItemRarity::VeryRare => ItemRarity::VeryRare,
        dnd::ItemRarity::Legendary => ItemRarity::Legendary,
        dnd::ItemRarity::Artifact => ItemRarity::Artifact,
        dnd::ItemRarity::Unknown => ItemRarity::Unknown,
        dnd::ItemRarity::UnknownMagic => ItemRarity::UnknownMagic,
        dnd::ItemRarity::Varies => ItemRarity::Varies,
    }
}

pub fn item_type(t: dnd::ItemType) -> ItemType {
    match t {
        dnd::ItemType::LightArmor => ItemType::LightArmor,
        dnd::ItemType::MediumArmor => ItemType::MediumArmor,
        dnd::ItemType::HeavyArmor => ItemType::HeavyArmor,
        dnd::ItemType::Shield => ItemType::Shield,
        dnd::ItemType::MeleeWeapon => ItemType::MeleeWeapon,
        dnd::ItemType::RangedWeapon => ItemType::RangedWeapon,
        dnd::ItemType::Ammunition => ItemType::Ammunition,
        dnd::ItemType::Potion => ItemType::Potion,
        dnd::ItemType::Ring => ItemType::Ring,
        dnd::ItemType::Rod => ItemType::Rod,
        dnd::ItemType::Scroll => ItemType::Scroll,
        dnd::ItemType::Staff => ItemType::Staff,
        dnd::ItemType::Wand => ItemType::Wand,
        dnd::ItemType::WondrousItem => ItemType::WondrousItem,
        dnd::ItemType::Adventuring => ItemType::Adventuring,
        dnd::ItemType::Tool => ItemType::Tool,
        dnd::ItemType::Instrument => ItemType::Instrument,
        dnd::ItemType::GamingSet => ItemType::GamingSet,
        dnd::ItemType::Mount => ItemType::Mount,
        dnd::ItemType::Ship => ItemType::Ship,
        dnd::ItemType::Vehicle => ItemType::Vehicle,
        dnd::ItemType::Airship => ItemType::Airship,
        dnd::ItemType::TradeGood => ItemType::TradeGood,
        dnd::ItemType::Treasure => ItemType::Treasure,
        dnd::ItemType::SpellcastingFocus => ItemType::SpellcastingFocus,
        dnd::ItemType::Food => ItemType::Food,
        dnd::ItemType::Tack => ItemType::Tack,
        dnd::ItemType::Explosive => ItemType::Explosive,
        dnd::ItemType::SpellComponent => ItemType::SpellComponent,
        dnd::ItemType::GenericVariant => ItemType::GenericVariant,
        dnd::ItemType::Other => ItemType::Other,
    }
}

pub fn feat_category(c: dnd::FeatCategory) -> FeatCategory {
    match c {
        dnd::FeatCategory::General => FeatCategory::General,
        dnd::FeatCategory::Origin => FeatCategory::Origin,
        dnd::FeatCategory::EpicBoon => FeatCategory::EpicBoon,
        dnd::FeatCategory::FightingStyle => FeatCategory::FightingStyle,
        dnd::FeatCategory::Dragonmark => FeatCategory::Dragonmark,
    }
}

pub fn skill(s: dnd::Skill) -> Skill {
    match s {
        dnd::Skill::Acrobatics => Skill::Acrobatics,
        dnd::Skill::AnimalHandling => Skill::AnimalHandling,
        dnd::Skill::Arcana => Skill::Arcana,
        dnd::Skill::Athletics => Skill::Athletics,
        dnd::Skill::Deception => Skill::Deception,
        dnd::Skill::History => Skill::History,
        dnd::Skill::Insight => Skill::Insight,
        dnd::Skill::Intimidation => Skill::Intimidation,
        dnd::Skill::Investigation => Skill::Investigation,
        dnd::Skill::Medicine => Skill::Medicine,
        dnd::Skill::Nature => Skill::Nature,
        dnd::Skill::Perception => Skill::Perception,
        dnd::Skill::Performance => Skill::Performance,
        dnd::Skill::Persuasion => Skill::Persuasion,
        dnd::Skill::Religion => Skill::Religion,
        dnd::Skill::SleightOfHand => Skill::SleightOfHand,
        dnd::Skill::Stealth => Skill::Stealth,
        dnd::Skill::Survival => Skill::Survival,
    }
}

pub fn skill_grant(g: dnd::SkillGrant) -> SkillGrant {
    match g {
        dnd::SkillGrant::Fixed(skills) => {
            SkillGrant::Fixed(skills.into_iter().map(skill).collect())
        }
        dnd::SkillGrant::Choose(c) => SkillGrant::Choose(SkillChoice {
            count: c.count,
            from: c.from.into_iter().map(skill).collect(),
        }),
        dnd::SkillGrant::Any(n) => SkillGrant::Any(n),
    }
}

pub fn tool_grant(g: dnd::ToolGrant) -> ToolGrant {
    match g {
        dnd::ToolGrant::Fixed(tools) => ToolGrant::Fixed(tools),
        dnd::ToolGrant::Choose(c) => ToolGrant::Choose(StringChoice {
            count: c.count,
            from: c.from,
        }),
        dnd::ToolGrant::Any(n) => ToolGrant::Any(n),
    }
}

pub fn language(l: dnd::Language) -> Language {
    match l {
        dnd::Language::Common => Language::Common,
        dnd::Language::Dwarvish => Language::Dwarvish,
        dnd::Language::Elvish => Language::Elvish,
        dnd::Language::Giant => Language::Giant,
        dnd::Language::Gnomish => Language::Gnomish,
        dnd::Language::Goblin => Language::Goblin,
        dnd::Language::Halfling => Language::Halfling,
        dnd::Language::Orc => Language::Orc,
        dnd::Language::Abyssal => Language::Abyssal,
        dnd::Language::Celestial => Language::Celestial,
        dnd::Language::DeepSpeech => Language::DeepSpeech,
        dnd::Language::Draconic => Language::Draconic,
        dnd::Language::Infernal => Language::Infernal,
        dnd::Language::Primordial => Language::Primordial,
        dnd::Language::Sylvan => Language::Sylvan,
        dnd::Language::Undercommon => Language::Undercommon,
        dnd::Language::Aquan => Language::Aquan,
        dnd::Language::Auran => Language::Auran,
        dnd::Language::Ignan => Language::Ignan,
        dnd::Language::Terran => Language::Terran,
        dnd::Language::Druidic => Language::Druidic,
        dnd::Language::ThievesCant => Language::ThievesCant,
        dnd::Language::Gith => Language::Gith,
        dnd::Language::Other => Language::Other,
    }
}

pub fn language_grant(g: dnd::LanguageGrant) -> LanguageGrant {
    match g {
        dnd::LanguageGrant::Fixed(langs) => {
            LanguageGrant::Fixed(langs.into_iter().map(language).collect())
        }
        dnd::LanguageGrant::Choose(c) => LanguageGrant::Choose(LanguageChoice {
            count: c.count,
            from: c.from.into_iter().map(language).collect(),
        }),
        dnd::LanguageGrant::AnyStandard(n) => LanguageGrant::AnyStandard(n),
        dnd::LanguageGrant::AnyExotic(n) => LanguageGrant::AnyExotic(n),
        dnd::LanguageGrant::Any(n) => LanguageGrant::Any(n),
    }
}

pub fn speed(s: dnd::Speed) -> Speed {
    Speed {
        walk: s.walk,
        fly: s.fly,
        swim: s.swim,
        climb: s.climb,
        burrow: s.burrow,
    }
}

pub fn ability_bonus(b: dnd::AbilityBonus) -> AbilityBonus {
    AbilityBonus {
        ability: ability(b.ability),
        bonus: b.bonus,
    }
}

pub fn ability_grant(g: dnd::AbilityGrant) -> AbilityGrant {
    match g {
        dnd::AbilityGrant::Fixed(bonuses) => {
            AbilityGrant::Fixed(bonuses.into_iter().map(ability_bonus).collect())
        }
        dnd::AbilityGrant::ChooseAny(c) => AbilityGrant::ChooseAny(AbilityChoice {
            count: c.count,
            amount: c.amount,
        }),
    }
}

pub fn class(c: dnd::Class) -> Class {
    match c {
        dnd::Class::Artificer => Class::Artificer,
        dnd::Class::Barbarian => Class::Barbarian,
        dnd::Class::Bard => Class::Bard,
        dnd::Class::Cleric => Class::Cleric,
        dnd::Class::Druid => Class::Druid,
        dnd::Class::Fighter => Class::Fighter,
        dnd::Class::Monk => Class::Monk,
        dnd::Class::Paladin => Class::Paladin,
        dnd::Class::Ranger => Class::Ranger,
        dnd::Class::Rogue => Class::Rogue,
        dnd::Class::Sorcerer => Class::Sorcerer,
        dnd::Class::Warlock => Class::Warlock,
        dnd::Class::Wizard => Class::Wizard,
    }
}

pub fn caster_progression(c: dnd::CasterProgression) -> CasterProgression {
    match c {
        dnd::CasterProgression::Full => CasterProgression::Full,
        dnd::CasterProgression::Half => CasterProgression::Half,
        dnd::CasterProgression::Third => CasterProgression::Third,
        dnd::CasterProgression::Artificer => CasterProgression::Artificer,
        dnd::CasterProgression::Pact => CasterProgression::Pact,
    }
}

pub fn pact_boon(p: dnd::PactBoon) -> PactBoon {
    match p {
        dnd::PactBoon::Chain => PactBoon::Chain,
        dnd::PactBoon::Blade => PactBoon::Blade,
        dnd::PactBoon::Tome => PactBoon::Tome,
        dnd::PactBoon::Talisman => PactBoon::Talisman,
    }
}

pub fn warlock_patron(p: dnd::WarlockPatron) -> WarlockPatron {
    match p {
        dnd::WarlockPatron::Archfey => WarlockPatron::Archfey,
        dnd::WarlockPatron::Fiend => WarlockPatron::Fiend,
        dnd::WarlockPatron::GreatOldOne => WarlockPatron::GreatOldOne,
        dnd::WarlockPatron::Celestial => WarlockPatron::Celestial,
        dnd::WarlockPatron::Hexblade => WarlockPatron::Hexblade,
        dnd::WarlockPatron::Fathomless => WarlockPatron::Fathomless,
        dnd::WarlockPatron::Genie => WarlockPatron::Genie,
        dnd::WarlockPatron::Undead => WarlockPatron::Undead,
        dnd::WarlockPatron::Undying => WarlockPatron::Undying,
    }
}

pub fn optional_feature_type(t: dnd::OptionalFeatureType) -> OptionalFeatureType {
    match t {
        dnd::OptionalFeatureType::ArtificerInfusion => OptionalFeatureType::ArtificerInfusion,
        dnd::OptionalFeatureType::ArcaneShot => OptionalFeatureType::ArcaneShot,
        dnd::OptionalFeatureType::ElementalDiscipline => OptionalFeatureType::ElementalDiscipline,
        dnd::OptionalFeatureType::EldritchInvocation => OptionalFeatureType::EldritchInvocation,
        dnd::OptionalFeatureType::FightingStyleBard => OptionalFeatureType::FightingStyleBard,
        dnd::OptionalFeatureType::FightingStyleFighter => OptionalFeatureType::FightingStyleFighter,
        dnd::OptionalFeatureType::FightingStylePaladin => OptionalFeatureType::FightingStylePaladin,
        dnd::OptionalFeatureType::FightingStyleRanger => OptionalFeatureType::FightingStyleRanger,
        dnd::OptionalFeatureType::Metamagic => OptionalFeatureType::Metamagic,
        dnd::OptionalFeatureType::ManeuverBattleMaster => OptionalFeatureType::ManeuverBattleMaster,
        dnd::OptionalFeatureType::PactBoon => OptionalFeatureType::PactBoon,
        dnd::OptionalFeatureType::Rune => OptionalFeatureType::Rune,
        dnd::OptionalFeatureType::RunePrestige => OptionalFeatureType::RunePrestige,
    }
}

pub fn optional_feature_prereq(p: dnd::OptionalFeaturePrereq) -> OptionalFeaturePrereq {
    OptionalFeaturePrereq {
        level: p.level.map(|l| ClassLevelPrereq {
            class: class(l.class),
            level: l.level,
        }),
        pact: p.pact.map(pact_boon),
        patron: p.patron.map(warlock_patron),
    }
}

pub fn race(r: dnd::Race) -> Race {
    match r {
        dnd::Race::Dragonborn => Race::Dragonborn,
        dnd::Race::Dwarf => Race::Dwarf,
        dnd::Race::Elf => Race::Elf,
        dnd::Race::Gnome => Race::Gnome,
        dnd::Race::HalfElf => Race::HalfElf,
        dnd::Race::Halfling => Race::Halfling,
        dnd::Race::HalfOrc => Race::HalfOrc,
        dnd::Race::Human => Race::Human,
        dnd::Race::Tiefling => Race::Tiefling,
        dnd::Race::Aasimar => Race::Aasimar,
        dnd::Race::Firbolg => Race::Firbolg,
        dnd::Race::Goliath => Race::Goliath,
        dnd::Race::Kenku => Race::Kenku,
        dnd::Race::Lizardfolk => Race::Lizardfolk,
        dnd::Race::Tabaxi => Race::Tabaxi,
        dnd::Race::Triton => Race::Triton,
        dnd::Race::Goblin => Race::Goblin,
        dnd::Race::Hobgoblin => Race::Hobgoblin,
        dnd::Race::Bugbear => Race::Bugbear,
        dnd::Race::Orc => Race::Orc,
        dnd::Race::Kobold => Race::Kobold,
        dnd::Race::Genasi => Race::Genasi,
        dnd::Race::Tortle => Race::Tortle,
        dnd::Race::Changeling => Race::Changeling,
        dnd::Race::Shifter => Race::Shifter,
        dnd::Race::Warforged => Race::Warforged,
        dnd::Race::Kalashtar => Race::Kalashtar,
        dnd::Race::Vampire => Race::Vampire,
    }
}

pub fn ability_score(s: dnd::AbilityScore) -> AbilityScore {
    AbilityScore {
        ability: ability(s.ability),
        minimum: s.minimum,
    }
}

pub fn feat_prereq(p: dnd::FeatPrereq) -> FeatPrereq {
    FeatPrereq {
        level: p.level,
        races: p.races.into_iter().map(race).collect(),
        sizes: p.sizes.into_iter().map(creature_size).collect(),
        abilities: p.abilities.into_iter().map(ability_score).collect(),
        spellcasting: p.spellcasting,
    }
}

pub fn spell_school_from_client(s: SpellSchool) -> dnd::SpellSchool {
    match s {
        SpellSchool::Abjuration => dnd::SpellSchool::Abjuration,
        SpellSchool::Conjuration => dnd::SpellSchool::Conjuration,
        SpellSchool::Divination => dnd::SpellSchool::Divination,
        SpellSchool::Enchantment => dnd::SpellSchool::Enchantment,
        SpellSchool::Evocation => dnd::SpellSchool::Evocation,
        SpellSchool::Illusion => dnd::SpellSchool::Illusion,
        SpellSchool::Necromancy => dnd::SpellSchool::Necromancy,
        SpellSchool::Transmutation => dnd::SpellSchool::Transmutation,
    }
}

pub fn creature_size_from_client(s: CreatureSize) -> dnd::CreatureSize {
    match s {
        CreatureSize::Tiny => dnd::CreatureSize::Tiny,
        CreatureSize::Small => dnd::CreatureSize::Small,
        CreatureSize::Medium => dnd::CreatureSize::Medium,
        CreatureSize::Large => dnd::CreatureSize::Large,
        CreatureSize::Huge => dnd::CreatureSize::Huge,
        CreatureSize::Gargantuan => dnd::CreatureSize::Gargantuan,
        CreatureSize::Varies => dnd::CreatureSize::Varies,
    }
}

pub fn creature_type_from_client(t: CreatureType) -> dnd::CreatureType {
    match t {
        CreatureType::Aberration => dnd::CreatureType::Aberration,
        CreatureType::Beast => dnd::CreatureType::Beast,
        CreatureType::Celestial => dnd::CreatureType::Celestial,
        CreatureType::Construct => dnd::CreatureType::Construct,
        CreatureType::Dragon => dnd::CreatureType::Dragon,
        CreatureType::Elemental => dnd::CreatureType::Elemental,
        CreatureType::Fey => dnd::CreatureType::Fey,
        CreatureType::Fiend => dnd::CreatureType::Fiend,
        CreatureType::Giant => dnd::CreatureType::Giant,
        CreatureType::Humanoid => dnd::CreatureType::Humanoid,
        CreatureType::Monstrosity => dnd::CreatureType::Monstrosity,
        CreatureType::Ooze => dnd::CreatureType::Ooze,
        CreatureType::Plant => dnd::CreatureType::Plant,
        CreatureType::Undead => dnd::CreatureType::Undead,
    }
}

pub fn item_rarity_from_client(r: ItemRarity) -> dnd::ItemRarity {
    match r {
        ItemRarity::NoRarity => dnd::ItemRarity::NoRarity,
        ItemRarity::Common => dnd::ItemRarity::Common,
        ItemRarity::Uncommon => dnd::ItemRarity::Uncommon,
        ItemRarity::Rare => dnd::ItemRarity::Rare,
        ItemRarity::VeryRare => dnd::ItemRarity::VeryRare,
        ItemRarity::Legendary => dnd::ItemRarity::Legendary,
        ItemRarity::Artifact => dnd::ItemRarity::Artifact,
        ItemRarity::Unknown => dnd::ItemRarity::Unknown,
        ItemRarity::UnknownMagic => dnd::ItemRarity::UnknownMagic,
        ItemRarity::Varies => dnd::ItemRarity::Varies,
    }
}

pub fn item_type_from_client(t: ItemType) -> dnd::ItemType {
    match t {
        ItemType::LightArmor => dnd::ItemType::LightArmor,
        ItemType::MediumArmor => dnd::ItemType::MediumArmor,
        ItemType::HeavyArmor => dnd::ItemType::HeavyArmor,
        ItemType::Shield => dnd::ItemType::Shield,
        ItemType::MeleeWeapon => dnd::ItemType::MeleeWeapon,
        ItemType::RangedWeapon => dnd::ItemType::RangedWeapon,
        ItemType::Ammunition => dnd::ItemType::Ammunition,
        ItemType::Potion => dnd::ItemType::Potion,
        ItemType::Ring => dnd::ItemType::Ring,
        ItemType::Rod => dnd::ItemType::Rod,
        ItemType::Scroll => dnd::ItemType::Scroll,
        ItemType::Staff => dnd::ItemType::Staff,
        ItemType::Wand => dnd::ItemType::Wand,
        ItemType::WondrousItem => dnd::ItemType::WondrousItem,
        ItemType::Adventuring => dnd::ItemType::Adventuring,
        ItemType::Tool => dnd::ItemType::Tool,
        ItemType::Instrument => dnd::ItemType::Instrument,
        ItemType::GamingSet => dnd::ItemType::GamingSet,
        ItemType::Mount => dnd::ItemType::Mount,
        ItemType::Ship => dnd::ItemType::Ship,
        ItemType::Vehicle => dnd::ItemType::Vehicle,
        ItemType::Airship => dnd::ItemType::Airship,
        ItemType::TradeGood => dnd::ItemType::TradeGood,
        ItemType::Treasure => dnd::ItemType::Treasure,
        ItemType::SpellcastingFocus => dnd::ItemType::SpellcastingFocus,
        ItemType::Food => dnd::ItemType::Food,
        ItemType::Tack => dnd::ItemType::Tack,
        ItemType::Explosive => dnd::ItemType::Explosive,
        ItemType::SpellComponent => dnd::ItemType::SpellComponent,
        ItemType::GenericVariant => dnd::ItemType::GenericVariant,
        ItemType::Other => dnd::ItemType::Other,
    }
}
