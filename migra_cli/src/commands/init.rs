use crate::app::App;
use crate::config::{Config, MIGRA_TOML_FILENAME};
use std::path::PathBuf;

pub(crate) fn initialize_migra_manifest(app: &App) -> migra::StdResult<()> {
    let config_path = app.config_path().cloned().map_or_else(
        || PathBuf::from(MIGRA_TOML_FILENAME),
        |mut config_path| {
            let ext = config_path.extension();
            if config_path.is_dir() || ext.is_none() {
                config_path.push(MIGRA_TOML_FILENAME);
            }

            config_path
        },
    );

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
