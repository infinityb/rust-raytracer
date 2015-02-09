use material::materials::FlatMaterial;
use geometry::prims::Triangle;
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

pub struct IcosahedronBuilder {
    center: Vec3,
    radius: f64,
    max_depth: i32,
}

impl IcosahedronBuilder {
    pub fn new() -> IcosahedronBuilder {
        IcosahedronBuilder {
            center: Vec3::zero(),
            radius: 1.0,
            max_depth: 4,
        }
    }

    pub fn radius(&mut self, radius: f64) -> &mut IcosahedronBuilder {
        self.radius = radius;
        self
    }

    pub fn center(&mut self, center: Vec3) -> &mut IcosahedronBuilder {
        self.center = center;
        self
    }

    pub fn max_depth(&mut self, max_depth: i32) -> &mut IcosahedronBuilder {
        self.max_depth = max_depth;
        self
    }

    #[allow(unused)]
    pub fn build(&self) -> IcosahedronIter {
        let initial_stack = ISOCAHEDRON_FACES.iter().cloned()
            .enumerate()
            .map(|(symbol, st)| (0, symbol as i32, st)).collect::<Vec<_>>();

        IcosahedronIter::new(self.center, BaseIcosahedronIter {
            radius: self.radius,
            max_depth: self.max_depth,
            face_stack: initial_stack,
        })
    }

    pub fn build_sphere(&self) -> IcosphereIter {
        let initial_stack = ISOCAHEDRON_FACES.iter().cloned()
            .enumerate()
            .map(|(symbol, st)| (0, symbol as i32, st)).collect::<Vec<_>>();

        IcosphereIter::new(self.center, BaseIcosahedronIter {
            radius: self.radius,
            max_depth: self.max_depth,
            face_stack: initial_stack,
        })
    }
}

pub struct BaseIcosahedronIter {
    radius: f64,
    max_depth: i32,
    face_stack: Vec<(i32, i32, PureTriangle)>,
}

impl Iterator for BaseIcosahedronIter {
    type Item = (i32, i32, PureTriangle);

    fn next(&mut self) -> Option<(i32, i32, PureTriangle)> {
        loop {
            let (depth, symbol, trixel) = match self.face_stack.pop() {
                Some(triple) => triple,
                None => return None,
            };
            if depth < self.max_depth {
                for (symbol, &subtrixel) in trixel.split().iter().enumerate() {
                    self.face_stack.push((depth + 1, symbol as i32, subtrixel));
                }
            } else {
                self.face_stack.push((depth, symbol, trixel));
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

    fn make_triangle(face: PureTriangle, symbol: i32, center: Vec3) -> Triangle {
        let material = FlatMaterial { color: match symbol % 4 {
            0 => Vec3 { x: 0.6078, y: 0.3490, z: 0.7137 },
            1 => Vec3 { x: 0.5294, y: 0.8275, z: 0.4863 },
            2 => Vec3 { x: 0.1725, y: 0.2431, z: 0.3137 },
            3 => Vec3 { x: 0.8118, y: 0.0000, z: 0.0588 },
            _ => unreachable!(),
        }};
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
        self.base.next().and_then(|(_, symbol, triangle)| {
           Some(IcosahedronIter::make_triangle(triangle, symbol, self.center))
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

    fn make_triangle(face: PureTriangle, symbol: i32, radius: f64, center: Vec3) -> Triangle {
        let material = FlatMaterial { color: match symbol {
            0 => Vec3 { x: 0.6078, y: 0.3490, z: 0.7137 },
            1 => Vec3 { x: 0.5294, y: 0.8275, z: 0.4863 },
            2 => Vec3 { x: 0.1725, y: 0.2431, z: 0.3137 },
            3 => Vec3 { x: 0.8118, y: 0.0000, z: 0.0588 },
            _ => unreachable!(),
        }};
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
        self.base.next().and_then(|(_, symbol, triangle)| {
            Some(IcosphereIter::make_triangle(triangle, symbol, self.base.radius, self.center))
        })
    }
}

