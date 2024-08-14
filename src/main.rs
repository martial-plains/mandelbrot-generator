use anyhow::Ok;
use fractal::Creator;
use rgb::Rgb;

mod bitmap;
mod fractal;
mod mandelbrot;
mod rgb;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const ORIGINAL_SCALE: f64 = HEIGHT as f64 / 2.0;
const SCALE: f64 = ORIGINAL_SCALE;

const POSITION_TO_ZOOM: (isize, isize) = (WIDTH as isize / 2, HEIGHT as isize / 2);

fn main() -> anyhow::Result<()> {
    println!("Processing image >_");
    let mut fractal_creator = Creator::new(WIDTH, HEIGHT, ORIGINAL_SCALE, SCALE, POSITION_TO_ZOOM);

    fractal_creator.add_color_range(0.0, Rgb::new(0, 0, 0));
    fractal_creator.add_color_range(0.3, Rgb::new(255, 0, 0));
    fractal_creator.add_color_range(0.5, Rgb::new(255, 255, 0));
    fractal_creator.add_color_range(1.0, Rgb::new(255, 255, 255));

    println!("{}", fractal_creator.get_range(999));
    fractal_creator.create_fractal()?;

    println!("Finnished!");

    Ok(())
}
