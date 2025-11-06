use image::{ ImageBuffer, Rgb };
use std::time::Instant;
use num_complex::Complex;
use hsv_to_rgb::hsv_to_rgb;

fn main() {
    let image_width:u32 = 1920;
    let image_height:u32 = 1080;
    let max_iterations:u32 = 1000;

    let mut imgbuf = ImageBuffer::new(image_width, image_height);

    let x_min:f64 = -2.0;
    let x_max:f64 = 1.0;
    let y_min:f64 = -1.0;
    let y_max:f64 = 1.0;

    let start = Instant::now();
    // Precompute scale factors to avoid repeated division
    let dx = (x_max - x_min) / (image_width as f64);
    let dy = (y_max - y_min) / (image_height as f64);

    for y in 0..image_height {
        let cy = y_min + (y as f64) * dy;
        for x in 0..image_width {
            // Map pixel to complex plane (optimized by using precomputed dx/dy)
            let cx = x_min + (x as f64) * dx;

            // Mandelbrot iteration: z_{n+1} = z_n^2 + c
            let c = Complex::new(cx, cy);
            let mut z = Complex::new(0.0, 0.0);
            let mut iterations = 0u32;
            while z.norm_sqr() <= 4.0 && iterations < max_iterations {
                z = z * z + c;
                iterations += 1;
            }

            // Color mapping
            let pixel: Rgb<u8> = if iterations == max_iterations {
                // Angle-based hue for points that did not escape (gives pleasing bands)
                let angle = z.im.atan2(z.re) as f32;
                let hue_norm = (angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
                let hue = hue_norm * 360.0;
                hsv_to_rgb(hue, 1.0, 1.0)
            } else {
                // Hue based on escape iteration count
                let hue = (iterations as f32 / max_iterations as f32) * 360.0;
                hsv_to_rgb(hue, 1.0, 1.0)
            };

            imgbuf.put_pixel(x, y, pixel);
        }
    }

    let duration = start.elapsed();
    println!("Rendering time: {:?}", duration);

    std::fs::create_dir_all("./out").unwrap();
    imgbuf.save("./out/mandelbrot_single.png").unwrap();
    println!("Image saved to ./out/mandelbrot_single.png");
}