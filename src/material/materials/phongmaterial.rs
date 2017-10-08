use material::{Material, Texture};
use raytracer::compositor::ColorRGBA;
use vec3::Vec3;

#[allow(dead_code)]
#[derive(Clone)]
pub struct PhongMaterial {
    pub k_a: f64,           // Ambient coefficient
    pub k_d: f64,           // Diffuse coefficient
    pub k_s: f64,           // Local specular coefficient
    pub k_sg: f64,          // Global specular coefficient (mirror reflection)
    pub k_tg: f64,          // Global transmissive coefficient (refraction)
    pub ambient: Vec3,      // Ambient color
    pub diffuse: Vec3,      // Diffuse color
    pub transmission: Vec3, // Transmissive color
    pub specular: Vec3,     // Specular color
    pub shininess: f64,     // Size of Phong specular highlight
    pub glossiness: f64,    // How glossy reflections are. 0 for non-glossy surfaces.
    pub ior: f64,           // Index of refraction
    pub diffuse_texture: Option<Box<Texture>>
}

impl Material for PhongMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> Vec3 {
        let h = (l + i).unit();

        // Blinn-Phong approximation
        let color    = self.diffuse_texture.as_ref()
            .map(|x| x.color(u, v))
            .unwrap_or_else(ColorRGBA::white);
        let ambient  = self.ambient.scale(self.k_a);
        let diffuse  = self.diffuse.scale(self.k_d).scale(n.dot(&l)) * color.to_vec3();
        let specular = self.specular.scale(self.k_s).scale(n.dot(&h).powf(self.shininess));

        ambient + diffuse + specular
    }

    fn is_reflective(&self) -> bool {
        self.k_sg > 0.0
    }

    fn is_refractive(&self) -> bool {
        self.k_tg > 0.0
    }

    fn global_specular(&self, color: &Vec3) -> Vec3 {
        color.scale(self.k_sg)
    }

    fn global_transmissive(&self, color: &Vec3) -> Vec3 {
        color.scale(self.k_tg)
    }

    fn transmission(&self) -> Vec3 {
        self.transmission
    }

    fn ior(&self) -> f64 {
        self.ior
    }

    fn is_glossy(&self) -> bool {
        self.glossiness > ::std::f64::EPSILON
    }

    fn glossiness(&self) -> f64 {
        self.glossiness
    }
}

impl Default for PhongMaterial {
    fn default() -> PhongMaterial {
        PhongMaterial {
            k_a: 0.0,
            k_d: 1.0,
            k_s: 1.0,
            k_sg: 0.0,
            k_tg: 0.0,
            shininess: 10.0,
            glossiness: 0.0,
            ior: 1.0,
            ambient: Vec3::one(),
            diffuse: Vec3 { x: 0.5, y: 0.5, z: 0.5 },
            specular: Vec3::one(),
            transmission: Vec3::zero(),
            diffuse_texture: None
        }
    }
}
