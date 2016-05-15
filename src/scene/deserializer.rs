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
use ::material::Material;
use ::material::materials::{
    FlatMaterial,
    CookTorranceMaterial,
    PhongMaterial,
};
use ::material::Texture;
use ::material::textures::{
    UVTexture,
};

enum MaterialKind {
    FlatMaterial,
    CookTorranceMaterial,
    PhongMaterial,
}

impl Deserialize for MaterialKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<MaterialKind, D::Error>
        where D: Deserializer,
    {
        deserializer.deserialize(MaterialKindVisitor)
    }
}

struct MaterialKindVisitor;

impl Visitor for MaterialKindVisitor {
    type Value = MaterialKind;

    fn visit_str<E>(&mut self, value: &str) -> Result<MaterialKind, E>
        where E: serde::de::Error
    {
        match value {
            "FlatMaterial" => Ok(MaterialKind::FlatMaterial),
            "CookTorranceMaterial" => Ok(MaterialKind::CookTorranceMaterial),
            "PhongMaterial" => Ok(MaterialKind::PhongMaterial),
            _ => Err(serde::de::Error::custom("unexpected value")),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SceneFactory {
    // lights: BTreeMap<String, Box<LightFactory>>,
    // textures: BTreeMap<String, Box<TextureFactory>>,
    materials: BTreeMap<String, MatFac>,
    // prims: BTreeMap<String, Box<PrimFactory>>,
}

impl SceneFactory {
    pub fn get_texture(&self, name: &str) -> Option<Box<Texture+Send+Sync>> {
        // FIXME
        Some(Box::new(UVTexture))
    }
}

struct SceneFactoryVisitor;

pub trait MaterialFactory: ::std::fmt::Debug {
    fn create(&self, context: SceneFactory) -> Box<Material+Send+Sync>;
}

#[derive(Deserialize, Debug)]
pub struct FlatMaterialFactory {
    color: Vec3
}

impl MaterialFactory for FlatMaterialFactory {
    fn create(&self, _context: SceneFactory) -> Box<Material+Send+Sync> {
        Box::new(FlatMaterial { color: self.color })
    }
}

#[derive(Deserialize, Debug)]
struct CookTorranceMaterialFactory {
    k_a: f64,            // Ambient coefficient
    k_d: f64,            // Diffuse coefficient
    k_s: f64,            // Local specular coefficient
    k_sg: f64,           // Global specular coefficient (mirror reflection)
    k_tg: f64,           // Global transmissive coefficient (refraction)
    ambient: Vec3,       // Ambient color
    diffuse: Vec3,       // Diffuse color
    transmission: Vec3,  // Transmissive color
    specular: Vec3,      // Specular color
    roughness: f64,      // Smaller = shininer => smaller highlight spot on surface
    glossiness: f64,     // How glossy reflections are. 0 for non-glossy surfaces.
    gauss_constant: f64, // Controls curve of distribution of microfacets
    ior: f64,            // Index of refraction, also used for specular highlights
    diffuse_texture: Option<String>
}

impl MaterialFactory for CookTorranceMaterialFactory {
    fn create(&self, context: SceneFactory) -> Box<Material+Send+Sync> {
        let mut texture = None;
        if let Some(ref texture_name) = self.diffuse_texture {
            texture = Some(context.get_texture(texture_name).unwrap());
        }
        Box::new(CookTorranceMaterial {
            k_a: self.k_a,
            k_d: self.k_d,
            k_s: self.k_s,
            k_sg: self.k_sg,
            k_tg: self.k_tg,
            ambient: self.ambient,
            diffuse: self.diffuse,
            transmission: self.transmission,
            specular: self.specular,
            roughness: self.roughness,
            glossiness: self.glossiness,
            gauss_constant: self.gauss_constant,
            ior: self.ior,
            diffuse_texture: texture,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct PhongMaterialFactory {
    k_a: f64,           // Ambient coefficient
    k_d: f64,           // Diffuse coefficient
    k_s: f64,           // Local specular coefficient
    k_sg: f64,          // Global specular coefficient (mirror reflection)
    k_tg: f64,          // Global transmissive coefficient (refraction)
    ambient: Vec3,      // Ambient color
    diffuse: Vec3,      // Diffuse color
    transmission: Vec3, // Transmissive color
    specular: Vec3,     // Specular color
    shininess: f64,     // Size of Phong specular highlight
    glossiness: f64,    // How glossy reflections are. 0 for non-glossy surfaces.
    ior: f64,           // Index of refraction
    diffuse_texture: Option<String>
}

impl MaterialFactory for PhongMaterialFactory {
    fn create(&self, context: SceneFactory) -> Box<Material+Send+Sync> {
        let mut texture = None;
        if let Some(ref texture_name) = self.diffuse_texture {
            texture = Some(context.get_texture(texture_name).unwrap());
        }
        Box::new(PhongMaterial {
            k_a: self.k_a,
            k_d: self.k_d,
            k_s: self.k_s,
            k_sg: self.k_sg,
            k_tg: self.k_tg,
            ambient: self.ambient,
            diffuse: self.diffuse,
            transmission: self.transmission,
            specular: self.specular,
            shininess: self.shininess,
            glossiness: self.glossiness,
            ior: self.ior,
            diffuse_texture: texture,
        })
    }
}

impl MaterialKind {
    fn get_material<V>(&self, visitor: &mut V) -> Result<Box<MaterialFactory>, V::Error>
        where V: SeqVisitor
    {
        use self::MaterialKind as MK;

        return match *self {
            MK::FlatMaterial => {
                let mat_fac: Result<Option<FlatMaterialFactory>, _> = visitor.visit();
                mat_fac.and_then(|mf| match mf {
                    Some(fac) => Ok(Box::new(fac) as Box<MaterialFactory>),
                    None => Err(SerdeError::invalid_length(1)),
                })
            },
            MK::CookTorranceMaterial => {
                let mat_fac: Result<Option<CookTorranceMaterialFactory>, _> = visitor.visit();
                mat_fac.and_then(|mf| match mf {
                    Some(fac) => Ok(Box::new(fac) as Box<MaterialFactory>),
                    None => Err(SerdeError::invalid_length(1)),
                })
            },
            MK::PhongMaterial => {
                let mat_fac: Result<Option<PhongMaterialFactory>, _> = visitor.visit();
                mat_fac.and_then(|mf| match mf {
                    Some(fac) => Ok(Box::new(fac) as Box<MaterialFactory>),
                    None => Err(SerdeError::invalid_length(1)),
                })
            }
        };
    }
}

struct MaterialVisitor;

impl Visitor for MaterialVisitor {
    type Value = MatFac;

    #[inline]
    fn visit_seq<V>(&mut self, mut visitor: V) -> Result<MatFac, V::Error>
        where V: SeqVisitor
    {
        let mat_kind: MaterialKind;
        if let Some(tmp_mat_kind) = try!(visitor.visit()) {
            mat_kind = tmp_mat_kind;
        } else {
            return Err(SerdeError::invalid_length(0));
        }
        let mat: Box<MaterialFactory> = try!(mat_kind.get_material(&mut visitor));
        try!(visitor.end());
        Ok(MatFac(mat))
    }
}

#[derive(Debug)]
struct MatFac(Box<MaterialFactory>);

impl Deserialize for MatFac {
    fn deserialize<D>(deserializer: &mut D) -> Result<MatFac, D::Error>
        where D: Deserializer,
    {
        deserializer.deserialize(MaterialVisitor)
    }
}

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

