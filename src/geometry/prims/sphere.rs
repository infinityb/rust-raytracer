use geometry::bbox::BBox;
use geometry::prim::Prim;
use material::Material;
use raytracer::{Ray, Intersection};
use vec3::Vec3;

#[allow(dead_code)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material+Send+Share>
}


impl Prim for Sphere {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection<'a>> {
        let i = ray.origin - self.center;
        let a = 1.0;
        let b = 2.0 * ray.direction.dot(&i);
        let c = i.dot(&i) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant <= 0.0 {
            None
        } else {
            // Up to two intersections
            let disc_sqrt = discriminant.sqrt();
            let t1 = (-b + disc_sqrt) / 2.0 * a;
            let t2 = (-b - disc_sqrt) / 2.0 * a;

            if t1 >= t_min && t1 <= t_max ||
               t2 >= t_min && t2 <= t_max {
                // Valid intersection(s): get nearer intersection
                let t = if t1.abs() < t2.abs() { t1 } else { t2 };
                let intersection_point = ray.origin + ray.direction.scale(t);
                let n = (intersection_point - self.center).unit();

                let u = 0.5 + n.z.atan2(n.x) / ::std::f64::consts::PI_2;
                let v = 0.5 - n.y.asin() / ::std::f64::consts::PI;

                Some(Intersection {
                    n: n,
                    t: t,
                    u: u,
                    v: v,
                    position: intersection_point,
                    material: &self.material
                })
            } else {
                None
            }
        }
    }

    fn bounding(&self) -> Option<BBox> {
        Some(BBox {
            min: Vec3 {
                x: self.center.x - self.radius,
                y: self.center.y - self.radius,
                z: self.center.z - self.radius
            },
            max: Vec3 {
                x: self.center.x + self.radius,
                y: self.center.y + self.radius,
                z: self.center.z + self.radius
            }
        })
    }
}