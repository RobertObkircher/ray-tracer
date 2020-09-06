use crate::ray::*;
use crate::v3::*;
use rayon::prelude::*;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Instant;

mod ray;
mod v3;

fn percentage(name: &str, percentage: f64) {
    print!("\r{} {:6.2}%", name, 100.0 * percentage);
    std::io::stdout().flush().expect("failed to flush stdout");
}

fn ray_color(ray: &Ray) -> C3 {
    let dir = ray.direction.norm();
    let t = 0.5 * dir.y + 1.0;
    c3(1.0, 1.0, 1.0).scale(1.0 - t) + c3(0.5, 0.7, 1.0).scale(t)
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).round() as usize;

    // camera
    // y up, x right, z back (rhs coordinate system)
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = zero3();
    let horizontal = v3(viewport_width, 0.0, 0.0);
    let vertical = v3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal.div(2.0) - vertical.div(2.0) - v3(0.0, 0.0, focal_length);

    let mut pixels = vec![vec![v3(0.0, 0.0, 0.0); image_width]; image_height];
    render(&mut pixels, |row, col, pixel| {
        let u = col as f64 / (image_width - 1) as f64;
        let v = row as f64 / (image_height - 1) as f64;

        let r = Ray {
            origin,
            direction: lower_left_corner + horizontal.scale(u) + vertical.scale(v) - origin,
        };

        *pixel = ray_color(&r);
    });

    let file = File::create("image.ppm")?;
    write_ppm_file(file, &pixels)?;

    Ok(())
}

fn render<F>(pixels: &mut Vec<Vec<V3>>, f: F)
where
    F: Fn(usize, usize, &mut V3) + Sync,
{
    let start = Instant::now();
    let progress = AtomicUsize::new(0);
    percentage("rendering", 0.0);
    let height = pixels.len();
    pixels
        .par_iter_mut()
        .enumerate()
        .for_each(|(row, scanline)| {
            scanline
                .iter_mut()
                .enumerate()
                .for_each(|(col, pixel)| f(row, col, pixel));
            let old = progress.fetch_add(1, Ordering::SeqCst);
            percentage("rendering", (old + 1) as f64 / height as f64);
        });
    percentage("rendering", 1.0);
    println!("\nRendering took {:.2}s", start.elapsed().as_secs_f64());
}

fn write_ppm_file(mut file: File, pixels: &Vec<Vec<V3>>) -> std::io::Result<()> {
    let start = Instant::now();
    percentage("writing", 0.0);

    writeln!(file, "P3")?;
    writeln!(
        file,
        "{} {}",
        pixels.get(0).map(|row| row.len()).unwrap_or(0),
        pixels.len()
    )?;
    writeln!(file, "255")?;
    // The book iterates the rows backwards, because it assumes
    // the bottom left corner is (0,0). In ppm the first pixel
    // is at the top left.
    for (i, pixel_row) in pixels.iter().rev().enumerate() {
        for pixel in pixel_row {
            let ir = min(255, (256.0 * pixel.x) as u64);
            let ig = min(255, (256.0 * pixel.y) as u64);
            let ib = min(255, (256.0 * pixel.z) as u64);

            writeln!(file, "{} {} {} ", ir, ig, ib)?;
        }
        percentage("writing", (i + 1) as f64 / pixels.len() as f64);
    }

    println!("\nWriting took {:.2}s", start.elapsed().as_secs_f64());
    Ok(())
}
