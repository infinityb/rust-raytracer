#![allow(dead_code, unused_imports)]

use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use geometry::prim::Prim;
use geometry::prims::Triangle;
use raytracer::Ray;
use std::f64::{MAX_VALUE, MIN_VALUE};
use std::num::Float;
use std::default::Default;
use vec3::Vec3;
use material::Material;



type Vertex = (f64, f64, f64);

const ICOSA_A: f64 = 0.5;  // 1/2
const ICOSA_B: f64 = 0.3090169943749474; // 1 / (1 + sqrt(5))

static ISOCAHEDRON_FACES: [PureTriangle; 20] = [
    PureTriangle((6.123233995736766e-17, 0.0, 1.0), (0.8944271909999159, 0.0, 0.4472135954999579), (0.27639320225002106, 0.8506508083520399, 0.4472135954999579)),
    PureTriangle((6.123233995736766e-17, 0.0, 1.0), (0.27639320225002106, 0.8506508083520399, 0.4472135954999579), (-0.7236067977499788, 0.5257311121191337, 0.4472135954999579)),
    PureTriangle((6.123233995736766e-17, 0.0, 1.0), (-0.7236067977499788, 0.5257311121191337, 0.4472135954999579), (-0.723606797749979, -0.5257311121191335, 0.4472135954999579)),
    PureTriangle((6.123233995736766e-17, 0.0, 1.0), (-0.723606797749979, -0.5257311121191335, 0.4472135954999579), (0.27639320225002084, -0.85065080835204, 0.4472135954999579)),
    PureTriangle((6.123233995736766e-17, 0.0, 1.0), (0.27639320225002084, -0.85065080835204, 0.4472135954999579), (0.8944271909999159, 0.0, 0.4472135954999579)),
    PureTriangle((0.8944271909999159, 0.0, 0.4472135954999579), (0.7236067977499789, 0.5257311121191336, -0.4472135954999579), (0.27639320225002106, 0.8506508083520399, 0.4472135954999579)),
    PureTriangle((0.7236067977499789, 0.5257311121191336, -0.4472135954999579), (0.27639320225002106, 0.8506508083520399, 0.4472135954999579), (-0.27639320225002095, 0.85065080835204, -0.4472135954999579)),
    PureTriangle((0.27639320225002106, 0.8506508083520399, 0.4472135954999579), (-0.27639320225002095, 0.85065080835204, -0.4472135954999579), (-0.7236067977499788, 0.5257311121191337, 0.4472135954999579)),
    PureTriangle((-0.27639320225002095, 0.85065080835204, -0.4472135954999579), (-0.7236067977499788, 0.5257311121191337, 0.4472135954999579), (-0.8944271909999159, 1.0953573965284052e-16, -0.4472135954999579)),
    PureTriangle((-0.7236067977499788, 0.5257311121191337, 0.4472135954999579), (-0.8944271909999159, 1.0953573965284052e-16, -0.4472135954999579), (-0.723606797749979, -0.5257311121191335, 0.4472135954999579)),
    PureTriangle((-0.8944271909999159, 1.0953573965284052e-16, -0.4472135954999579), (-0.723606797749979, -0.5257311121191335, 0.4472135954999579), (-0.2763932022500211, -0.8506508083520399, -0.4472135954999579)),
    PureTriangle((-0.723606797749979, -0.5257311121191335, 0.4472135954999579), (-0.2763932022500211, -0.8506508083520399, -0.4472135954999579), (0.27639320225002084, -0.85065080835204, 0.4472135954999579)),
    PureTriangle((-0.2763932022500211, -0.8506508083520399, -0.4472135954999579), (0.27639320225002084, -0.85065080835204, 0.4472135954999579), (0.7236067977499788, -0.5257311121191338, -0.4472135954999579)),
    PureTriangle((0.27639320225002084, -0.85065080835204, 0.4472135954999579), (0.7236067977499788, -0.5257311121191338, -0.4472135954999579), (0.8944271909999159, 0.0, 0.4472135954999579)),
    PureTriangle((0.7236067977499788, -0.5257311121191338, -0.4472135954999579), (0.8944271909999159, 0.0, 0.4472135954999579), (0.7236067977499789, 0.5257311121191336, -0.4472135954999579)),
    PureTriangle((0.7236067977499789, 0.5257311121191336, -0.4472135954999579), (-0.27639320225002095, 0.85065080835204, -0.4472135954999579), (6.123233995736766e-17, 0.0, -1.0)),
    PureTriangle((-0.27639320225002095, 0.85065080835204, -0.4472135954999579), (-0.8944271909999159, 1.0953573965284052e-16, -0.4472135954999579), (6.123233995736766e-17, 0.0, -1.0)),
    PureTriangle((-0.8944271909999159, 1.0953573965284052e-16, -0.4472135954999579), (-0.2763932022500211, -0.8506508083520399, -0.4472135954999579), (6.123233995736766e-17, 0.0, -1.0)),
    PureTriangle((-0.2763932022500211, -0.8506508083520399, -0.4472135954999579), (0.7236067977499788, -0.5257311121191338, -0.4472135954999579), (6.123233995736766e-17, 0.0, -1.0)),
    PureTriangle((0.7236067977499788, -0.5257311121191338, -0.4472135954999579), (0.7236067977499789, 0.5257311121191336, -0.4472135954999579), (6.123233995736766e-17, 0.0, -1.0))
];

