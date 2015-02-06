use raytracer::compositor::{ColorRGBA, SurfaceFactory};


pub struct SurfaceIterator {
    x_delta: usize,
    x_off: usize,
    y_delta: usize,
    y_off: usize,
    parent_width: usize,
    parent_height: usize,
    background: ColorRGBA<u8>,
}


impl SurfaceIterator {
    fn incr_tile(&mut self) {
        if self.x_off + self.x_delta < self.parent_width {
            self.x_off += self.x_delta;
        } else {
            self.x_off = 0;
            self.y_off += self.y_delta;
        }
    }

    fn current_tile(&self) -> Option<SurfaceFactory> {
        if self.x_off < self.parent_width && self.y_off < self.parent_height {
            Some(SurfaceFactory::new(
                self.x_delta,
                self.y_delta,
                self.x_off,
                self.y_off,
                self.background
            ))
        } else {
            None
        }
    }
}

impl Iterator for SurfaceIterator {
    type Item = SurfaceFactory;

    fn next(&mut self) -> Option<SurfaceFactory> {
        let tile = self.current_tile();
        self.incr_tile();
        tile
    }
}