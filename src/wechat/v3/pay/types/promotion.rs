use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PromotionScope {
    Global,
    Single,
}
