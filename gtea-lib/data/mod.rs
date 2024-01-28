use crate::prelude::*;
use crate::config::Config;

/// Write a *Config* to the config file
///
/// - *config*: config to save
pub fn save_config(config: Config) -> Result<()> {
    let str = config.to_string()?;
    let path = Config::path()?;
    std::fs::write(path, str)?;
    Ok(())
}
