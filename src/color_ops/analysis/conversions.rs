//! Type conversion logic and serializable color representations
//!
//! Provides serializable color types for JSON/YAML output and conversion
//! utilities between color spaces for analysis purposes.

use palette::{Hsl, Hsv, Lab, Lch, Srgb};
use serde::{Deserialize, Serialize};

/// Serializable RGB color representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableRgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl From<Srgb> for SerializableRgb {
    fn from(srgb: Srgb) -> Self {
        Self {
            red: srgb.red,
            green: srgb.green,
            blue: srgb.blue,
        }
    }
}

impl From<SerializableRgb> for Srgb {
    fn from(rgb: SerializableRgb) -> Self {
        Srgb::new(rgb.red, rgb.green, rgb.blue)
    }
}

/// Serializable HSL color representation  
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableHsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl From<Hsl> for SerializableHsl {
    fn from(hsl: Hsl) -> Self {
        Self {
            hue: hsl.hue.into_inner(),
            saturation: hsl.saturation,
            lightness: hsl.lightness,
        }
    }
}

/// Serializable HSV color representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableHsv {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl From<Hsv> for SerializableHsv {
    fn from(hsv: Hsv) -> Self {
        Self {
            hue: hsv.hue.into_inner(),
            saturation: hsv.saturation,
            value: hsv.value,
        }
    }
}

/// Serializable LAB color representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableLab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl From<Lab> for SerializableLab {
    fn from(lab: Lab) -> Self {
        Self {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        }
    }
}

/// Serializable LCH color representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableLch {
    pub l: f32,
    pub chroma: f32,
    pub hue: f32,
}

impl From<Lch> for SerializableLch {
    fn from(lch: Lch) -> Self {
        Self {
            l: lch.l,
            chroma: lch.chroma,
            hue: lch.hue.into_inner(),
        }
    }
}

/// Color space representations for analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorSpaces {
    pub hsl: SerializableHsl,
    pub hsv: SerializableHsv,
    pub lab: SerializableLab,
    pub lch: SerializableLch,
}

/// Get color space representations
pub fn get_color_spaces(color: Srgb) -> ColorSpaces {
    use crate::color_ops::conversion;

    ColorSpaces {
        hsl: conversion::srgb_to_hsl(color).into(),
        hsv: conversion::srgb_to_hsv(color).into(),
        lab: conversion::srgb_to_lab(color).into(),
        lch: conversion::srgb_to_lch(color).into(),
    }
}
