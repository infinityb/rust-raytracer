use std::cmp::{Eq, PartialOrd, Ord, Ordering};
use raytracer::Photon;

// For use in PhotonCache's querying of nearest N-neighbours
// #[derive(Eq, PartialOrd, PartialEq)]
#[derive(Clone)]
pub struct PhotonQuery {
    pub photon: Photon,
    pub distance_to_point: f64
}

// Need total ordering for binary heap
impl Ord for PhotonQuery {
    fn cmp(&self, other: &PhotonQuery) -> Ordering {
        Ord::cmp(self.distance_to_point, other.distance_to_point)
    }
}

impl Eq for PhotonQuery {
}

impl PartialOrd for PhotonQuery {
    fn partial_cmp(&self, other: &PhotonQuery) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.distance_to_point, other.distance_to_point)
    }
}

impl PartialEq for PhotonQuery {
    fn eq(&self, other: &PhotonQuery) -> bool {
        self == other
    }
}
