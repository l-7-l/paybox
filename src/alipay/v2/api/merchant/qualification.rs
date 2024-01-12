use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndustryQualification {
    pub industry_qualification_type: String,
    pub industry_qualification_image: String,
}
