use material::Texture;
use raytracer::compositor::ColorRGBA;


/// Maps the supplied (u, v) coordinate to the (red, green) color channels.
#[derive(Clone)]
pub struct UVTexture;

impl Texture for UVTexture {
    fn color(&self, u: f64, v: f64) -> ColorRGBA<f64> {
    	ColorRGBA::new_rgb(u % 1.0, v % 1.0, 0.0)
    }

    fn clone_self(&self) -> Box<Texture+Send+Sync> {
        box UVTexture as Box<Texture+Send+Sync>
    }
}
