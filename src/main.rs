mod color;

use std::ffi::OsString;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::process;

use anyhow::Result;
use clap::{crate_name, Parser};
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder, RgbImage};
use num::Complex;
use rayon::iter::{ParallelBridge, ParallelIterator};
use time::OffsetDateTime;

use crate::color::palettes::PolarLuvPalette;

/// Iterate a complex number `c` to determine whether it's in the Mandelbrot 
/// set. If so, return `None`. Otherwise, return an option containing the 
/// number of iterations that `c` took to escape (the "escape time").
fn iterate_point(c: Complex<f64>, num_iter: usize) -> Option<usize> {
    let mut z: Complex<f64> = Complex::new(0.0, 0.0);
    for i in 0..num_iter {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z.powf(2.0) + c
    }
    None
}

/// A bounding box in the complex plane defined by its upper left vertex, 
/// width and height
struct ComplexBoundingBox {
    upper_left: Complex<f64>,
    dims: (f64, f64),
}

impl ComplexBoundingBox {
    pub(crate) fn new(
        upper_left: Complex<f64>,
        complex_height: f64,
        aspect_ratio: f64,
    ) -> Self {
        ComplexBoundingBox {
            upper_left,
            dims: (complex_height * aspect_ratio, complex_height),
        }
    }

    pub(crate) fn map_pixel_to_point(
        &self,
        pixel: (u32, u32),
        image_dims: (u32, u32),
    ) -> Complex<f64> {
        Complex::new(
            self.upper_left.re + pixel.0 as f64 * self.dims.0 / image_dims.0 as f64,
            self.upper_left.im - pixel.1 as f64 * self.dims.1 / image_dims.1 as f64,
        )
    }
}

fn draw_fractal(
    image: &mut RgbImage,
    bounding_box: &ComplexBoundingBox,
    max_iter: usize,
    palette: &PolarLuvPalette,
    reverse: bool,
) {
    let image_dims = image.dimensions();
    image
        .enumerate_rows_mut()
        .par_bridge()
        .for_each(|(_, mut pixels)| {
            for p in &mut pixels {
                let point = bounding_box.map_pixel_to_point((p.0, p.1), image_dims);
                let result = iterate_point(point, max_iter);
                *p.2 = match result {
                    Some(i) => palette
                        .map_scalar_to_color(i as f64 / max_iter as f64, reverse)
                        .as_image_Rgb(),
                    None => image::Rgb([0; 3])
                };
            }
        });
}

fn write_image_to_disk(image: &RgbImage, out_path: &Path) -> Result<()> {
    let file = File::create(out_path)?;
    let png_writer = BufWriter::new(file);
    let encoder = PngEncoder::new(png_writer);
    encoder.write_image(image, image.width(), image.height(), ColorType::Rgb8)?;
    Ok(())
}

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[arg(short = 'W', long)]
    width: u32,

    #[arg(short = 'H', long)]
    height: u32,

    #[arg(long)]
    upper_left: Complex<f64>,

    #[arg(long)]
    cheight: f64,

    #[arg(short, long)]
    palette: OsString,

    #[arg(short, long)]
    reverse: bool,

    #[arg(short, long)]
    aspect_ratio: Option<f64>,

    #[arg(short = 'N', long)]
    max_iter: Option<usize>,

    #[arg(short, long)]
    out_file: Option<OsString>,
}

fn run(cli: &Cli) -> Result<()> {
    let (image_width, image_height) = (cli.width, cli.height);
    let upper_left = cli.upper_left;
    let complex_height = cli.cheight;
    let max_iter = cli.max_iter.unwrap_or(1000);
    let palette_path = Path::new(&cli.palette);

    let now_str;
    let out_path = match cli.out_file {
        Some(ref path) => Path::new(path),
        None => {
            now_str = format!("./{}.png", OffsetDateTime::now_utc().unix_timestamp());
            Path::new(&now_str)
        }
    };
    let reverse = cli.reverse;
    let aspect_ratio = cli.aspect_ratio.unwrap_or(image_width as f64 / image_height as f64);

    let mut image = RgbImage::new(image_width, image_height);
    let bounding_box = ComplexBoundingBox::new(upper_left, complex_height, aspect_ratio);
    let palette = PolarLuvPalette::new(palette_path)?;
    draw_fractal(&mut image, &bounding_box, max_iter, &palette, reverse);
    write_image_to_disk(&image, out_path)?;
    Ok(())
}

fn main() {
    let name = crate_name!();
    let cli = Cli::parse();
    match run(&cli) {
        Ok(()) => process::exit(0),
        Err(err) => {
            eprintln!("{}: error: {:#}", name, err);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    pub(crate) mod float;
    use crate::{iterate_point, Cli, ComplexBoundingBox};
    use num::Complex;

    #[test]
    fn iterate_point_test() {
        let num_iter = 1000;

        let result1 = iterate_point(Complex::new(0.0, 0.0), num_iter);
        assert!(result1.is_none());

        let result2 = iterate_point(Complex::new(1.0, 0.0), num_iter);
        assert_eq!(result2.unwrap(), 3);
    }

    #[test]
    fn map_pixel_to_point_test() {
        let bounding_box = ComplexBoundingBox {
            upper_left: Complex::<f64> { re: -1.0, im: 1.0 },
            dims: (2.0, 2.0),
        };
        let image_dims = (100, 100);
        assert_eq!(
            bounding_box.map_pixel_to_point((0, 0), image_dims),
            Complex::new(-1.0, 1.0),
        );
        assert_eq!(
            bounding_box.map_pixel_to_point(image_dims, image_dims),
            Complex::new(1.0, -1.0),
        );
    }

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
