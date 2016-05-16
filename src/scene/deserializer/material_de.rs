use serde;
use serde::de::{
    Error as SerdeError,
    Visitor,
    SeqVisitor,
    Deserialize,
    Deserializer,
};

use super::SceneFactory;
use ::vec3::Vec3;
use ::material::Material;
use ::material::materials::{
    FlatMaterial,
    CookTorranceMaterial,
    PhongMaterial,
};

const FLAT_MATERIAL_NAME: &'static str = "FlatMaterial";
const COOK_TORRANCE_MATERIAL_NAME: &'static str = "CookTorranceMaterial";
const PHONG_MATERIAL_NAME: &'static str = "PhongMaterial";

enum MaterialKind {
    Flat,
    CookTorrance,
    Phong,
}

impl Deserialize for MaterialKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<MaterialKind, D::Error>
        where D: Deserializer,
    {
        struct MaterialKindVisitor;

        impl Visitor for MaterialKindVisitor {
            type Value = MaterialKind;

            fn visit_str<E>(&mut self, value: &str) -> Result<MaterialKind, E>
                where E: serde::de::Error
            {
                match value {
                    FLAT_MATERIAL_NAME => Ok(MaterialKind::Flat),
                    COOK_TORRANCE_MATERIAL_NAME => Ok(MaterialKind::CookTorrance),
                    PHONG_MATERIAL_NAME => Ok(MaterialKind::Phong),
                    _ => Err(serde::de::Error::custom("unexpected value")),
                }
            }
        }
        
        deserializer.deserialize(MaterialKindVisitor)
    }
}

impl MaterialKind {
    fn get_material_factory<V>(&self, visitor: &mut V) -> Result<Box<MaterialFactory>, V::Error>
        where V: SeqVisitor
    {
        return match *self {
            MaterialKind::Flat => {
                visitor.visit().and_then(ensure_present_and_box::<FlatMaterialFactory, _>)
            },
            MaterialKind::CookTorrance => {
                visitor.visit().and_then(ensure_present_and_box::<CookTorranceMaterialFactory, _>)
            },
            MaterialKind::Phong => {
                visitor.visit().and_then(ensure_present_and_box::<PhongMaterialFactory, _>)
            }
        };

        #[inline]
        fn ensure_present_and_box<T, E>(mf: Option<T>) -> Result<Box<MaterialFactory>, E>
            where
                T: MaterialFactory + 'static,
                E: SerdeError
        {
            match mf {
                Some(fac) => Ok(Box::new(fac) as Box<MaterialFactory>),
                None => Err(SerdeError::invalid_length(1)),
            }
        }
    }
}

trait MaterialFactory: ::std::fmt::Debug {
    fn create(&self, context: SceneFactory) -> Box<Material+Send+Sync>;
}

#[derive(Deserialize, Debug)]
struct FlatMaterialFactory {
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

#[derive(Debug)]
pub struct MaterialFactoryWrapper(Box<MaterialFactory>);

impl Deserialize for MaterialFactoryWrapper {
    fn deserialize<D>(deserializer: &mut D) -> Result<MaterialFactoryWrapper, D::Error>
        where D: Deserializer,
    {
        struct MaterialVisitor;

        impl Visitor for MaterialVisitor {
            type Value = MaterialFactoryWrapper;

            #[inline]
            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<MaterialFactoryWrapper, V::Error>
                where V: SeqVisitor
            {
                let mat_kind: MaterialKind;
                if let Some(tmp_mat_kind) = try!(visitor.visit()) {
                    mat_kind = tmp_mat_kind;
                } else {
                    return Err(SerdeError::invalid_length(0));
                }
                let mat = try!(mat_kind.get_material_factory(&mut visitor));
                try!(visitor.end());
                Ok(MaterialFactoryWrapper(mat))
            }
        }

        deserializer.deserialize(MaterialVisitor)
    }
}
