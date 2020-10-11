use crate::texture;
use crate::validation::Checked;
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
use crate::{material::StrengthFactor, validation::Validate, Extras};
use gltf_derive::Validate;
use serde::de;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// The material appearance of a primitive.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Material {
    #[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
    #[serde(
        default,
        rename = "KHR_materials_pbrSpecularGlossiness",
        skip_serializing_if = "Option::is_none"
    )]
    pub pbr_specular_glossiness: Option<PbrSpecularGlossiness>,
    #[cfg(feature = "KHR_materials_unlit")]
    #[serde(
        default,
        rename = "KHR_materials_unlit",
        skip_serializing_if = "Option::is_none"
    )]
    pub unlit: Option<Unlit>,
    #[serde(
        default,
        rename = "EXT_pbr_attributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub ext_pbr_attributes: Option<PbrAttributes>,
    #[serde(default, rename = "AA_shadow", skip_serializing_if = "Option::is_none")]
    pub aa_shadow: Option<AAShadow>,
}

/// A set of parameter values that are used to define the metallic-roughness
/// material model from Physically-Based Rendering (PBR) methodology.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct PbrMetallicRoughness {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColorSpace {
    SRGB,
    Linear,
}

pub const VALID_COLOR_SPACES: &'static [&'static str] = &["sRGB", "linear"];

impl serde::Serialize for ColorSpace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            ColorSpace::SRGB => serializer.serialize_str("sRGB"),
            ColorSpace::Linear => serializer.serialize_str("linear"),
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<ColorSpace> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<ColorSpace>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_COLOR_SPACES)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::ColorSpace::*;
                use crate::validation::Checked::*;
                Ok(match value {
                    "sRGB" => Valid(SRGB),
                    "linear" => Valid(Linear),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl Default for ColorSpace {
    fn default() -> Self {
        ColorSpace::Linear
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct PbrAttributes {
    pub base_color_attrib_space: Checked<ColorSpace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_attrib: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roughness_attrib: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_attrib: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusion_attrib: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_attrib: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_attrib: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct AAShadow {
    pub shadow_texture: Option<texture::Info>,
}

/// A set of parameter values that are used to define the specular-glossiness
/// material model from Physically-Based Rendering (PBR) methodology.
///
/// This model supports more materials than metallic-roughness, at the cost of
/// increased memory use. When both are available, specular-glossiness should be
/// preferred.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
#[serde(default, rename_all = "camelCase")]
pub struct PbrSpecularGlossiness {
    /// The material's diffuse factor.
    ///
    /// The RGBA components of the reflected diffuse color of the
    /// material. Metals have a diffuse value of `[0.0, 0.0, 0.0]`. The fourth
    /// component (A) is the alpha coverage of the material. The `alphaMode`
    /// property specifies how alpha is interpreted. The values are linear.
    pub diffuse_factor: PbrDiffuseFactor,

    /// The diffuse texture.
    ///
    /// This texture contains RGB(A) components of the reflected diffuse color
    /// of the material in sRGB color space. If the fourth component (A) is
    /// present, it represents the alpha coverage of the material. Otherwise, an
    /// alpha of 1.0 is assumed. The `alphaMode` property specifies how alpha is
    /// interpreted. The stored texels must not be premultiplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse_texture: Option<texture::Info>,

    /// The material's specular factor.
    pub specular_factor: PbrSpecularFactor,

    /// The glossiness or smoothness of the material.
    ///
    /// A value of 1.0 means the material has full glossiness or is perfectly
    /// smooth. A value of 0.0 means the material has no glossiness or is
    /// completely rough. This value is linear.
    pub glossiness_factor: StrengthFactor,

    /// The specular-glossiness texture.
    ///
    /// A RGBA texture, containing the specular color of the material (RGB
    /// components) and its glossiness (A component). The values are in sRGB
    /// space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular_glossiness_texture: Option<texture::Info>,

    /// Optional application specific data.
    #[cfg_attr(feature = "extras", serde(skip_serializing_if = "Option::is_none"))]
    pub extras: Extras,
}

/// Defines the normal texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct NormalTexture {}

/// Defines the occlusion texture of a material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct OcclusionTexture {}

/// The diffuse factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrDiffuseFactor(pub [f32; 4]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrDiffuseFactor {
    fn default() -> Self {
        PbrDiffuseFactor([1.0, 1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrDiffuseFactor {}

/// The specular factor of a material.
#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PbrSpecularFactor(pub [f32; 3]);

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Default for PbrSpecularFactor {
    fn default() -> Self {
        PbrSpecularFactor([1.0, 1.0, 1.0])
    }
}

#[cfg(feature = "KHR_materials_pbrSpecularGlossiness")]
impl Validate for PbrSpecularFactor {}

/// Empty struct that should be present for primitives which should not be shaded with the PBR shading model.
#[cfg(feature = "KHR_materials_unlit")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Unlit {}
