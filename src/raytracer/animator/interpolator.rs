use vec3::Vec3;
use scene::{Camera, AnimatedCamera};
use super::CameraKeyframe;

pub trait Interpolator {
    fn interpolate(&self, acam: &AnimatedCamera, time: f64) -> Camera;
}

pub struct LerpInterpolator;

impl Interpolator for LerpInterpolator {
    fn interpolate(&self, acam: &AnimatedCamera, time: f64) -> Camera {
        let (first, second, alpha) = LerpInterpolator::get_neighbour_keyframes(acam, time);

        let lerped_position = Vec3::lerp(&first.position, &second.position, alpha);
        let lerped_look_at  = Vec3::lerp(&first.look_at, &second.look_at, alpha);
        let lerped_up       = Vec3::lerp(&first.up, &second.up, alpha);

        Camera::new(
            lerped_position,
            lerped_look_at,
            lerped_up,
            acam.camera.fov_deg,
            acam.camera.image_width,
            acam.camera.image_height,
        )
    }
}

impl LerpInterpolator {
    fn get_neighbour_keyframes(acam: &AnimatedCamera, time: f64) -> (CameraKeyframe, CameraKeyframe, f64) {
        if acam.keyframes.len() <= 1 {
            panic!("Not enough keyframes to interpolate: got: {} expected: >= 2", acam.keyframes.len());
        }

        // Get the two keyframes inbetween current time
        let mut first = &acam.keyframes[0];
        let mut second = &acam.keyframes[1];

        for keyframe in acam.keyframes.iter() {
            if keyframe.time <= time && time - keyframe.time >= first.time - time {
                first = keyframe;
            }

            if keyframe.time > time &&
               (keyframe.time - time < second.time - time || second.time < time) {
                second = keyframe;
            }
        }

        let keyframe_length = second.time - first.time;

        let alpha = if keyframe_length == 0.0 {
            0.0
        } else {
            second.easing.t((time - first.time) / keyframe_length)
        };

        (first.clone(), second.clone(), alpha)
    }
}

#[test]
fn test_lerp_camera_position() {
    // Camera rotates 180 degrees
    let camera = Camera::new(
        Vec3 { x: -1.0, y: -1.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        45.0,
        10,
        10
    );
    let acamera = AnimatedCamera::new(camera, vec![
        CameraKeyframe {
            time: 5.0,
            position: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            look_at: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
        },
        CameraKeyframe {
            time: 10.0,
            position: Vec3 { x: 10.0, y: 0.0, z: 0.0 },
            look_at: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
            up: Vec3 { x: 0.0, y: 1.0, z: 0.0 }
        },
    ]);


    let expected_position_0 = Vec3 { x: -1.0, y: -1.0, z: -1.0 };
    assert_eq!(LerpInterpolator::interpolate(&acamera, 0.0).position, expected_position_0);

    let expected_position_5 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    assert_eq!(LerpInterpolator::interpolate(&acamera, 5.0).position, expected_position_5);

    let expected_position_7_5 = Vec3 { x: 5.0, y: 0.0, z: 0.0 };
    assert_eq!(LerpInterpolator::interpolate(&acamera, 7.5).position, expected_position_7_5);

    let expected_position_10 = Vec3 { x: 10.0, y: 0.0, z: 0.0 };
    assert_eq!(LerpInterpolator::interpolate(&acamera, 10.0).position, expected_position_10);
}
