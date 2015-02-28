use material::materials::FlatMaterial;
use geometry::prims::Triangle;
use raytracer::compositor::ColorRGBA;
use vec3::Vec3;

static ISOCAHEDRON_FACES: [PureTriangle; 20] = [
    PureTriangle(Vec3 { x: 0.0, y: 0.0, z: 1.0 }, Vec3 { x: 0.8944271909999159, y: 0.0, z: 0.4472135954999579 }, Vec3 { x: 0.27639320225002106, y: 0.8506508083520399, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.0, y: 0.0, z: 1.0 }, Vec3 { x: 0.27639320225002106, y: 0.8506508083520399, z: 0.4472135954999579 }, Vec3 { x: -0.7236067977499788, y: 0.5257311121191337, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.0, y: 0.0, z: 1.0 }, Vec3 { x: -0.7236067977499788, y: 0.5257311121191337, z: 0.4472135954999579 }, Vec3 { x: -0.723606797749979, y: -0.5257311121191335, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.0, y: 0.0, z: 1.0 }, Vec3 { x: -0.723606797749979, y: -0.5257311121191335, z: 0.4472135954999579 }, Vec3 { x: 0.27639320225002084, y: -0.85065080835204, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.0, y: 0.0, z: 1.0 }, Vec3 { x: 0.27639320225002084, y: -0.85065080835204, z: 0.4472135954999579 }, Vec3 { x: 0.8944271909999159, y: 0.0, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.8944271909999159, y: 0.0, z: 0.4472135954999579 }, Vec3 { x: 0.7236067977499789, y: 0.5257311121191336, z: -0.4472135954999579 }, Vec3 { x: 0.27639320225002106, y: 0.8506508083520399, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.7236067977499789, y: 0.5257311121191336, z: -0.4472135954999579 }, Vec3 { x: 0.27639320225002106, y: 0.8506508083520399, z: 0.4472135954999579 }, Vec3 { x: -0.27639320225002095, y: 0.85065080835204, z: -0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.27639320225002106, y: 0.8506508083520399, z: 0.4472135954999579 }, Vec3 { x: -0.27639320225002095, y: 0.85065080835204, z: -0.4472135954999579 }, Vec3 { x: -0.7236067977499788, y: 0.5257311121191337, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: -0.27639320225002095, y: 0.85065080835204, z: -0.4472135954999579 }, Vec3 { x: -0.7236067977499788, y: 0.5257311121191337, z: 0.4472135954999579 }, Vec3 { x: -0.8944271909999159, y: 0.0, z: -0.4472135954999579 }),
    PureTriangle(Vec3 { x: -0.7236067977499788, y: 0.5257311121191337, z: 0.4472135954999579 }, Vec3 { x: -0.8944271909999159, y: 0.0, z: -0.4472135954999579 }, Vec3 { x: -0.723606797749979, y: -0.5257311121191335, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: -0.8944271909999159, y: 0.0, z: -0.4472135954999579 }, Vec3 { x: -0.723606797749979, y: -0.5257311121191335, z: 0.4472135954999579 }, Vec3 { x: -0.2763932022500211, y: -0.8506508083520399, z: -0.4472135954999579 }),
    PureTriangle(Vec3 { x: -0.723606797749979, y: -0.5257311121191335, z: 0.4472135954999579 }, Vec3 { x: -0.2763932022500211, y: -0.8506508083520399, z: -0.4472135954999579 }, Vec3 { x: 0.27639320225002084, y: -0.85065080835204, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: -0.2763932022500211, y: -0.8506508083520399, z: -0.4472135954999579 }, Vec3 { x: 0.27639320225002084, y: -0.85065080835204, z: 0.4472135954999579 }, Vec3 { x: 0.7236067977499788, y: -0.5257311121191338, z: -0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.27639320225002084, y: -0.85065080835204, z: 0.4472135954999579 }, Vec3 { x: 0.7236067977499788, y: -0.5257311121191338, z: -0.4472135954999579 }, Vec3 { x: 0.8944271909999159, y: 0.0, z: 0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.7236067977499788, y: -0.5257311121191338, z: -0.4472135954999579 }, Vec3 { x: 0.8944271909999159, y: 0.0, z: 0.4472135954999579 }, Vec3 { x: 0.7236067977499789, y: 0.5257311121191336, z: -0.4472135954999579 }),
    PureTriangle(Vec3 { x: 0.7236067977499789, y: 0.5257311121191336, z: -0.4472135954999579 }, Vec3 { x: -0.27639320225002095, y: 0.85065080835204, z: -0.4472135954999579 }, Vec3 { x: 0.0, y: 0.0, z: -1.0 }),
    PureTriangle(Vec3 { x: -0.27639320225002095, y: 0.85065080835204, z: -0.4472135954999579 }, Vec3 { x: -0.8944271909999159, y: 0.0, z: -0.4472135954999579 }, Vec3 { x: 0.0, y: 0.0, z: -1.0 }),
    PureTriangle(Vec3 { x: -0.8944271909999159, y: 0.0, z: -0.4472135954999579 }, Vec3 { x: -0.2763932022500211, y: -0.8506508083520399, z: -0.4472135954999579 }, Vec3 { x: 0.0, y: 0.0, z: -1.0 }),
    PureTriangle(Vec3 { x: -0.2763932022500211, y: -0.8506508083520399, z: -0.4472135954999579 }, Vec3 { x: 0.7236067977499788, y: -0.5257311121191338, z: -0.4472135954999579 }, Vec3 { x: 0.0, y: 0.0, z: -1.0 }),
    PureTriangle(Vec3 { x: 0.7236067977499788, y: -0.5257311121191338, z: -0.4472135954999579 }, Vec3 { x: 0.7236067977499789, y: 0.5257311121191336, z: -0.4472135954999579 }, Vec3 { x: 0.0, y: 0.0, z: -1.0 })
];

