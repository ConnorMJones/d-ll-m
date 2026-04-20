use serde::{Deserialize, Serialize};
use spacetimedb::SpacetimeType;
use std::str::FromStr;

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
pub enum Alignment {
    #[serde(rename = "L")]
    Lawful,
    #[serde(rename = "C")]
    Chaotic,
    #[serde(rename = "N")]
    Neutral,
    #[serde(rename = "G")]
    Good,
    #[serde(rename = "E")]
    Evil,
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
    #[serde(rename = "V")]
    Varies,
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

impl FromStr for ItemRarity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "none" => Ok(Self::NoRarity),
            "common" => Ok(Self::Common),
            "uncommon" => Ok(Self::Uncommon),
            "rare" => Ok(Self::Rare),
            "very rare" => Ok(Self::VeryRare),
            "legendary" => Ok(Self::Legendary),
            "artifact" => Ok(Self::Artifact),
            "unknown" => Ok(Self::Unknown),
            "unknown (magic)" => Ok(Self::UnknownMagic),
            "varies" => Ok(Self::Varies),
            _ => Err(()),
        }
    }
}

#[derive(SpacetimeType, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    LightArmor,
    MediumArmor,
    HeavyArmor,
    Shield,
    MeleeWeapon,
    RangedWeapon,
    Ammunition,
    Potion,
    Ring,
    Rod,
    Scroll,
    Staff,
    Wand,
    WondrousItem,
    Adventuring,
    Tool,
    Instrument,
    GamingSet,
    Mount,
    Ship,
    Vehicle,
    Airship,
    TradeGood,
    Treasure,
    SpellcastingFocus,
    Food,
    Tack,
    Explosive,
    SpellComponent,
    GenericVariant,
    Other,
}

impl ItemType {
    pub fn from_code(code: &str) -> Option<Self> {
        let base_code = code.split('|').next().unwrap_or(code).trim();
        match base_code {
            "LA" => Some(Self::LightArmor),
            "MA" => Some(Self::MediumArmor),
            "HA" => Some(Self::HeavyArmor),
            "S" => Some(Self::Shield),
            "M" => Some(Self::MeleeWeapon),
            "R" => Some(Self::RangedWeapon),
            "A" | "AF" => Some(Self::Ammunition),
            "P" => Some(Self::Potion),
            "RG" => Some(Self::Ring),
            "RD" => Some(Self::Rod),
            "SC" => Some(Self::Scroll),
            "ST" => Some(Self::Staff),
            "WD" => Some(Self::Wand),
            "W" => Some(Self::WondrousItem),
            "G" => Some(Self::Adventuring),
            "AT" | "T" => Some(Self::Tool),
            "INS" => Some(Self::Instrument),
            "GS" => Some(Self::GamingSet),
            "MNT" => Some(Self::Mount),
            "SHP" => Some(Self::Ship),
            "VEH" => Some(Self::Vehicle),
            "AIR" => Some(Self::Airship),
            "TG" => Some(Self::TradeGood),
            "$" | "$A" | "$C" | "$G" | "$H" | "$I" | "$P" | "$W" => Some(Self::Treasure),
            "SCF" => Some(Self::SpellcastingFocus),
            "FD" => Some(Self::Food),
            "TAH" => Some(Self::Tack),
            "EXP" => Some(Self::Explosive),
            "SPC" => Some(Self::SpellComponent),
            "GV" => Some(Self::GenericVariant),
            "OTH" | "MR" | "TB" => Some(Self::Other),
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for ItemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_code(&s).ok_or_else(|| serde::de::Error::unknown_variant(&s, &[]))
    }
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
    Other,
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

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasterProgression {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "1/2")]
    Half,
    #[serde(rename = "1/3")]
    Third,
    #[serde(rename = "artificer")]
    Artificer,
    #[serde(rename = "pact")]
    Pact,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PactBoon {
    #[serde(alias = "Pact of the Chain")]
    Chain,
    #[serde(alias = "Pact of the Blade")]
    Blade,
    #[serde(alias = "Pact of the Tome")]
    Tome,
    #[serde(alias = "Pact of the Talisman")]
    Talisman,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarlockPatron {
    Archfey,
    Fiend,
    #[serde(rename = "Great Old One")]
    GreatOldOne,
    Celestial,
    Hexblade,
    Fathomless,
    Genie,
    Undead,
    Undying,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptionalFeatureType {
    #[serde(rename = "AI")]
    ArtificerInfusion,
    #[serde(rename = "AS")]
    ArcaneShot,
    #[serde(rename = "ED")]
    ElementalDiscipline,
    #[serde(rename = "EI")]
    EldritchInvocation,
    #[serde(rename = "FS:B")]
    FightingStyleBard,
    #[serde(rename = "FS:F")]
    FightingStyleFighter,
    #[serde(rename = "FS:P")]
    FightingStylePaladin,
    #[serde(rename = "FS:R")]
    FightingStyleRanger,
    #[serde(rename = "MM")]
    Metamagic,
    #[serde(rename = "MV:B")]
    ManeuverBattleMaster,
    #[serde(rename = "PB")]
    PactBoon,
    #[serde(rename = "RN")]
    Rune,
    #[serde(rename = "RP")]
    RunePrestige,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ClassLevelPrereq {
    pub class: Class,
    pub level: u8,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct OptionalFeaturePrereq {
    pub level: Option<ClassLevelPrereq>,
    pub pact: Option<PactBoon>,
    pub patron: Option<WarlockPatron>,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Race {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    #[serde(rename = "half-elf")]
    HalfElf,
    Halfling,
    #[serde(rename = "half-orc")]
    HalfOrc,
    Human,
    Tiefling,
    Aasimar,
    Firbolg,
    Goliath,
    Kenku,
    Lizardfolk,
    Tabaxi,
    Triton,
    Goblin,
    Hobgoblin,
    Bugbear,
    Orc,
    Kobold,
    Genasi,
    Tortle,
    Changeling,
    Shifter,
    Warforged,
    Kalashtar,
    #[serde(rename = "Vampire (Ixalan)")]
    Vampire,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct FeatPrereq {
    pub level: Option<u8>,
    pub races: Vec<Race>,
    pub sizes: Vec<CreatureSize>,
    pub abilities: Vec<AbilityScore>,
    pub spellcasting: bool,
}

#[derive(SpacetimeType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AbilityScore {
    pub ability: Ability,
    pub minimum: u8,
}
