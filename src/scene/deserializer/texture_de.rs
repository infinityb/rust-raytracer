use serde;
use serde::de::{
    Error as SerdeError,
    Visitor,
    SeqVisitor,
    Deserialize,
    Deserializer,
};

use super::{SceneFactory, Color};
use ::material::Texture;
use ::material::textures::{
    CheckerTexture,
    // CubeMap,
    // ImageTexture,
    // UVTexture,
};

const CHECKER_TEXTURE_NAME: &'static str = "CheckerTexture";
const CUBE_MAP_TEXTURE_NAME: &'static str = "CubeMapTexture";
const IMAGE_TEXTURE_NAME: &'static str = "ImageTexture";
const UV_TEXTURE_NAME: &'static str = "UVTexture";


enum TextureKind {
    Checker,
    CubeMap,
    Image,
    Uv,
}

impl Deserialize for TextureKind {
    fn deserialize<D>(deserializer: &mut D) -> Result<TextureKind, D::Error>
        where D: Deserializer,
    {
        struct TextureKindVisitor;

        impl Visitor for TextureKindVisitor {
            type Value = TextureKind;

            fn visit_str<E>(&mut self, value: &str) -> Result<TextureKind, E>
                where E: serde::de::Error
            {
                match value {
                    CHECKER_TEXTURE_NAME => Ok(TextureKind::Checker),
                    CUBE_MAP_TEXTURE_NAME => Ok(TextureKind::CubeMap),
                    IMAGE_TEXTURE_NAME => Ok(TextureKind::Image),
                    UV_TEXTURE_NAME => Ok(TextureKind::Uv),
                    _ => Err(serde::de::Error::custom("unexpected value")),
                }
            }
        }
        
        deserializer.deserialize(TextureKindVisitor)
    }
}

impl TextureKind {
    fn get_texture_factory<V>(&self, visitor: &mut V) -> Result<Box<TextureFactory>, V::Error>
        where V: SeqVisitor
    {
        return match *self {
            TextureKind::Checker => {
                visitor.visit().and_then(ensure_present_and_box::<CheckerTextureFactory, _>)
            },
            TextureKind::CubeMap => {
                visitor.visit().and_then(ensure_present_and_box::<CubeMapTextureFactory, _>)
            },
            TextureKind::Image => {
                visitor.visit().and_then(ensure_present_and_box::<ImageTextureFactory, _>)
            },
            TextureKind::Uv => {
                visitor.visit().and_then(ensure_present_and_box::<UvTextureFactory, _>)
            },
        };

        #[inline]
        fn ensure_present_and_box<T, E>(mf: Option<T>) -> Result<Box<TextureFactory>, E>
            where
                T: TextureFactory + 'static,
                E: SerdeError
        {
            match mf {
                Some(fac) => Ok(Box::new(fac) as Box<TextureFactory>),
                None => Err(SerdeError::invalid_length(1)),
            }
        }
    }
}

trait TextureFactory: ::std::fmt::Debug {
    fn create(&self, context: SceneFactory) -> Box<Texture+Send+Sync>;
}

/// -- 

#[derive(Deserialize, Debug)]
struct CheckerTextureFactory {
    color1: Color,
    color2: Color,
    scale: f64 // Controls how large the squares are.
}

impl TextureFactory for CheckerTextureFactory {
    fn create(&self, context: SceneFactory) -> Box<Texture+Send+Sync> {
        Box::new(CheckerTexture {
            color1: self.color1.into(),
            color2: self.color2.into(),
            scale: self.scale,
        })
    }
}

/// -- 

#[derive(Deserialize, Debug)]
struct CubeMapTextureFactory;

impl TextureFactory for CubeMapTextureFactory {
    fn create(&self, context: SceneFactory) -> Box<Texture+Send+Sync> {
        unimplemented!();
    }
}

/// -- 

#[derive(Deserialize, Debug)]
struct ImageTextureFactory;

impl TextureFactory for ImageTextureFactory {
    fn create(&self, context: SceneFactory) -> Box<Texture+Send+Sync> {
        unimplemented!();
    }
}

/// --

#[derive(Deserialize, Debug)]
struct UvTextureFactory;

impl TextureFactory for UvTextureFactory {
    fn create(&self, context: SceneFactory) -> Box<Texture+Send+Sync> {
        unimplemented!();
    }
}

/// --

#[derive(Debug)]
pub struct TextureFactoryWrapper(Box<TextureFactory>);

impl Deserialize for TextureFactoryWrapper {
    fn deserialize<D>(deserializer: &mut D) -> Result<TextureFactoryWrapper, D::Error>
        where D: Deserializer,
    {
        struct TextureVisitor;

        impl Visitor for TextureVisitor {
            type Value = TextureFactoryWrapper;

            #[inline]
            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<TextureFactoryWrapper, V::Error>
                where V: SeqVisitor
            {
                let mat_kind: TextureKind;
                if let Some(tmp_mat_kind) = try!(visitor.visit()) {
                    mat_kind = tmp_mat_kind;
                } else {
                    return Err(SerdeError::invalid_length(0));
                }
                let mat = try!(mat_kind.get_texture_factory(&mut visitor));
                try!(visitor.end());
                Ok(TextureFactoryWrapper(mat))
            }
        }

        deserializer.deserialize(TextureVisitor)
    }
}