#[derive(Copy, Clone)]
struct PureTriangle(Vec3, Vec3, Vec3);

impl PureTriangle {
    pub fn to_3vec3(&self, center: Vec3) -> (Vec3, Vec3, Vec3) {
        (self.0 + center, self.1 + center, self.2 + center)
    }

    pub fn split(&self) -> [PureTriangle; 4] {
        //      a                
        //      /\              /\  
        //   ab/  \ac  -->     /__\   
        //    /    \          /\  /\  
        //   /______\        /__\/__\ 
        // b    bc    c
        //
        let (a, b, c) = (self.0, self.1, self.2);
        let ab = (a + b) / 2.0;
        let ac = (a + c) / 2.0;
        let bc = (b + c) / 2.0;

        [
            PureTriangle(a, ab, ac),
            PureTriangle(ab, b, bc),
            PureTriangle(ac, bc, c),
            PureTriangle(ab, ac, bc)
        ]
    }

    pub fn set_radius(&self, radius: f64) -> PureTriangle {
        PureTriangle(
            self.0 * (radius / self.0.len()),
            self.1 * (radius / self.1.len()),
            self.2 * (radius / self.2.len()) 
        )
    }
}

pub trait Paint {
    fn get_color(&self, path: &[u8]) -> ColorRGBA<f64>;
}

struct DefaultPaint;

impl Paint for DefaultPaint {
    fn get_color(&self, path: &[u8]) -> ColorRGBA<f64> {
        use std::iter::AdditiveIterator;

        let div = 1.0 + path[1..].iter().map(|&x| if x == 3 { 1.0 } else { 0.0 }).sum();
        ColorRGBA::new_rgb(1.0 / div, 0.0, 0.0)
    }
}

pub mod painters {
    use std::num::Float;
    // use std::iter::AdditiveIterator;
    use raytracer::compositor::{ColorRGBA, Surface};
    use super::{Paint, PureTriangle, ISOCAHEDRON_FACES};

    fn path_to_triangle(path: &[u8]) -> PureTriangle {
        assert!(0 < path.len());
        let mut triangle = ISOCAHEDRON_FACES[path[0] as usize];
        for idx in path[1..].iter() {
            triangle = triangle.split()[*idx as usize];
        }
        triangle
    }

    pub struct TexturePainter {
        surface: Surface,
    }

    impl TexturePainter {
        pub fn new(surface: Surface) -> TexturePainter {
            TexturePainter { surface: surface }
        }
    }

    impl Paint for TexturePainter {
        fn get_color(&self, path: &[u8]) -> ColorRGBA<f64> {
            let triangle = path_to_triangle(path);
            let avg = (triangle.0 + triangle.1 + triangle.2) / 3.0;

            let sw = (self.surface.width - 1) as f64;
            let sh = (self.surface.height - 1) as f64;

            let lat = 90.0 - (avg.z / avg.len()).acos().to_degrees();
            let lon = avg.y.atan2(avg.x).to_degrees();

            let x = (sw * (lon + 180.0) / 360.0).round() as usize;
            let y = (sh * (lat + 90.0) / 180.0).round() as usize;

            self.surface[(x, y)].floatify()
        }
    }
}

pub struct IcosahedronBuilder {
    center: Vec3,
    radius: f64,
    max_depth: i32,
    paint: Box<Paint+'static>,
}

