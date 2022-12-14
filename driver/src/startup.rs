use anyhow::{Context as _, Result};
use config::{Config, File, FileFormat};

use crate::config::GracroverseConfig;

pub fn load_config() -> Result<GracroverseConfig> {
    let config = Config::builder()
        .add_source(File::new("config.yml", FileFormat::Yaml))
        .build()
        .context("Could not load config from `config.yml`")?;

    let config = config
        .try_deserialize::<GracroverseConfig>()
        .context("Could not deserialize config from `config.yml`")?;

    Ok(config)
}
