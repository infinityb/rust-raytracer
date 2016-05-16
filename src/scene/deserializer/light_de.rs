use serde;
use serde::de::{
    Error as SerdeError,
    Visitor,
    SeqVisitor,
    Deserialize,
    Deserializer,
};

use super::{SceneFactory, Color};
use ::vec3::Vec3;
use ::light::Light;
use ::light::lights::{
    SphereLight,
    PointLight,
};

const SPHERE_LIGHT_NAME: &'static str = "SphereLight";
const POINT_LIGHT_NAME: &'static str = "PointLight";

enum LightKind {
    Sphere,
    Point,
}

impl Deserialize for LightKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<LightKind, D::Error>
        where D: Deserializer,
    {
        struct LightKindVisitor;

        impl Visitor for LightKindVisitor {
            type Value = LightKind;

            fn visit_str<E>(&mut self, value: &str) -> Result<LightKind, E>
                where E: serde::de::Error
            {
                match value {
                    SPHERE_LIGHT_NAME => Ok(LightKind::Sphere),
                    POINT_LIGHT_NAME => Ok(LightKind::Point),
                    _ => Err(serde::de::Error::custom("unexpected value")),
                }
            }
        }
        
        deserializer.deserialize(LightKindVisitor)
    }
}

impl LightKind {
    fn get_light_factory<V>(&self, visitor: &mut V) -> Result<Box<LightFactory>, V::Error>
        where V: SeqVisitor
    {
        return match *self {
            LightKind::Sphere => {
                visitor.visit().and_then(ensure_present_and_box::<SphereLightFactory, _>)
            },
            LightKind::Point => {
                visitor.visit().and_then(ensure_present_and_box::<PointLightFactory, _>)
            },
        };

        #[inline]
        fn ensure_present_and_box<T, E>(mf: Option<T>) -> Result<Box<LightFactory>, E>
            where
                T: LightFactory + 'static,
                E: SerdeError
        {
            match mf {
                Some(fac) => Ok(Box::new(fac) as Box<LightFactory>),
                None => Err(SerdeError::invalid_length(1)),
            }
        }
    }
}

/// --

trait LightFactory: ::std::fmt::Debug {
    fn create(&self, context: SceneFactory) -> Box<Light+Send+Sync>;
}

/// --

#[derive(Deserialize, Debug)]
struct SphereLightFactory {
    position: Vec3,
    color: Color,
    radius: f64,
}

impl LightFactory for SphereLightFactory {
    fn create(&self, _context: SceneFactory) -> Box<Light+Send+Sync> {
        Box::new(SphereLight {
            position: self.position,
            color: self.color.into(),
            radius: self.radius,
        })
    }
}

/// --

#[derive(Deserialize, Debug)]
struct PointLightFactory {
    position: Vec3,
    color: Color,
}

impl LightFactory for PointLightFactory {
    fn create(&self, _context: SceneFactory) -> Box<Light+Send+Sync> {
        Box::new(PointLight {
            position: self.position,
            color: self.color.into(),
        })
    }
}

/// --

#[derive(Debug)]
pub struct LightFactoryWrapper(Box<LightFactory>);

impl Deserialize for LightFactoryWrapper {
    fn deserialize<D>(deserializer: &mut D) -> Result<LightFactoryWrapper, D::Error>
        where D: Deserializer,
    {
        struct LightVisitor;

        impl Visitor for LightVisitor {
            type Value = LightFactoryWrapper;

            #[inline]
            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<LightFactoryWrapper, V::Error>
                where V: SeqVisitor
            {
                let lig_kind: LightKind;
                if let Some(tmp_mat_kind) = try!(visitor.visit()) {
                    lig_kind = tmp_mat_kind;
                } else {
                    return Err(SerdeError::invalid_length(0));
                }
                let mat = try!(lig_kind.get_light_factory(&mut visitor));
                try!(visitor.end());
                Ok(LightFactoryWrapper(mat))
            }
        }

        deserializer.deserialize(LightVisitor)
    }
}
