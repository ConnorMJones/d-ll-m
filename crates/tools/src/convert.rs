//! Conversions from dllm types to generated client types.
//! These exist because SpacetimeDB generates separate client types.

use dllm::dnd5e as dnd;
use dllm_client::{
    ability_bonus_type::AbilityBonus, ability_choice_type::AbilityChoice,
    ability_grant_type::AbilityGrant, ability_type::Ability,
    creature_size_type::CreatureSize, creature_type_type::CreatureType,
    feat_category_type::FeatCategory, item_rarity_type::ItemRarity, item_type_type::ItemType,
    language_choice_type::LanguageChoice, language_grant_type::LanguageGrant,
    language_type::Language, skill_choice_type::SkillChoice, skill_grant_type::SkillGrant,
    skill_type::Skill, speed_type::Speed, spell_school_type::SpellSchool,
    string_choice_type::StringChoice, tool_grant_type::ToolGrant,
};

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
        dnd::AbilityGrant::ChooseAny(c) => {
            AbilityGrant::ChooseAny(AbilityChoice {
                count: c.count,
                amount: c.amount,
            })
        }
    }
}
