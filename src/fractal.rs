use std::io::{stdout, Write};

use anyhow::Ok;

use crate::{bitmap::Image, mandelbrot, rgb::Rgb};

pub struct Creator {
    width: usize,
    height: usize,
    image: Image,
    original_scale: f64,
    scale: f64,
    position_to_zoom: (isize, isize),
    color_ranges: Vec<f64>,
    colors: Vec<Rgb>,
    range_totals: Vec<usize>,
    got_first_range: bool,
    histogram: Vec<usize>,
    fractal: Vec<usize>,
}

impl Creator {
    #[must_use]
    pub fn new(
        width: usize,
        height: usize,
        original_scale: f64,
        scale: f64,
        position_to_zoom: (isize, isize),
    ) -> Self {
        Self {
            width,
            height,
            image: Image::new(width, height),
            original_scale,
            scale,
            position_to_zoom,
            color_ranges: Vec::default(),
            colors: Vec::default(),
            range_totals: Vec::default(),
            got_first_range: Default::default(),
            histogram: vec![0; mandelbrot::MAX_ITERATIONS + 1],
            fractal: vec![0; width * height],
        }
    }

    pub fn add_color_range(&mut self, range: f64, color: Rgb) {
        self.color_ranges
            .push(range + mandelbrot::MAX_ITERATIONS as f64);
        self.colors.push(color);

        if self.got_first_range {
            self.range_totals.push(0);
        }

        self.got_first_range = true;
    }

    pub fn calculate_range_totals(&mut self) {
        let mut range_index = 0;

        for i in 0..mandelbrot::MAX_ITERATIONS {
            let pixels = self.histogram[i];
            if i as f64 >= self.color_ranges[range_index + 1] {
                range_index += 1;
            }

            self.range_totals[range_index] += pixels;
        }
    }

    #[must_use]
    pub fn get_range(&self, iterations: usize) -> usize {
        let mut range = 0;

        for i in 0..self.color_ranges.len() {
            range = i;
            if (iterations as f64) < self.color_ranges[i + 1] {
                break;
            }
        }

        assert!(range < self.color_ranges.len());

        range
    }

    pub fn create_fractal(&mut self) -> anyhow::Result<()> {
        self.calculate_iterations()?;
        self.calculate_range_totals();
        self.draw_fractal();
        self.write_bitmap("image.bmp")?;
        Ok(())
    }

    pub fn calculate_iterations(&mut self) -> anyhow::Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                let cartesian = Self::do_zoom(
                    x,
                    y,
                    self.width,
                    self.height,
                    self.original_scale,
                    self.scale,
                    self.position_to_zoom,
                );
                let iterations = mandelbrot::get_iterations(cartesian.0, cartesian.1);

                self.fractal[(y * self.width) + x] = iterations;

                if iterations != mandelbrot::MAX_ITERATIONS {
                    self.histogram[iterations] += 1;
                }
            }

            if y % 80 == 0 {
                print!("-");
                stdout().flush()?;
            }
        }

        println!();
        Ok(())
    }

    pub fn draw_fractal(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let iterations = self.fractal[(y * self.width) + x];
                let (red, green, blue): (u8, u8, u8);
                let range = self.get_range(iterations);
                let range_total = self.range_totals[range];

                let start_color = self.colors[range];
                let end_color = self.colors[range + 1];
                let color_diff = end_color - start_color;

                if iterations == mandelbrot::MAX_ITERATIONS {
                    red = 0;
                    green = 0;
                    blue = 0;
                } else {
                    let mut total_pixels = 0;

                    for i in 0..iterations {
                        total_pixels += self.histogram[i];
                    }

                    red = (start_color.red + color_diff.red * total_pixels / range_total) as u8;
                    green =
                        (start_color.green + color_diff.green * total_pixels / range_total) as u8;
                    blue = (start_color.blue + color_diff.blue * total_pixels / range_total) as u8;
                }

                self.image.set_pixel(x, y, red, green, blue);
            }
        }
    }

    fn write_bitmap(&self, file_name: &str) -> anyhow::Result<()> {
        self.image.write(file_name)?;
        Ok(())
    }

    fn do_zoom(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        original_scale: f64,
        scale: f64,
        position_to_zoom: (isize, isize),
    ) -> (f64, f64) {
        let x_center = (position_to_zoom.0 as f64 - (width as f64 / 2.0)) / original_scale;
        let y_center = (position_to_zoom.1 as f64 - (height as f64 / 2.0)) / original_scale;

        if y == 0 && x == 0 {
            println!("{x_center} {y_center}");
        }

        let fractal_x = (x as f64 - (width as f64 / 2.0) - width as f64 / 4.0) / scale + x_center;
        let fractal_y = (y as f64 - (height as f64 / 2.0)) / scale + y_center;

        (fractal_x, fractal_y)
    }
}
