use num::{Float, ToPrimitive};
use std::ops::{Add, Mul, Sub};

pub trait Channel: ToPrimitive {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn add(a: Self, b: Self) -> Self;
    fn sub(a: Self, b: Self) -> Self;
}

impl Channel for u8 {
    #[inline]
    fn min_value() -> u8 { u8::min_value() }

    #[inline]
    fn max_value() -> u8 { u8::max_value() }

    #[inline]
    fn add(a: u8, b: u8) -> u8 { a.saturating_add(b) }

    #[inline]
    fn sub(a: u8, b: u8) -> u8 { a.saturating_sub(b) }
}

impl Channel for f64 {
    #[inline]
    fn min_value() -> f64 { 0.0 }

    #[inline]
    fn max_value() -> f64 { 1.0 }

    #[inline]
    fn add(a: f64, b: f64) -> f64 { a + b }

    #[inline]
    fn sub(a: f64, b: f64) -> f64 { a - b }
}


#[derive(Copy)]
pub struct ColorRGBA<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: Clone> Clone for ColorRGBA<T> {
    fn clone(&self) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
            a: self.a.clone()
        }
    }
}

fn clamp<T: PartialOrd>(value: T, min_value: T, max_value: T) -> T {
    if value < min_value {
        return min_value;
    }
    if max_value < value {
        return max_value;
    }
    value
}

// Maybe later?: ColorRGBA<f64>.quantize() -> ColorRGBA<usize>
// How do we implement this more generally so that we may have ColorRGBA<f64>
impl ColorRGBA<u8> {
    pub fn new_rgb_clamped(r: f64, g: f64, b: f64) -> ColorRGBA<u8> {
        let min_color_i32 = i32::from(<u8 as Channel>::min_value());
        let max_color_i32 = i32::from(<u8 as Channel>::max_value());
        let max_color_f64 = f64::from(<u8 as Channel>::max_value());

        ColorRGBA::new_rgb(
            clamp((r * max_color_f64).round() as i32, min_color_i32, max_color_i32) as u8,
            clamp((g * max_color_f64).round() as i32, min_color_i32, max_color_i32) as u8,
            clamp((b * max_color_f64).round() as i32, min_color_i32, max_color_i32) as u8)
    }
}

impl ColorRGBA<f64> {
    pub fn custom_clamp(&self, min_value: f64, max_value: f64) -> ColorRGBA<f64> {
        ColorRGBA {
            r: clamp(self.r, min_value, max_value),
            g: clamp(self.g, min_value, max_value),
            b: clamp(self.b, min_value, max_value),
            a: clamp(self.a, min_value, max_value),
        }
    }

    pub fn clamp(&self) -> ColorRGBA<f64> {
        self.custom_clamp(0.0, 1.0)
    }

    pub fn into_rgba8888_noalpha(&self) -> ColorRGBA<u8> {
        ColorRGBA::new_rgb_clamped(self.r, self.g, self.b)
    }
}

// Maybe later?: ColorRGBA<f64>.quantize() -> ColorRGBA<uint>
// How do we implement this more generally so that we may have ColorRGBA<f64>
impl<T: Channel> ColorRGBA<T> {
    #[allow(dead_code)]
    pub fn new_rgba(r: T, g: T, b: T, a: T) -> ColorRGBA<T> {
        ColorRGBA { r: r, g: g, b: b, a: a }
    }

    #[allow(dead_code)]
    pub fn new_rgb(r: T, g: T, b: T) -> ColorRGBA<T> {
        ColorRGBA { r: r, g: g, b: b, a: Channel::max_value() }
    }

    #[allow(dead_code)]
    pub fn black() -> ColorRGBA<T> {
        ColorRGBA::new_rgb(
            Channel::min_value(),
            Channel::min_value(),
            Channel::min_value())
    }

    #[allow(dead_code)]
    pub fn white() -> ColorRGBA<T> {
        ColorRGBA::new_rgb(
            Channel::max_value(),
            Channel::max_value(),
            Channel::max_value())
    }

    pub fn transparent() -> ColorRGBA<T> {
        ColorRGBA::new_rgba(
            Channel::min_value(),
            Channel::min_value(),
            Channel::min_value(),
            Channel::min_value())
    }

    pub fn channel_f64(&self) -> ColorRGBA<f64> {
        let max_val: T = Channel::max_value();
        ColorRGBA {
            r: self.r.to_f64().unwrap() / max_val.to_f64().unwrap(),
            g: self.g.to_f64().unwrap() / max_val.to_f64().unwrap(),
            b: self.b.to_f64().unwrap() / max_val.to_f64().unwrap(),
            a: self.a.to_f64().unwrap() / max_val.to_f64().unwrap(),
        }
    }
}

impl<T: Channel> Add for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn add(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: Channel::add(self.r, other.r),
            g: Channel::add(self.g, other.g),
            b: Channel::add(self.b, other.b),
            a: Channel::add(self.a, other.a),
        }
    }
}

impl<T: Channel> Sub for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn sub(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: Channel::sub(self.r, other.r),
            g: Channel::sub(self.g, other.g),
            b: Channel::sub(self.b, other.b),
            a: Channel::sub(self.a, other.a),
        }
    }
}