#[derive(Copy)]
struct PureTriangle(Vertex, Vertex, Vertex);

impl PureTriangle {
	pub fn zero() -> PureTriangle {
		PureTriangle((0.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 0.0, 0.0))
	}

	pub fn to_3vec3(&self) -> (Vec3, Vec3, Vec3) {
		(
			Vec3 { x: (self.0).0, y: (self.0).1, z: (self.0).2 },
			Vec3 { x: (self.1).0, y: (self.1).1, z: (self.1).2 },
			Vec3 { x: (self.2).0, y: (self.2).1, z: (self.2).2 },
		)
	}

	pub fn split(&self) -> [PureTriangle; 4] {
		let mut triangles = [PureTriangle::zero(); 4];
		//                      0
		//     /\              /\  
		//    /  \    -->   3 /__\   
		//   /    \          /\  /\  
		//  /______\      2 /__\/__\ 1
		//

		unimplemented!();
	}
}

impl ::std::ops::Mul<f64> for PureTriangle {
	type Output = PureTriangle;

	fn mul(self, scalar: f64) -> PureTriangle {
		PureTriangle(
			(
				scalar * (self.0).0,
				scalar * (self.0).1,
				scalar * (self.0).2
			),
			(
				scalar * (self.1).0,
				scalar * (self.1).1,
				scalar * (self.1).2
			),
			(
				scalar * (self.2).0,
				scalar * (self.2).1,
				scalar * (self.2).2
			)
		)
	}
}

pub struct IcosahedronBuilder {
	center: Vec3,
	radius: f64,
}

impl IcosahedronBuilder {
	pub fn new() -> IcosahedronBuilder {
		IcosahedronBuilder {
			center: Vec3::zero(),
			radius: 1.0,
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

	// pub fn to_mesh(&self) -> Mesh {
	// 	Mesh { triangles: self.build().
	// }

	pub fn build(&self) -> IcosahedronIter {
		IcosahedronIter {
			center: self.center,
			radius: self.radius,
			face_idx: 0,
		}
	}
}

pub struct IcosahedronIter {
	center: Vec3,
	radius: f64,
	face_idx: usize,
}

impl Iterator for IcosahedronIter {
	type Item = Triangle;

	fn next(&mut self) -> Option<Triangle> {
		if self.face_idx < ISOCAHEDRON_FACES.len() {
			let face = ISOCAHEDRON_FACES[self.face_idx];
			self.face_idx += 1;
			Some(face_to_triangle(face))
		} else {
			None
		}
	}
}

fn face_to_triangle(face: PureTriangle) -> Triangle {
	let face = face * 2.1111;
	let material = FlatMaterial { color: Vec3 { x: 1.0, y: 0.0, z: 0.0 } };

	let vecs = face.to_3vec3();
	Triangle::auto_normal(
		vecs.0, vecs.1, vecs.2,
		(0.0, 0.0), (0.0, 1.0), (1.0, 1.0),
		box material,
	)
}