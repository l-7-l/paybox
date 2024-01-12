use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliMerchantContact {
    pub name: String,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

impl AliMerchantContact {
    pub fn with_mobile(name: String, mobile: String) -> Self {
        AliMerchantContact {
            name,
            phone: None,
            mobile: Some(mobile),
            email: None,
        }
    }
}