impl<T: Float> Mul for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn mul(self, other: ColorRGBA<T>) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a
        }
    }
}

// Scalar multiplication
impl<T: Float> Mul<T> for ColorRGBA<T> {
    type Output = ColorRGBA<T>;

    fn mul(self, other: T) -> ColorRGBA<T> {
        ColorRGBA {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a
        }
    }
}

///

#[derive(Copy)]
pub struct ColorRGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Clone> Clone for ColorRGB<T> {
    fn clone(&self) -> ColorRGB<T> {
        ColorRGB {
            r: self.r.clone(),
            g: self.g.clone(),
            b: self.b.clone(),
        }
    }
}

// Maybe later?: ColorRGB<f64>.quantize() -> ColorRGB<usize>
// How do we implement this more generally so that we may have ColorRGB<f64>
impl ColorRGB<u8> {
    pub fn new_rgb_clamped(r: f64, g: f64, b: f64) -> ColorRGB<u8> {
        let min_color_i32 = i32::from(<u8 as Channel>::min_value());
        let max_color_i32 = i32::from(<u8 as Channel>::max_value());
        let max_color_f64 = f64::from(<u8 as Channel>::max_value());

        ColorRGB::new_rgb(
            clamp((r * max_color_f64).round() as i32, min_color_i32, max_color_i32) as u8,
            clamp((g * max_color_f64).round() as i32, min_color_i32, max_color_i32) as u8,
            clamp((b * max_color_f64).round() as i32, min_color_i32, max_color_i32) as u8)
    }
}

// Maybe later?: ColorRGB<f64>.quantize() -> ColorRGB<uint>
// How do we implement this more generally so that we may have ColorRGB<f64>
impl<T: Channel> ColorRGB<T> {
    #[allow(dead_code)]
    pub fn new_rgb(r: T, g: T, b: T) -> ColorRGB<T> {
        ColorRGB { r: r, g: g, b: b }
    }

    #[allow(dead_code)]
    pub fn black() -> ColorRGB<T> {
        ColorRGB::new_rgb(
            Channel::min_value(),
            Channel::min_value(),
            Channel::min_value())
    }

    #[allow(dead_code)]
    pub fn white() -> ColorRGB<T> {
        ColorRGB::new_rgb(
            Channel::max_value(),
            Channel::max_value(),
            Channel::max_value())
    }

    pub fn channel_f64(&self) -> ColorRGB<f64> {
        let max_val: T = Channel::max_value();
        ColorRGB {
            r: self.r.to_f64().unwrap() / max_val.to_f64().unwrap(),
            g: self.g.to_f64().unwrap() / max_val.to_f64().unwrap(),
            b: self.b.to_f64().unwrap() / max_val.to_f64().unwrap(),
        }
    }
}

impl<T: Channel> Add for ColorRGB<T> {
    type Output = ColorRGB<T>;

    fn add(self, other: ColorRGB<T>) -> ColorRGB<T> {
        ColorRGB {
            r: Channel::add(self.r, other.r),
            g: Channel::add(self.g, other.g),
            b: Channel::add(self.b, other.b),
        }
    }
}

impl<T: Channel> Sub for ColorRGB<T> {
    type Output = ColorRGB<T>;

    fn sub(self, other: ColorRGB<T>) -> ColorRGB<T> {
        ColorRGB {
            r: Channel::sub(self.r, other.r),
            g: Channel::sub(self.g, other.g),
            b: Channel::sub(self.b, other.b),
        }
    }
}

#[test]
fn color_add() {
    let foo_color: ColorRGBA<u8> = ColorRGBA::new_rgba(1, 1, 1, 1) +
            ColorRGBA::new_rgba(2, 2, 2, 2);
    assert_eq!(foo_color.r, 3);
    assert_eq!(foo_color.g, 3);
    assert_eq!(foo_color.b, 3);
    assert_eq!(foo_color.a, 3);

    let foo_color: ColorRGBA<u8> = ColorRGBA::new_rgba(200, 1, 1, 1) +
        ColorRGBA::new_rgba(200, 2, 2, 2);
    assert_eq!(foo_color.r, 255);
    assert_eq!(foo_color.g, 3);
    assert_eq!(foo_color.b, 3);
    assert_eq!(foo_color.a, 3);
}

#[test]
fn color_sub() {
    let foo_color: ColorRGBA<u8> = ColorRGBA::new_rgba(7, 7, 7, 7) -
            ColorRGBA::new_rgba(2, 2, 2, 2);
    assert_eq!(foo_color.r, 5);
    assert_eq!(foo_color.g, 5);
    assert_eq!(foo_color.b, 5);
    assert_eq!(foo_color.a, 5);
}

#[test]
fn color_mul() {
    let foo_color = ColorRGBA::<f64>::new_rgb(0.5, 0.0, 0.0) * 2.0;

    assert_eq!(foo_color.r, 1.0);
    assert_eq!(foo_color.g, 0.0);
    assert_eq!(foo_color.b, 0.0);
    assert_eq!(foo_color.a, 1.0);
}

