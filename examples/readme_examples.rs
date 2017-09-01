extern crate image;
extern crate palette;
extern crate num;

use image::{RgbImage, GenericImage};

use palette::{LinRgba, Gradient, Mix};
use palette::pixel::Srgb;

mod color_spaces {
    use palette::{LinRgb, Lch, Hue};
    use palette::pixel::Srgb;
    use display_colors;

    pub fn run() {
        let lch_color: Lch = LinRgb::from(Srgb::new(0.8, 0.2, 0.1)).into();
        let new_color: LinRgb = lch_color.shift_hue(180.0.into()).into();

        display_colors("examples/readme_color_spaces.png", &[
            ::palette::pixel::Srgb::new(0.8, 0.2, 0.1).to_pixel(),
            ::palette::pixel::Srgb::from(new_color).to_pixel()
        ]);
    }
}

mod manipulation {
    use palette::{Color, Shade, Saturate};
    use palette::pixel::Srgb;
    use display_colors;

    pub fn run() {
        let color: Color = Srgb::new(0.8, 0.2, 0.1).into();
        let lighter = color.lighten(0.1);
        let desaturated = color.desaturate(0.5);

        display_colors("examples/readme_manipulation.png", &[
            ::palette::pixel::Srgb::from(color).to_pixel(),
            ::palette::pixel::Srgb::from(lighter).to_pixel(),
            ::palette::pixel::Srgb::from(desaturated).to_pixel()
        ]);
    }
}

mod gradients {
    use palette::{LinRgb, Hsv, Gradient};
    use display_gradients;

    pub fn run() {
        let grad1 = Gradient::new(vec![
            LinRgb::new(1.0, 0.1, 0.1),
            LinRgb::new(0.1, 1.0, 1.0)
        ]);

        let grad2 = Gradient::new(vec![
            Hsv::from(LinRgb::new(1.0, 0.1, 0.1)),
            Hsv::from(LinRgb::new(0.1, 1.0, 1.0))
        ]);

        display_gradients("examples/readme_gradients.png", grad1, grad2);
    }
}

fn display_colors(filename: &str, colors: &[[u8; 3]]) {
    let mut image = RgbImage::new(colors.len() as u32 * 64, 64);
    for (i, &color) in colors.iter().enumerate() {
        for (_, _, pixel) in image.sub_image(i as u32 * 64, 0, 64, 64).pixels_mut() {
            pixel.data = color;
        }
    }

    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("failed to write '{}': {}", filename, e),
    }
}

fn display_gradients<A: Mix<Scalar=f64> + Clone, B: Mix<Scalar=f64> + Clone>(filename: &str, grad1: Gradient<A>, grad2: Gradient<B>) where
    LinRgba<::palette::rgb::standards::Srgb, f64>: From<A>,
    LinRgba<::palette::rgb::standards::Srgb, f64>: From<B>,
{
    let mut image = RgbImage::new(256, 64);

    for (x, _, pixel) in image.sub_image(0, 0, 256, 32).pixels_mut() {
        pixel.data = Srgb::linear_to_pixel(grad1.get(x as f64 / 255.0));
    }

    for (x, _, pixel) in image.sub_image(0, 32, 256, 32).pixels_mut() {
        pixel.data = Srgb::linear_to_pixel(grad2.get(x as f64/ 255.0));
    }

    match image.save(filename) {
        Ok(()) => println!("see '{}' for the result", filename),
        Err(e) => println!("failed to write '{}': {}", filename, e),
    }
}

fn main() {
    color_spaces::run();
    manipulation::run();
    gradients::run();
}
