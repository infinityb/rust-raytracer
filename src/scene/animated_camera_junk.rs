
impl AnimatedCamera {
    pub fn animate_range<'a>(&'a self, t_start: f64, t_end: f64, t_delta: f64, interpolator: &'a Interpolator) -> CameraAnimIter<'a> {
        CameraAnimIter {
            acam: self,
            interpolator: interpolator,
            t_current: t_start,
            t_delta: t_delta,
            t_end: t_end,
        }
    }

    pub fn animate<'a>(&'a self, t_delta: f64, interpolator: &'a Interpolator) -> CameraAnimIter<'a> {
        let mut t_start: f64 = 0.0;
        let mut t_end: f64 = 0.0;

        if self.keyframes.len() > 0 {
            t_start = self.keyframes[0].time;
            t_end = self.keyframes.iter().last().unwrap().time;
        }

        self.animate_range(t_start, t_end, t_delta, interpolator)
    }
}

struct CameraAnimIter<'a> {
    acam: &'a AnimatedCamera,
    interpolator: &'a Interpolator,
    t_current: f64,
    t_delta: f64,
    t_end: f64,
}

impl<'a> Iterator for CameraAnimIter<'a> {
    type Item = Camera;

    pub fn next(&mut self) -> Option<Camera> {
        // If we don't have keyframes, just emit the original camera.
        if self.acam.keyframes.len() == 0 {
            self.t_current = self.t_end + self.t_delta;
            return Some(self.acam.camera);
        }

        if self.t_end <= self.t_current {
            return None;
        }

        let cam = self.interpolator.interpolate(self.acam, self.t_current);
        self.t_current += self.t_delta;
        Some(cam)
    }
}
