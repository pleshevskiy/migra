use crate::config::Config;
use crate::config::MIGRA_TOML_FILENAME;
use crate::StdResult;
use std::path::PathBuf;

pub(crate) fn initialize_migra_manifest(config_path: Option<PathBuf>) -> StdResult<()> {
    let config_path = config_path
        .map(|mut config_path| {
            let ext = config_path.extension();
            if config_path.is_dir() || ext.is_none() {
                config_path.push(MIGRA_TOML_FILENAME);
            }

            config_path
        })
        .unwrap_or_else(|| PathBuf::from(MIGRA_TOML_FILENAME));

    if config_path.exists() {
        println!("{} already exists", config_path.to_str().unwrap());
        return Ok(());
    }

    if let Some(dirs) = config_path.parent() {
        std::fs::create_dir_all(dirs)?;
    }

    let config = Config::default();
    let content = toml::to_string(&config)?;
    std::fs::write(&config_path, content)?;

    println!("Created {}", config_path.to_str().unwrap());

    Ok(())
}
