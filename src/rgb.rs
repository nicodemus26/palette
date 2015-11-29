use {Color, Luma, Xyz, Lab, Mix, Shade, clamp};

///Linear RGB with an alpha component.
///
///Conversions and operations on this color space assumes that it's linear,
///meaning that gamma correction is required when converting to and from
///a displayable RGB, such as sRGB.
#[derive(Clone, Debug, PartialEq)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Rgb {
    ///Linear RGB.
    pub fn rgb(red: f32, green: f32, blue: f32) -> Rgb {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            alpha: 1.0,
        }
    }

    ///Linear RGB with transparency.
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Rgb {
        Rgb {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit values.
    pub fn rgb8(red: u8, green: u8, blue: u8) -> Rgb {
        Rgb {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
            alpha: 1.0,
        }
    }

    ///Linear RGB with transparency from 8 bit values.
    pub fn rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgb {
        Rgb {
            red: red as f32 / 255.0,
            green: green as f32 / 255.0,
            blue: blue as f32 / 255.0,
            alpha: alpha as f32 / 255.0,
        }
    }

    ///Linear RGB from sRGB.
    pub fn srgb(red: f32, green: f32, blue: f32) -> Rgb {
        Rgb {
            red: from_srgb(red),
            green: from_srgb(green),
            blue: from_srgb(blue),
            alpha: 1.0,
        }
    }

    ///Linear RGB from sRGB with transparency.
    pub fn srgba(red: f32, green: f32, blue: f32, alpha: f32) -> Rgb {
        Rgb {
            red: from_srgb(red),
            green: from_srgb(green),
            blue: from_srgb(blue),
            alpha: alpha,
        }
    }

    ///Linear RGB from 8 bit sRGB.
    pub fn srgb8(red: u8, green: u8, blue: u8) -> Rgb {
        Rgb {
            red: from_srgb(red as f32 / 255.0),
            green: from_srgb(green as f32 / 255.0),
            blue: from_srgb(blue as f32 / 255.0),
            alpha: 1.0,
        }
    }

    ///Linear RGB from 8 bit sRGB with transparency.
    pub fn srgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgb {
        Rgb {
            red: from_srgb(red as f32 / 255.0),
            green: from_srgb(green as f32 / 255.0),
            blue: from_srgb(blue as f32 / 255.0),
            alpha: alpha as f32 / 255.0,
        }
    }

    ///Convert to sRGB values and transparency.
    pub fn to_srgba(&self) -> (f32, f32, f32, f32) {
        (to_srgb(self.red), to_srgb(self.green), to_srgb(self.blue), self.alpha)
    }

    ///Convert to 8 bit sRGB values and transparency.
    pub fn to_srgba8(&self) -> (u8, u8, u8, u8) {
        (
            (clamp(to_srgb(self.red), 0.0, 1.0) * 255.0) as u8,
            (clamp(to_srgb(self.green), 0.0, 1.0) * 255.0) as u8,
            (clamp(to_srgb(self.blue), 0.0, 1.0) * 255.0) as u8,
            (clamp(self.alpha, 0.0, 1.0) * 255.0) as u8,
        )
    }

    ///Return a new RGB value with all channels clamped to `[0.0, 1.0]`.
    pub fn clamp(&self) -> Rgb {
        Rgb {
            red: clamp(self.red, 0.0, 1.0),
            green: clamp(self.green, 0.0, 1.0),
            blue: clamp(self.blue, 0.0, 1.0),
            alpha: clamp(self.alpha, 0.0, 1.0),
        }
    }

    ///Clamp all channels to `[0.0, 1.0]`.
    pub fn clamp_self(&mut self) {
        self.red = clamp(self.red, 0.0, 1.0);
        self.green = clamp(self.green, 0.0, 1.0);
        self.blue = clamp(self.blue, 0.0, 1.0);
        self.alpha = clamp(self.alpha, 0.0, 1.0);
    }
}

impl Mix for Rgb {
    fn mix(&self, other: &Rgb, factor: f32) -> Rgb {
        let factor = clamp(factor, 0.0, 1.0);

        Rgb {
            red: self.red + factor * (other.red - self.red),
            green: self.green + factor * (other.green - self.green),
            blue: self.blue + factor * (other.blue - self.blue),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl Shade for Rgb {
    fn lighten(&self, amount: f32) -> Rgb {
        Rgb {
            red: (self.red + amount).max(0.0),
            green: (self.green + amount).max(0.0),
            blue: (self.blue + amount).max(0.0),
            alpha: self.alpha,
        }
    }
}

impl Default for Rgb {
    fn default() -> Rgb {
        Rgb::rgb(0.0, 0.0, 0.0)
    }
}

from_color!(to Rgb from Xyz, Luma, Lab);

impl From<Luma> for Rgb {
    fn from(luma: Luma) -> Rgb {
        Rgb {
            red: luma.luma,
            green: luma.luma,
            blue: luma.luma,
            alpha: luma.alpha,
        }
    }
}

impl From<Xyz> for Rgb {
    fn from(xyz: Xyz) -> Rgb {
        Rgb {
            red: xyz.x * 3.2406 + xyz.y * -1.5372 + xyz.z * -0.4986,
            green: xyz.x * -0.9689 + xyz.y * 1.8758 + xyz.z * 0.0415,
            blue: xyz.x * 0.0557 + xyz.y * -0.2040 + xyz.z * 1.0570,
            alpha: xyz.alpha,
        }
    }
}

impl From<Lab> for Rgb {
    fn from(lab: Lab) -> Rgb {
        Xyz::from(lab).into()
    }
}

fn from_srgb(x: f32) -> f32 {
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}

fn to_srgb(x: f32) -> f32 {
    if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}