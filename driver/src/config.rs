use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GracroverseConfig {
    pub host: String,
    #[serde_as(as = "DisplayFromStr")]
    pub log_level: tracing::Level,
}
