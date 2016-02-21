use super::Camera;
use ::raytracer::animator::CameraKeyframe;

pub struct AnimatedCamera {
    pub camera: Camera,
    pub keyframes: Vec<CameraKeyframe>
}

impl AnimatedCamera {
    pub fn new(camera: Camera, keyframes: Vec<CameraKeyframe>) -> AnimatedCamera {
        AnimatedCamera {
            camera: camera,
            keyframes: keyframes,
        }
    }
}
