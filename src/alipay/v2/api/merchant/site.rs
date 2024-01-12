use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfo {
    pub site_type: String,
    // pub screenshot_image: Option<String>,
    // pub tiny_app_id: Option<String>,
}
