#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StarRailInfoParsed {
    pub player: Player,
    pub characters: Vec<Character>,
}

#[derive(Deserialize)]
pub struct Player {
    pub uid: String,
    #[serde(rename = "nickname")]
    pub name: String,
    pub level: u8,
    pub world_level: u8,
    pub friend_count: u8,
    pub avatar: Avatar,
    pub signature: String,
    pub is_display: bool,
    pub space_info: SpaceInfo,
}

#[derive(Deserialize)]
pub struct Avatar {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct SpaceInfo {
    pub memory_data: ForgottenHall,
    pub universe_level: u8,
    pub avatar_count: u16,
    pub light_cone_count: u16,
    pub relic_count: u16,
    pub achievement_count: u16,
    pub book_count: u16,
    pub music_count: u16,
}

#[derive(Deserialize)]
pub struct ForgottenHall {
    pub level: u8,
    pub chaos_id: Option<String>, 
    pub chaos_level: u8,
    pub chaos_star_count: u8,
}

#[derive(Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub level: u8,
    #[serde(rename = "promotion")]
    pub ascension: u8,
    #[serde(rename = "rank")]
    pub eidolon: u8,
    #[serde(rename = "rank_icons")]
    pub eidolon_icons: Vec<String>,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: Path,
    pub element: Element,
    #[serde(rename = "skills")]
    pub traces: Vec<Trace>,
    #[serde(rename = "skill_trees")]
    pub trace_tree: Vec<TraceTreeNode>,
    pub light_cone: Option<LightCone>,
    pub relics: Vec<Relic>,
    pub relic_sets: Vec<RelicSet>,
    pub attributes: Vec<Attribute>,
    pub additions: Vec<Attribute>,
    pub properties: Vec<Property>,
}

impl Character {
    pub fn max_level(&self) -> u8 {
        20 + 10 * self.ascension
    }
}

#[derive(Deserialize)]
pub struct Path {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct Trace {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub max_level: u8,
    pub element: Option<Element>,
    #[serde(rename = "type")]
    pub trace_type: String,
    pub type_text: String,
    pub effect: String,
    pub effect_text: String,
    pub simple_desc: String,
    pub desc: String,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct TraceTreeNode {
    pub id: String,
    pub level: u8,
    pub max_level: u8,
    pub icon: String,
    pub anchor: String,
    pub parent: Option<String>,
}

#[derive(Deserialize)]
pub struct LightCone {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    #[serde(rename = "rank")]
    pub superimpose: u8,
    pub level: u8,
    #[serde(rename = "promotion")]
    pub ascension: u8,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
    pub path: Path,
    pub attributes: Vec<Attribute>,
    pub properties: Vec<Property>,
}

impl LightCone {
    pub fn max_level(&self) -> u8 {
        20 + 10 * self.ascension
    }
}

#[derive(Deserialize)]
pub struct Relic {
    pub id: String,
    pub name: String,
    pub set_id: String,
    pub set_name: String,
    pub rarity: u8,
    pub level: u8,
    pub main_affix: MainAffix,
    #[serde(rename = "sub_affix")]
    pub sub_affixes: Vec<SubAffix>,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct RelicSet {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub num: u8,
    pub desc: String,
    pub properties: Vec<Property>,
}

#[derive(Deserialize)]
pub struct Attribute {
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    #[serde(rename = "display")]
    pub displayed_value: String,
    #[serde(rename = "percent")]
    pub is_percent: bool,
}

#[derive(Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub property_type: String,
    pub field: String,
    pub name: String,
    pub icon: String,
    pub value: f64,
    #[serde(rename = "display")]
    pub displayed_value: String,
    #[serde(rename = "percent")]
    pub is_percent: bool,
}

#[derive(Deserialize)]
pub struct MainAffix {
    #[serde(flatten)]
    pub property: Property,
}

#[derive(Deserialize)]
pub struct SubAffix {
    #[serde(flatten)]
    pub property: Property,
    pub count: u8,
    pub step: u8,
}

pub enum Language {
    CHT,
    CHS,
    DE,
    EN,
    ES,
    FR,
    ID,
    JP,
    KR,
    PT,
    RU,
    TH,
    VI,
}

impl Language {
    pub fn value(&self) -> &'static str {
        match self {
            Language::CHT => "cht",
            Language::CHS => "cn",
            Language::DE => "de",
            Language::EN => "en",
            Language::ES => "es",
            Language::FR => "fr",
            Language::ID => "id",
            Language::JP => "jp",
            Language::KR => "kr",
            Language::PT => "pt",
            Language::RU => "ru",
            Language::TH => "th",
            Language::VI => "vi",
        }
    }
}

pub async fn mihomo(uid: u32, lang: Language) -> Result<StarRailInfoParsed, Box<dyn std::error::Error>> {
    let url = format!("https://api.mihomo.me/sr_info_parsed/{}?lang={}", uid, lang.value());

    let response = reqwest::get(&url).await?.text().await?;
    
    let star_rail_info_parsed: StarRailInfoParsed = serde_json::from_str(&response)?;

    Ok(star_rail_info_parsed)
}