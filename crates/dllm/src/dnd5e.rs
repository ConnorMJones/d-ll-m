use serde::{Deserialize, Serialize};
use spacetimedb::SpacetimeType;

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpellSchool {
    #[serde(rename = "A")]
    Abjuration,
    #[serde(rename = "C")]
    Conjuration,
    #[serde(rename = "D")]
    Divination,
    #[serde(rename = "E")]
    Enchantment,
    #[serde(rename = "V")]
    Evocation,
    #[serde(rename = "I")]
    Illusion,
    #[serde(rename = "N")]
    Necromancy,
    #[serde(rename = "T")]
    Transmutation,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Ability {
    #[serde(alias = "str")]
    Strength,
    #[serde(alias = "dex")]
    Dexterity,
    #[serde(alias = "con")]
    Constitution,
    #[serde(alias = "int")]
    Intelligence,
    #[serde(alias = "wis")]
    Wisdom,
    #[serde(alias = "cha")]
    Charisma,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CastingTimeUnit {
    Action,
    #[serde(alias = "bonus")]
    BonusAction,
    Reaction,
    Minute,
    Hour,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreatureSize {
    #[serde(rename = "T")]
    Tiny,
    #[serde(rename = "S")]
    Small,
    #[serde(rename = "M")]
    Medium,
    #[serde(rename = "L")]
    Large,
    #[serde(rename = "H")]
    Huge,
    #[serde(rename = "G")]
    Gargantuan,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CreatureType {
    Aberration,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    Humanoid,
    Monstrosity,
    Ooze,
    Plant,
    Undead,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ItemRarity {
    #[serde(rename = "none")]
    NoRarity,
    Common,
    Uncommon,
    Rare,
    #[serde(rename = "very rare")]
    VeryRare,
    Legendary,
    Artifact,
    Unknown,
    #[serde(rename = "unknown (magic)")]
    UnknownMagic,
    Varies,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    #[serde(rename = "LA")]
    LightArmor,
    #[serde(rename = "MA")]
    MediumArmor,
    #[serde(rename = "HA")]
    HeavyArmor,
    #[serde(rename = "S")]
    Shield,
    #[serde(rename = "M")]
    MeleeWeapon,
    #[serde(rename = "R")]
    RangedWeapon,
    #[serde(rename = "A", alias = "AF")]
    Ammunition,
    #[serde(rename = "P")]
    Potion,
    #[serde(rename = "RG")]
    Ring,
    #[serde(rename = "RD")]
    Rod,
    #[serde(rename = "SC")]
    Scroll,
    #[serde(rename = "ST")]
    Staff,
    #[serde(rename = "WD")]
    Wand,
    #[serde(rename = "W")]
    WondrousItem,
    #[serde(rename = "G")]
    Adventuring,
    #[serde(rename = "AT", alias = "T")]
    Tool,
    #[serde(rename = "INS")]
    Instrument,
    #[serde(rename = "GS")]
    GamingSet,
    #[serde(rename = "MNT")]
    Mount,
    #[serde(rename = "SHP")]
    Ship,
    #[serde(rename = "VEH")]
    Vehicle,
    #[serde(rename = "AIR")]
    Airship,
    #[serde(rename = "TG")]
    TradeGood,
    #[serde(
        rename = "$",
        alias = "$A",
        alias = "$C",
        alias = "$G",
        alias = "$H",
        alias = "$I",
        alias = "$P",
        alias = "$W"
    )]
    Treasure,
    #[serde(rename = "SCF")]
    SpellcastingFocus,
    #[serde(rename = "FD")]
    Food,
    #[serde(rename = "TAH")]
    Tack,
    #[serde(rename = "EXP")]
    Explosive,
    #[serde(rename = "SPC")]
    SpellComponent,
    #[serde(rename = "GV")]
    GenericVariant,
    #[serde(rename = "OTH", alias = "MR", alias = "TB")]
    Other,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeatCategory {
    #[serde(rename = "G")]
    General,
    #[serde(rename = "O")]
    Origin,
    #[serde(rename = "EB")]
    EpicBoon,
    #[serde(rename = "FS", alias = "FS:R", alias = "FS:P")]
    FightingStyle,
    #[serde(rename = "D")]
    Dragonmark,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Skill {
    Acrobatics,
    #[serde(rename = "animal handling")]
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    #[serde(rename = "sleight of hand")]
    SleightOfHand,
    Stealth,
    Survival,
}

impl Skill {
    pub fn ability(&self) -> Ability {
        match self {
            Self::Athletics => Ability::Strength,
            Self::Acrobatics | Self::SleightOfHand | Self::Stealth => Ability::Dexterity,
            Self::Arcana | Self::History | Self::Investigation | Self::Nature | Self::Religion => {
                Ability::Intelligence
            }
            Self::AnimalHandling
            | Self::Insight
            | Self::Medicine
            | Self::Perception
            | Self::Survival => Ability::Wisdom,
            Self::Deception | Self::Intimidation | Self::Performance | Self::Persuasion => {
                Ability::Charisma
            }
        }
    }
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SkillChoice {
    pub count: u8,
    pub from: Vec<Skill>,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SkillGrant {
    Fixed(Vec<Skill>),
    Choose(SkillChoice),
    Any(u8),
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct StringChoice {
    pub count: u8,
    pub from: Vec<String>,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ToolGrant {
    Fixed(Vec<String>),
    Choose(StringChoice),
    Any(u8),
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Common,
    #[serde(alias = "dwarven")]
    Dwarvish,
    #[serde(alias = "elven")]
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    #[serde(alias = "orcish")]
    Orc,
    Abyssal,
    Celestial,
    #[serde(rename = "deep speech")]
    DeepSpeech,
    Draconic,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
    Aquan,
    Auran,
    Ignan,
    Terran,
    Druidic,
    #[serde(rename = "thieves' cant")]
    ThievesCant,
    Gith,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LanguageChoice {
    pub count: u8,
    pub from: Vec<Language>,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LanguageGrant {
    Fixed(Vec<Language>),
    Choose(LanguageChoice),
    AnyStandard(u8),
    AnyExotic(u8),
    Any(u8),
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Speed {
    pub walk: u16,
    pub fly: u16,
    pub swim: u16,
    pub climb: u16,
    pub burrow: u16,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AbilityBonus {
    pub ability: Ability,
    pub bonus: i8,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AbilityChoice {
    pub count: u8,
    pub amount: i8,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum AbilityGrant {
    Fixed(Vec<AbilityBonus>),
    ChooseAny(AbilityChoice),
}
