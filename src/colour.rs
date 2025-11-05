use anyhow::Context;
use raqote::SolidSource;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[serde(try_from = "String")]
pub struct Colour(u32);

// much of this implementation is borrowed from yofi under the MIT license
// copyright 2018 kitsu
impl Colour {
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(u32::from_be_bytes([r, g, b, a]))
    }

    pub const fn to_rgba(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    pub fn as_source(&self) -> SolidSource {
        let [r, g, b, a] = self.to_rgba();
        SolidSource::from_unpremultiplied_argb(a, r, g, b)
    }

    pub fn as_cosmic(&self) -> cosmic_text::Color {
        let [r, g, b, a] = self.to_rgba();
        cosmic_text::Color::rgba(r, g, b, a)
    }
}

impl TryFrom<String> for Colour {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let decoded = u32::from_str_radix(value.as_str(), 16).context("parse hex number");
        Ok(Self(match (decoded, value.len()) {
            (Ok(d), 6) => d << 8 | 0xff,
            (e, _) => anyhow::bail!("hex color can only be specified in RRGGBB format, {e:?}"),
        }))
    }
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Colours {
    One(Colour),
    Many(Vec<Colour>),
}

impl Colours {
    pub fn cycle(&self) -> Box<dyn Iterator<Item = &Colour> + '_> {
        match self {
            Colours::One(c) => Box::new(std::iter::repeat(c)),
            Colours::Many(cs) => Box::new(cs.iter().cycle()),
        }
    }
}
