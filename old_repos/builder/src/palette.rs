use std::{fmt, str::FromStr};
use ext_palette::Srgb;
use itertools::Itertools;
use serde::{Serialize, de::{Visitor, self}, Deserialize, Deserializer};

use crate::{Color, generator};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Palette {
    pub colors: [Color;9]
}

pub struct PaletteOption {
    pub colors: [Option<Color>;9]
}

impl PaletteOption {
    pub fn new() -> PaletteOption {
        PaletteOption {
            colors: [None; 9]
        }
    }
}

impl fmt::Display for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.colors.map(|c| format!("{:x}", c)).join("-");
        f.write_str(&s)?;
        Ok(())
    }
}

impl fmt::Display for PaletteOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.colors.map(|co| co.map_or("".to_string(), |c| format!("{:x}", c))).join("-");
        f.write_str(&s)?;
        Ok(())
    }
}

impl FromStr for Palette {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let palette_option = PaletteOption::from_str(s)?;
        Ok(generator::generate(&palette_option))
    }
}

impl FromStr for PaletteOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut palette_option = PaletteOption::new();
        let len = s.split('-').count();
        if len > 9 {
            return Err(format!("too many colors: {}", len));
        }
        for (i, c) in s.split('-').enumerate() {
            match c {
                "_" => (),
                "?" => {
                    return Ok(palette_option);
                },
                c if const_regex::match_regex!(r"[0-9a-fA-F]{6}", c.as_bytes()) => {
                    let r = u8::from_str_radix(&c[0..2], 16).unwrap();
                    let g = u8::from_str_radix(&c[2..4], 16).unwrap();
                    let b = u8::from_str_radix(&c[4..6], 16).unwrap();
                    palette_option.colors[i] = Some(Srgb::new(r,g,b));
                },
                _ => {
                    return Err(format!("{}th color in palette is wrong format: {}", i, c));
                }
            }
        }
        if len != 9 {
            return Err(format!("wrong number of colors: {}", len));
        }
        Ok(palette_option)
    }
}

impl Serialize for Palette {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for PaletteOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct PaletteVisitor;

impl<'de> Visitor<'de> for PaletteVisitor {
    type Value = Palette;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("base9 palette code")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::from_str(s).map_err(|x| E::custom(x))
    }
}

struct PaletteOptionVisitor;

impl<'de> Visitor<'de> for PaletteOptionVisitor {
    type Value = PaletteOption;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("base9 palette code")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::from_str(s).map_err(|x| E::custom(x))
    }
}

impl<'de> Deserialize<'de> for Palette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PaletteVisitor)
    }
}

impl<'de> Deserialize<'de> for PaletteOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PaletteOptionVisitor)
    }
}

#[test]
fn from_str_works() {
    let palette_str = "000000-ffffff-222222-333333-444444-555555-666666-777777-888888";
    let palette = Palette::from_str(palette_str).unwrap();
    assert_eq!(palette.colors.iter().map(|x| format!("{:x}", x)).join("-"), palette_str);
}
#[test]
fn from_str_works_with_underscore() {
    let palette_str = [
        "_",
        "ffffff",
        "_",
        "333333",
        "444444",
        "555555",
        "666666",
        "777777",
        "888888",
    ];
    let palette = Palette::from_str(&palette_str.join("-")).unwrap();
    let actual: Vec<String> = palette.colors.iter().map(|x| format!("{:x}", x)).collect();
    palette_str.iter().zip_eq(actual).for_each(|(&e, a)| {
        match e {
            "_" => (),
            _ => assert_eq!(*e, a),
        }
    });
}

#[test]
fn from_str_works_with_question_mark() {
    let palette_str = [
        "_",
        "ffffff",
        "_",
        "333333",
        "444444",
        "555555",
        "?"
    ];
    let palette = Palette::from_str(&palette_str.join("-")).unwrap();
    let actual: Vec<String> = palette.colors.iter().map(|x| format!("{:x}", x)).collect();
    palette_str.iter().zip(actual).for_each(|(&e, a)| {
        match e {
            "_" => (),
            "?" => (),
            _ => assert_eq!(*e, a),
        }
    });
}