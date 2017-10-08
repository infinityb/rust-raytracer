use std::f64::consts::PI;
use material::{Material, Texture};
use raytracer::compositor::ColorRGBA;
use vec3::Vec3;

#[allow(dead_code)]
#[derive(Clone)]
pub struct CookTorranceMaterial {
    pub k_a: f64,            // Ambient coefficient
    pub k_d: f64,            // Diffuse coefficient
    pub k_s: f64,            // Local specular coefficient
    pub k_sg: f64,           // Global specular coefficient (mirror reflection)
    pub k_tg: f64,           // Global transmissive coefficient (refraction)
    pub ambient: Vec3,       // Ambient color
    pub diffuse: Vec3,       // Diffuse color
    pub transmission: Vec3,  // Transmissive color
    pub specular: Vec3,      // Specular color
    pub roughness: f64,      // Smaller = shininer => smaller highlight spot on surface
    pub glossiness: f64,     // How glossy reflections are. 0 for non-glossy surfaces.
    pub gauss_constant: f64, // Controls curve of distribution of microfacets
    pub ior: f64,            // Index of refraction, also used for specular highlights
    pub diffuse_texture: Option<Box<Texture>>
}

impl Material for CookTorranceMaterial {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> Vec3 {
        let color    = self.diffuse_texture.as_ref()
            .map(|x| x.color(u, v))
            .unwrap_or_else(ColorRGBA::white);
        let ambient  = self.ambient.scale(self.k_a);
        let diffuse  = self.diffuse.scale(self.k_d).scale(n.dot(&l)) * color.to_vec3();

        // Specular calculations
        let h = (l + i).unit();
        let n_dot_h = n.dot(&h);
        let n_dot_l = n.dot(&l);
        let v_dot_h = i.dot(&h);
        let n_dot_v = n.dot(&i);

        // Fresnel term (Schlick's approximation)
        let n1 = 1.0;
        let n2 = self.ior;
        let f0 = ((n1 - n2) / (n1 + n2)).powf(2.0);
        let f = (1.0 - v_dot_h).powf(5.0) * (1.0 - f0) + f0;

        // Microfacet distribution
        let alpha = n_dot_h.acos();
        let d = self.gauss_constant * (-alpha / self.roughness.sqrt()).exp();

        // Geometric attenuation factor
        let g1 = (2.0 * n_dot_h * n_dot_v) / v_dot_h;
        let g2 = (2.0 * n_dot_h * n_dot_l) / v_dot_h;
        let g = g1.min(g2);

        let brdf = f * d * g / (n_dot_v * n_dot_l * PI);

        self.specular.scale(self.k_s * brdf) + diffuse + ambient
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

impl Default for CookTorranceMaterial {
    fn default() -> CookTorranceMaterial {
        CookTorranceMaterial {
            k_a: 0.0,
            k_d: 1.0,
            k_s: 1.0,
            k_sg: 0.0,
            k_tg: 0.0,
            gauss_constant: 1.0,
            roughness: 0.15,
            glossiness: 0.0,
            ior: 1.5,
            ambient: Vec3::one(),
            diffuse: Vec3 { x: 0.5, y: 0.5, z: 0.5 },
            specular: Vec3::one(),
            transmission: Vec3::zero(),
            diffuse_texture: None
        }
    }
}
