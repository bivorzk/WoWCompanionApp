use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

type BlizzardExtraFields = BTreeMap<String, Value>;

#[derive(Clone, Debug, Deserialize)]
pub struct PvPTiers {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub tiers: Vec<PvPTier>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardLinks {
    #[serde(rename = "self")]
    pub self_link: BlizzardLink,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardLink {
    pub href: String,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PvPTier {
    pub key: BlizzardLink,
    pub name: String,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PvPTierMedia {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub assets: Vec<BlizzardMediaAsset>,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardMediaAsset {
    pub key: String,
    pub value: String,
    pub file_data_id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterAppearance {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub character: BlizzardAppearanceCharacter,
    pub playable_race: BlizzardAppearanceReference,
    pub playable_class: BlizzardAppearanceReference,
    pub active_spec: BlizzardAppearanceReference,
    pub gender: BlizzardTypeName,
    pub faction: BlizzardTypeName,
    pub guild_crest: Option<BlizzardAppearanceGuildCrest>,
    pub items: Vec<BlizzardAppearanceItem>,
    pub customizations: Vec<BlizzardAppearanceCustomization>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceCharacter {
    pub key: BlizzardLink,
    pub name: String,
    pub id: u32,
    pub realm: BlizzardAppearanceRealm,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceRealm {
    pub key: BlizzardLink,
    pub name: String,
    pub id: u32,
    pub slug: String,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceReference {
    pub key: BlizzardLink,
    pub name: String,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardTypeName {
    pub r#type: String,
    pub name: String,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceGuildCrest {
    pub emblem: BlizzardAppearanceGuildCrestComponent,
    pub border: BlizzardAppearanceGuildCrestComponent,
    pub background: BlizzardAppearanceGuildBackground,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceGuildCrestComponent {
    pub id: u32,
    pub media: BlizzardAppearanceMediaReference,
    pub color: BlizzardAppearanceColor,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceGuildBackground {
    pub color: BlizzardAppearanceColor,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceMediaReference {
    pub key: BlizzardLink,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceColor {
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceItem {
    pub id: u32,
    pub slot: BlizzardTypeName,
    pub enchant: u32,
    pub item_appearance_modifier_id: Option<u32>,
    pub internal_slot_id: u32,
    pub subclass: Option<u32>,
    pub secondary_id: Option<u32>,
    pub secondary_item_appearance_modifier_id: Option<u32>,
    pub secondary_subclass: Option<u32>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceCustomization {
    pub option: BlizzardAppearanceCustomizationOption,
    pub choice: BlizzardAppearanceCustomizationChoice,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceCustomizationOption {
    pub name: String,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAppearanceCustomizationChoice {
    pub name: Option<String>,
    pub id: u32,
    pub display_order: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterPvPSummary {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub honor_level: u32,
    pub pvp_map_statistics: Vec<BlizzardPvPMapStatistic>,
    pub honorable_kills: u32,
    pub character: BlizzardAppearanceCharacter,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardPvPMapStatistic {
    pub world_map: BlizzardNamedEntity,
    pub match_statistics: BlizzardPvPMatchStatistics,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardNamedEntity {
    pub name: String,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardPvPMatchStatistics {
    pub played: u32,
    pub won: u32,
    pub lost: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterPvPBracketStatistics {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub character: BlizzardAppearanceCharacter,
    pub faction: BlizzardTypeName,
    pub bracket: BlizzardPvPBracket,
    pub rating: u32,
    pub season: BlizzardKeyedIdReference,
    pub tier: BlizzardKeyedIdReference,
    pub season_match_statistics: BlizzardPvPMatchStatistics,
    pub weekly_match_statistics: BlizzardPvPMatchStatistics,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardPvPBracket {
    pub id: u32,
    pub r#type: String,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardKeyedIdReference {
    pub key: BlizzardLink,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ItemDetails {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub id: u32,
    pub name: String,
    pub quality: BlizzardTypeName,
    pub level: u32,
    pub required_level: u32,
    pub media: BlizzardAppearanceMediaReference,
    pub item_class: BlizzardAppearanceReference,
    pub item_subclass: BlizzardAppearanceReference,
    pub inventory_type: BlizzardTypeName,
    pub purchase_price: u64,
    pub sell_price: u64,
    pub max_count: u32,
    pub is_equippable: bool,
    pub is_stackable: bool,
    pub preview_item: Option<BlizzardItemPreview>,
    pub purchase_quantity: Option<u32>,
    pub appearances: Option<Vec<BlizzardKeyedIdReference>>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemPreview {
    pub item: BlizzardKeyedIdReference,
    pub context: Option<u32>,
    pub bonus_list: Option<Vec<u32>>,
    pub quality: BlizzardTypeName,
    pub name: String,
    pub media: BlizzardAppearanceMediaReference,
    pub item_class: BlizzardAppearanceReference,
    pub item_subclass: BlizzardAppearanceReference,
    pub inventory_type: BlizzardTypeName,
    pub binding: Option<BlizzardTypeName>,
    pub unique_equipped: Option<String>,
    pub weapon: Option<BlizzardItemWeapon>,
    pub stats: Option<Vec<BlizzardItemStat>>,
    pub spells: Option<Vec<BlizzardItemSpell>>,
    pub requirements: Option<BlizzardItemRequirements>,
    pub level: Option<BlizzardItemValueDisplayU32>,
    pub durability: Option<BlizzardItemValueDisplayU32>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemWeapon {
    pub damage: BlizzardItemDamage,
    pub attack_speed: BlizzardItemValueDisplayU32,
    pub dps: BlizzardItemValueDisplayF64,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemDamage {
    pub min_value: u32,
    pub max_value: u32,
    pub display_string: String,
    pub damage_class: BlizzardTypeName,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemStat {
    pub r#type: BlizzardTypeName,
    pub value: i32,
    pub is_negated: Option<bool>,
    pub display: BlizzardItemStatDisplay,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemStatDisplay {
    pub display_string: String,
    pub color: BlizzardColorRgba,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemSpell {
    pub spell: BlizzardAppearanceReference,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemRequirements {
    pub level: Option<BlizzardItemValueDisplayU32>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemValueDisplayU32 {
    pub value: u32,
    pub display_string: String,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardItemValueDisplayF64 {
    pub value: f64,
    pub display_string: String,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterEquipmentSummary {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub character: BlizzardAppearanceCharacter,
    #[serde(default)]
    pub equipped_items: Vec<BlizzardEquippedItem>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardEquippedItem {
    pub item: BlizzardKeyedIdReference,
    pub slot: BlizzardTypeName,
    pub quality: BlizzardTypeName,
    pub name: String,
    pub media: BlizzardAppearanceMediaReference,
    pub level: Option<BlizzardItemValueDisplayU32>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterAchievementsSummary {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub total_quantity: u32,
    pub total_points: u32,
    pub character: BlizzardAppearanceCharacter,
    #[serde(default)]
    pub recent_events: Vec<BlizzardAchievementEvent>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAchievementEvent {
    pub achievement: BlizzardAchievementReference,
    pub timestamp: u64,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardAchievementReference {
    pub key: BlizzardLink,
    pub name: String,
    pub id: u32,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterMountsCollection {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub character: BlizzardAppearanceCharacter,
    #[serde(default)]
    pub mounts: Vec<BlizzardCollectedMount>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardCollectedMount {
    pub mount: BlizzardAppearanceReference,
    pub is_useable: bool,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CharacterPetsCollection {
    #[serde(rename = "_links")]
    pub links: BlizzardLinks,
    pub character: BlizzardAppearanceCharacter,
    #[serde(default)]
    pub pets: Vec<BlizzardCollectedPet>,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlizzardCollectedPet {
    pub species: BlizzardAppearanceReference,
    pub level: u32,
    pub quality: BlizzardTypeName,
    #[serde(flatten)]
    pub extra_fields: BlizzardExtraFields,
}