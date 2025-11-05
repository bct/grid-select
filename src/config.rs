use crate::colour;
use anyhow::{Context, Result};
use defaults::Defaults;
use serde::Deserialize;
use std::path::PathBuf;

const DEFAULT_CONFIG_NAME: &str = "config.toml";

fn default_config_path() -> Result<Option<PathBuf>> {
    let file = xdg::BaseDirectories::with_prefix(crate::prog_name!())
        .get_config_file(DEFAULT_CONFIG_NAME)
        .context("failed to get xdg dirs")?;
    if file
        .try_exists()
        .with_context(|| format!("reading default config at {}", file.display()))?
    {
        Ok(Some(file))
    } else {
        Ok(None)
    }
}

mod config_defaults {
    use crate::colour;

    pub fn bg_colour() -> colour::Colours {
        colour::Colours::One(colour::Colour::from_rgba(0x3c, 0x38, 0x36, 0xff))
    }

    pub fn fg_colour() -> colour::Colour {
        colour::Colour::from_rgba(0xa8, 0x99, 0x84, 0xff)
    }

    pub fn active_bg_colour() -> colour::Colour {
        colour::Colour::from_rgba(0xfa, 0xbd, 0x2f, 0xff)
    }

    pub fn active_fg_colour() -> colour::Colour {
        colour::Colour::from_rgba(0x28, 0x28, 0x28, 0xff)
    }

    pub fn border_colour() -> colour::Colour {
        colour::Colour::from_rgba(0xeb, 0xdb, 0xb2, 0xff)
    }
}

#[derive(Defaults, Deserialize)]
#[serde(default)]
pub struct Config {
    #[def = "100."]
    pub item_width: f32,
    #[def = "50."]
    pub item_height: f32,
    #[def = "10."]
    pub item_margin: f32,

    #[def = "1."]
    pub border_width: f32,

    #[def = "config_defaults::bg_colour()"]
    pub bg_colour: colour::Colours,

    #[def = "config_defaults::fg_colour()"]
    pub fg_colour: colour::Colour,

    #[def = "config_defaults::active_bg_colour()"]
    pub active_bg_colour: colour::Colour,

    #[def = "config_defaults::active_fg_colour()"]
    pub active_fg_colour: colour::Colour,

    #[def = "config_defaults::border_colour()"]
    pub border_colour: colour::Colour,

    #[def = "18."]
    pub font_size: f32,
    pub font_name: Option<String>,
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let path = match path {
            Some(p) => p,
            None => match default_config_path()? {
                Some(path) => path,
                None => return Ok(Config::default()),
            },
        };
        match std::fs::read_to_string(&path) {
            Ok(c) => toml::from_str(&c).context("invalid config"),
            Err(err) if matches!(err.kind(), std::io::ErrorKind::NotFound) => Ok(Config::default()),
            Err(e) => {
                Err(anyhow::Error::new(e).context(format!("config read at {}", path.display())))
            }
        }
    }

    pub fn param<'a, T>(&'a self) -> T
    where
        T: From<&'a Self>,
    {
        self.into()
    }
}
