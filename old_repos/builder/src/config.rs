use std::{collections::HashMap, fmt};
use serde::{Serialize, Deserialize, de::{Visitor, self}, Deserializer};

use crate::palette::Palette;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub palette: Palette,
    pub shades: HashMap<String, f32>,
    pub colors: HashMap<String, ColorNames>,
}

static DEFAULT_CONFIG: &'static str = include_str!(concat!(env!("OUT_DIR"), "/default_config.json"));

impl Config {
    pub fn default() -> Config {
        serde_json::from_str(DEFAULT_CONFIG).unwrap()
    }
    pub fn from_palette(palette: Palette) -> Config {
        let mut config = Self::default();
        config.palette = palette;
        config
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Shades {
    pub p10: f32,
    pub p25: f32,
    pub p50: f32,
    pub p75: f32,
    pub p100: f32,
    pub p125: f32,
}

#[derive(Debug, PartialEq)]
pub enum ColorNames {
    BuiltIn,
    Reference(Reference),
    Mapping(HashMap<String, ColorNames>),
}

impl Serialize for ColorNames {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ColorNames::BuiltIn => serializer.serialize_str("BUILT_IN"),
            ColorNames::Reference(s) => serializer.serialize_str(&s.string),
            ColorNames::Mapping(hash) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(hash.len()))?;
                for (k, v) in hash {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

struct ColorNamesVisitor;

impl<'de> Visitor<'de> for ColorNamesVisitor {
    type Value = ColorNames;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or map")
    }

    fn visit_str<E>(self, s: &str) -> Result<ColorNames, E>
    where
        E: de::Error,
    {
        if s == "BUILT_IN" {
            return Ok(ColorNames::BuiltIn);
        }
        if !const_regex::match_regex!(r"[a-z][0-9a-z_]*(\.[a-z][0-9a-z_]*)*", s.as_bytes()) {
            return Err(E::custom("invalid color name"));
        }
        Ok(ColorNames::Reference(Reference {string: s.to_string()}))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<ColorNames, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let mut values = HashMap::new();

        while let Some((key, value)) = visitor.next_entry()? {
            values.insert(key, value);
        }

        Ok(ColorNames::Mapping(values))
    }
}

impl<'de> Deserialize<'de> for ColorNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ColorNamesVisitor)
    }
}


#[derive(Debug, PartialEq)]
pub struct Reference {
    string: String,
}

impl Reference {
    pub fn key_iter<'a>(&'a self) -> std::str::Split<&str>
    {
        self.string.split(".")
    }
}