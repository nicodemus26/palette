//!RGB types, spaces and standards.

use num::Float;

use Yxy;
use white_point::WhitePoint;
use pixel::TransferFn;

pub use self::linear::{LinRgb, LinRgba};
pub use self::nonlinear::{Rgb, Rgba};

pub mod standards;
mod linear;
mod nonlinear;

///Nonlinear sRGB.
pub type Srgb<T = f32> = Rgb<standards::Srgb, T>;
///Nonlinear sRGB with an alpha component.
pub type Srgba<T = f32> = Rgba<standards::Srgb, T>;

///Linear sRGB.
pub type LinSrgb<T = f32> = LinRgb<standards::Srgb, T>;
///Linear sRGB with an alpha component.
pub type LinSrgba<T = f32> = LinRgba<standards::Srgb, T>;

///An RGB space and a transfer function.
pub trait RgbStandard {
    ///The RGB color space.
    type Space: RgbSpace;

    ///The transfer function for the color components.
    type TransferFn: TransferFn;
}

impl<S: RgbSpace, T: TransferFn> RgbStandard for (S, T) {
    type Space = S;
    type TransferFn = T;
}

impl<P: Primaries, W: WhitePoint, T: TransferFn> RgbStandard for (P, W, T) {
    type Space = (P, W);
    type TransferFn = T;
}

///A set of primaries and a white point.
pub trait RgbSpace {
    ///The primaries of the RGB color space.
    type Primaries: Primaries;

    ///The white point of the RGB color space.
    type WhitePoint: WhitePoint;
}

impl<P: Primaries, W: WhitePoint> RgbSpace for (P, W) {
    type Primaries = P;
    type WhitePoint = W;
}

///Represents the red, green and blue primaries of an RGB space.
pub trait Primaries {
    ///Primary red.
    fn red<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T>;
    ///Primary green.
    fn green<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T>;
    ///Primary blue.
    fn blue<Wp: WhitePoint, T: Float>() -> Yxy<Wp, T>;
}
