mod materials {
    pub mod cooktorrancematerial;
    pub mod flatmaterial;
    pub mod phongmaterial;
}

pub use self::materials::cooktorrancematerial::CookTorranceMaterial;
pub use self::materials::flatmaterial::FlatMaterial;
pub use self::materials::phongmaterial::PhongMaterial;

mod textures {
    pub mod checkertexture;
    pub mod uvtexture;
    pub mod imagetexture;
    pub mod cubemap;
}

pub use self::textures::checkertexture::CheckerTexture;
pub use self::textures::uvtexture::UVTexture;
pub use self::textures::imagetexture::ImageTexture;
pub use self::textures::cubemap::CubeMap;

use vec3::Vec3;
use raytracer::compositor::ColorRGBA;

/// TODO: Move specular/transmissive properties into traits
pub trait Material: Send + Sync {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> Vec3;
    fn is_reflective(&self) -> bool;
    fn is_refractive(&self) -> bool;
    fn global_specular(&self, color: &Vec3) -> Vec3;
    fn global_transmissive(&self, color: &Vec3) -> Vec3;
    fn transmission(&self) -> Vec3;
    fn ior(&self) -> f64;
    fn is_glossy(&self) -> bool;
    fn glossiness(&self) -> f64;
}

impl Material for Box<Material> {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> Vec3 {
        (**self).sample(n, i, l, u, v)
    }

    fn is_reflective(&self) -> bool {
        (**self).is_reflective()
    }

    fn is_refractive(&self) -> bool {
        (**self).is_refractive()
    }

    fn global_specular(&self, color: &Vec3) -> Vec3 {
        (**self).global_specular(color)
    }

    fn global_transmissive(&self, color: &Vec3) -> Vec3 {
        (**self).global_transmissive(color)
    }

    fn transmission(&self) -> Vec3 {
        (**self).transmission()
    }

    fn ior(&self) -> f64 {
        (**self).ior()
    }

    fn is_glossy(&self) -> bool{
        (**self).is_glossy()
    }

    fn glossiness(&self) -> f64 {
        (**self).glossiness()
    }
}

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64) -> ColorRGBA<f64>;

    fn clone_self(&self) -> Box<Texture>;
}

impl Texture for Box<Texture> {
    fn color(&self, u: f64, v: f64) -> ColorRGBA<f64> {
        (**self).color(u, v)
    }

    fn clone_self(&self) -> Box<Texture> {
        (**self).clone_self()
    }
}

impl Clone for Box<Texture> {
    fn clone(&self) -> Box<Texture> {
        self.clone_self()
    }
}
