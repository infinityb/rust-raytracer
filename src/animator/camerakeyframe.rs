use vec3::Vec3;
use raytracer::animator::Easing;

#[derive(Clone)]
pub struct CameraKeyframe {
    pub time: f64,
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub easing: Easing
}

enum _Void;

impl _Void {
    #[allow(dead_code)]
    pub fn new_with_keyframes(position: Vec3, look_at: Vec3, up: Vec3, fov_deg: f64,
                              image_width: u32, image_height: u32, keyframes: Vec<CameraKeyframe>)
                              -> Camera {

        let mut camera = Camera::new(position, look_at, up, fov_deg, image_width, image_height);
        camera.insert_keyframes(keyframes);
        camera
    }

    /// Add additional keyframes to the camera. The current state of the camera
    /// is treated as t=0, and a new keyframe at t=0 is created and added.
    #[allow(dead_code)]
    pub fn insert_keyframes(&mut self, additional_keyframes: Vec<CameraKeyframe>) {
        let t0_keyframe = CameraKeyframe {
            time: 0.0,
            position: self.position,
            look_at: self.look_at,
            up: self.up,
            easing: Easing::linear()
        };

        let mut keyframes = vec![t0_keyframe];
        keyframes.extend(additional_keyframes);

        self.keyframes = Some(keyframes);
    }
}