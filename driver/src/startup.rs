use anyhow::{Context as _, Result};
use config::{Config, File, FileFormat};

use crate::config::GracroverseConfig;

pub fn init_app() -> Result<GracroverseConfig> {
    println!("{:?}", std::env::current_dir());

    let config = Config::builder()
        .add_source(File::new("config.yml", FileFormat::Yaml))
        .build()
        .context("Could not load config from `config.yml`")?;

    let config = config
        .try_deserialize::<GracroverseConfig>()
        .context("Could not deserialize config from `config.yml`")?;

    tracing_subscriber::fmt()
        .with_max_level(config.log_level)
        .finish();

    Ok(config)
}
