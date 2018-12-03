// There is lots of automatically generated code using tables of numbers
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::module_inception))]

extern crate num_traits;
#[cfg(any(feature = "approx", test))]
#[macro_use]
extern crate approx;
extern crate angular_units as angle;

#[macro_use]
mod impl_macros;

pub mod channel;
mod linalg;

pub mod color_space;
pub mod encoding;
pub mod tags;
pub mod white_point;

mod alpha;
mod chromaticity;
mod color;
mod convert;

mod ehsi;
mod hsi;
mod hsl;
mod hsv;
mod hwb;
mod lab;
mod lchab;
mod lchuv;
pub mod lms;
mod luv;
mod rgb;
mod rgi;
mod xyy;
mod xyz;
pub mod ycbcr;

#[cfg(test)]
pub mod test;

pub use color::{
    Bounded, Color, Color3, Color4, DeviceDependentColor, Flatten, FromTuple, HomogeneousColor,
    Invert, Lerp, PolarColor,
};

pub use alpha::Alpha;
pub use chromaticity::ChromaticityCoordinates;
pub use convert::{FromColor, FromHsi, FromYCbCr};
pub use ehsi::eHsi;
pub use hsi::{Hsi, HsiOutOfGamutMode};
pub use hsl::{Hsl, Hsla};
pub use hsv::{Hsv, Hsva};
pub use hwb::{Hwb, HwbBoundedChannelTraits, Hwba};
pub use lab::Lab;
pub use lchab::Lchab;
pub use lchuv::Lchuv;
pub use linalg::Matrix3;
//pub use lms::{Lms, LmsBradford, LmsCam2002, LmsCam97s, LmsModel, LmsTag, Bradford, CieCam97s, CieCam2002};
pub use luv::Luv;
pub use rgb::{Rgb, Rgba};
pub use rgi::Rgi;
pub use xyy::XyY;
pub use xyz::Xyz;
