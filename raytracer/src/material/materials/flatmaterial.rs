use ::compositor::ColorRGBA;
use ::material::Material;
use ::util::Vec3;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FlatMaterial {
    pub color: ColorRGBA<f64>
}

impl Material for FlatMaterial {
    fn sample(&self, _n: Vec3, _i: Vec3, _l: Vec3, _u: f64, _v: f64) -> ColorRGBA<f64> {
        self.color
    }

    fn is_reflective(&self) -> bool {
        false
    }

    fn is_refractive(&self) -> bool {
        false
    }

    fn global_specular(&self, _color: &ColorRGBA<f64>) -> ColorRGBA<f64> {
        unimplemented!();
    }

    fn global_transmissive(&self, _color: &ColorRGBA<f64>) -> ColorRGBA<f64> {
        unimplemented!();
    }

    fn transmission(&self) -> ColorRGBA<f64> {
        unimplemented!();
    }

    fn ior(&self) -> f64 {
        1.0
    }

    fn is_glossy(&self) -> bool {
        false
    }

    fn glossiness(&self) -> f64 {
        0.0
    }

    fn clone_box(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}

impl Default for FlatMaterial {
    fn default() -> FlatMaterial {
        FlatMaterial {
            color: ColorRGBA::new_rgb(0.5, 0.5, 0.5),
        }
    }
}
