use serde::de::DeserializeOwned;

pub trait TomlConfig: DeserializeOwned {
    const DEFAULT_TOML: &str;

    fn load_or_create(path: &str) -> Self {
        std::fs::read_to_string(path).map_or_else(
            |_| {
                std::fs::write(path, Self::DEFAULT_TOML).unwrap();
                toml::from_str(Self::DEFAULT_TOML).unwrap()
            },
            |data| toml::from_str(&data).unwrap(),
        )
    }
}