impl IcosahedronBuilder {
    pub fn new() -> IcosahedronBuilder {
        IcosahedronBuilder {
            center: Vec3::zero(),
            radius: 1.0,
            max_depth: 4,
            paint: box DefaultPaint,
        }
    }

    pub fn radius(mut self, radius: f64) -> IcosahedronBuilder {
        self.radius = radius;
        self
    }

    pub fn center(mut self, center: Vec3) -> IcosahedronBuilder {
        self.center = center;
        self
    }

    pub fn max_depth(mut self, max_depth: i32) -> IcosahedronBuilder {
        self.max_depth = max_depth;
        self
    }

    pub fn paint<P>(mut self, paint: P) -> IcosahedronBuilder where P: Paint+'static {
        self.paint = box paint;
        self
    }

    #[allow(unused)]
    pub fn build(self) -> IcosahedronIter {
        let initial_stack = ISOCAHEDRON_FACES.iter().cloned()
            .enumerate()
            .map(|(symbol, st)| (vec![symbol as u8], st)).collect::<Vec<_>>();

        IcosahedronIter::new(self.center, BaseIcosahedronIter {
            radius: self.radius,
            max_depth: self.max_depth,
            face_stack: initial_stack,
            paint: self.paint,
        })
    }

    pub fn build_sphere(self) -> IcosphereIter {
        let initial_stack = ISOCAHEDRON_FACES.iter().cloned()
            .enumerate()
            .map(|(symbol, st)| (vec![symbol as u8], st)).collect::<Vec<_>>();

        IcosphereIter::new(self.center, BaseIcosahedronIter {
            radius: self.radius,
            max_depth: self.max_depth,
            face_stack: initial_stack,
            paint: self.paint,
        })
    }
}

pub struct BaseIcosahedronIter {
    radius: f64,
    max_depth: i32,
    face_stack: Vec<(Vec<u8>, PureTriangle)>,
    paint: Box<Paint+'static>,
}

impl Iterator for BaseIcosahedronIter {
    type Item = (Vec<u8>, PureTriangle);

    fn next(&mut self) -> Option<(Vec<u8>, PureTriangle)> {
        loop {
            let (path, trixel) = match self.face_stack.pop() {
                Some(face_data) => face_data,
                None => return None,
            };
            if path.len() < self.max_depth as usize {
                for (symbol, &subtrixel) in trixel.split().iter().enumerate() {
                    let new_path = path.clone() + &[symbol as u8];
                    self.face_stack.push((new_path, subtrixel));
                }
            } else {
                self.face_stack.push((path, trixel));
                break;
            }
        }

        self.face_stack.pop()
    }
}

pub struct IcosahedronIter {
    center: Vec3,
    base: BaseIcosahedronIter,
}

#[allow(unused)]
impl IcosahedronIter {
    fn new(center: Vec3, base: BaseIcosahedronIter) -> IcosahedronIter {
        IcosahedronIter {
            center: center,
            base: base,
        }
    }

    fn make_triangle(&self, face: PureTriangle, path: &[u8], center: Vec3) -> Triangle {
        let material = FlatMaterial { color: self.base.paint.get_color(path).to_vec3() };
        let vecs = face.to_3vec3(center);
        Triangle::auto_normal(
            vecs.0, vecs.1, vecs.2,
            (0.0, 0.0), (0.0, 1.0), (1.0, 1.0),
            box material,
        )
    }
}

impl Iterator for IcosahedronIter {
    type Item = Triangle;

    fn next(&mut self) -> Option<Triangle> {
        self.base.next().and_then(|(path, triangle)| {
           Some(self.make_triangle(triangle, path.as_slice(), self.center))
        })
    }
}

pub struct IcosphereIter {
    center: Vec3,
    base: BaseIcosahedronIter,
}

impl IcosphereIter {
    fn new(center: Vec3, base: BaseIcosahedronIter) -> IcosphereIter {
        IcosphereIter {
            center: center,
            base: base,
        }
    }

    fn make_triangle(&self, face: PureTriangle, path: &[u8], radius: f64, center: Vec3) -> Triangle {
        let material = FlatMaterial { color: self.base.paint.get_color(path).to_vec3() };
        let vecs = face.set_radius(radius).to_3vec3(center);

        Triangle::auto_normal(
            vecs.0, vecs.1, vecs.2,
            (0.0, 0.0), (0.0, 1.0), (1.0, 1.0),
            box material,
        )
    }
}

impl Iterator for IcosphereIter {
    type Item = Triangle;

    fn next(&mut self) -> Option<Triangle> {
        self.base.next().and_then(|(path, triangle)| {
            Some(self.make_triangle(triangle, path.as_slice(), self.base.radius, self.center))
        })
    }
}


