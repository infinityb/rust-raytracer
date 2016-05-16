use std::collections::BTreeMap;

use serde;
use serde::de::{
    Error as SerdeError,
    Visitor,
    SeqVisitor,
    Deserialize,
    Deserializer,
};

use ::vec3::Vec3;
use ::raytracer::compositor::ColorRGBA;
use ::material::Texture;
use ::material::textures::{
    UVTexture,
};


mod material_de;
use self::material_de::{MaterialFactoryWrapper};

mod light_de;
use self::light_de::{LightFactoryWrapper};

mod texture_de;
use self::texture_de::{TextureFactoryWrapper};


#[derive(Deserialize, Debug)]
pub struct SceneFactory {
    lights: Vec<LightFactoryWrapper>,
    textures: BTreeMap<String, TextureFactoryWrapper>,
    materials: BTreeMap<String, MaterialFactoryWrapper>,
    // prims: PrimIterFactory,
}

enum SceneFactoryField {
    Lights,
    Textures,
    Materials,
    Prims,
}

impl Deserialize for SceneFactoryField {
    fn deserialize<D>(deserializer: &mut D) -> Result<SceneFactoryField, D::Error>
        where D: serde::de::Deserializer
    {
        struct SceneFactoryFieldVisitor;

        impl serde::de::Visitor for SceneFactoryFieldVisitor {
            type Value = SceneFactoryField;

            fn visit_str<E>(&mut self, value: &str) -> Result<SceneFactoryField, E>
                where E: serde::de::Error
            {
                match value {
                    "lights" => Ok(SceneFactoryField::Lights),
                    "textures" => Ok(SceneFactoryField::Textures),
                    "materials" => Ok(SceneFactoryField::Materials),
                    "prims" => Ok(SceneFactoryField::Prims),
                    _ => Err(serde::de::Error::custom("unexpected key")),
                }
            }
        }

        deserializer.deserialize(SceneFactoryFieldVisitor)
    }
}

impl SceneFactory {
    pub fn get_texture(&self, name: &str) -> Option<Box<Texture+Send+Sync>> {
        // FIXME
        Some(Box::new(UVTexture))
    }
}

struct SceneFactoryVisitor;

struct Vec3Visitor;

impl Visitor for Vec3Visitor {
    type Value = Vec3;

    #[inline]
    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Vec3, V::Error>
        where V: SeqVisitor
    {
        let mut items: [f64; 3] = [0.0, 0.0, 0.0];
        for (idx, item) in items.iter_mut().enumerate() {
            if let Some(val) = try!(visitor.visit()) {
                *item = val;
            } else {
                return Err(SerdeError::invalid_length(idx));
            }
        }
        
        try!(visitor.end());

        Ok(Vec3::xyz(items[0], items[1], items[2]))
    }

    #[inline]
    fn visit_str<E>(&mut self, value: &str) -> Result<Vec3, E>
        where E: serde::de::Error
    {
        match value {
            "zero" => Ok(Vec3::zero()),
            "one" => Ok(Vec3::one()),
            _ => Err(serde::de::Error::custom("unexpected value")),
        }
    }
}

impl Deserialize for Vec3 {
    fn deserialize<D>(deserializer: &mut D) -> Result<Vec3, D::Error>
        where D: Deserializer,
    {
        deserializer.deserialize(Vec3Visitor)
    }
}

/// ---

#[derive(Debug, Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

struct ColorVisitor;

impl From<Color> for ColorRGBA<f64> {
    fn from(c: Color) -> ColorRGBA<f64> {
        ColorRGBA::new_rgb(c.r, c.g, c.b)
    }
}

impl From<Color> for Vec3 {
    fn from(c: Color) -> Vec3 {
        Vec3::xyz(c.r, c.g, c.b)
    }
}

impl Visitor for ColorVisitor {
    type Value = Color;

    #[inline]
    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Color, V::Error>
        where V: SeqVisitor
    {
        let mut out = Color { r: 0.0, g: 0.0, b: 0.0 };
        let mut got = 0;

        if let Some(val) = try!(visitor.visit()) {
            out.r = val;
            got += 1;
        }
        if let Some(val) = try!(visitor.visit()) {
            out.g = val;
            got += 1;
        }
        if let Some(val) = try!(visitor.visit()) {
            out.b = val;
            got += 1;
        }
        if got != 3 {
            return Err(SerdeError::invalid_length(got));
        }
        try!(visitor.end());
        Ok(out)
    }

    #[inline]
    fn visit_str<E>(&mut self, value: &str) -> Result<Color, E>
        where E: serde::de::Error
    {
        let (r, g, b) = try!(parse_css_like(value));
        Ok(Color {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0
        })
    }
}

impl Deserialize for Color {
    fn deserialize<D>(deserializer: &mut D) -> Result<Color, D::Error>
        where D: Deserializer,
    {
        deserializer.deserialize(ColorVisitor)
    }
}

fn parse_css_like<E>(css_like: &str) -> Result<(u8, u8, u8), E>
    where E: SerdeError
{
    unimplemented!();
}