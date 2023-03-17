use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_of<T: Hash>(t: T) -> u64 {
    let mut state = DefaultHasher::new();
    t.hash(&mut state);

    state.finish()
}

use std::{fs::File, io::Write};

use anyhow::Context;

use crate::settings::Settings;

pub fn is_first_run(path: &str) -> bool {
    !std::path::Path::new(path).exists()
}

pub fn create_default_file(path: &str, toml_settings: Settings) -> anyhow::Result<()> {
    let mut file =
        File::create(path).with_context(|| "Failed to create default configuration file")?;

    let data = toml::to_string_pretty(&toml_settings)
        .with_context(|| "Failed to serialize default settings")?;

    file.write(data.as_bytes())
        .with_context(|| "Failed to write to default configuration file")?;

    Ok(())
}
