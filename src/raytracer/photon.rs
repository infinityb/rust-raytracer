use vec3::Vec3;

#[derive(Clone)]
pub struct Photon {
    pub position: Vec3,
    pub incoming_dir: Vec3,
    pub power: Vec3
}
