use raytracer::Ray;
use vec3::Vec3;

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub fov_deg: f64,
    pub image_width: u32,
    pub image_height: u32,

    pub eye: Vec3,
    pub right: Vec3,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_width: f64,
    pub pixel_height: f64,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, fov_deg: f64,
               image_width: u32, image_height: u32)
               -> Camera {

        let mut camera = Camera {
            position: position,
            look_at: look_at,
            up: up,
            fov_deg: fov_deg,
            image_width: image_width,
            image_height: image_height,
            eye: Vec3::zero(),
            right: Vec3::zero(),
            half_width: 0.0,
            half_height: 0.0,
            pixel_width: 0.0,
            pixel_height: 0.0,
        };

        camera.update_eye_vector();
        camera.update_internal_sizes();

        camera
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(
            self.position,
            (self.eye + self.right.scale(x * self.pixel_width - self.half_width) +
            self.up.scale(y * self.pixel_height - self.half_height)).unit()
        )
    }

    fn update_eye_vector(&mut self) {
        self.eye = (self.look_at - self.position).unit();
        self.right = self.eye.cross(&self.up);
    }

    fn update_internal_sizes(&mut self) {
        let fov_rad = self.fov_deg.to_radians();
        let ratio = self.image_height as f64 / self.image_width as f64;

        self.half_width  = fov_rad.tan();
        self.half_height = self.half_width * ratio;

        let camera_width  = self.half_width  * 2.0;
        let camera_height = self.half_height * 2.0;

        self.pixel_width  = camera_width  / (self.image_width  - 1) as f64;
        self.pixel_height = camera_height / (self.image_height - 1) as f64;
    }
}
